// SPDX-License-Identifier: BSD-3-Clause

use std::sync::OnceLock;

use color_eyre::{
	config::HookBuilder,
	eyre::{EyreHandler, InstallError},
};
use color_print::cformat;

type EyreHookFunc =
	Box<dyn Fn(&(dyn std::error::Error + 'static)) -> Box<dyn EyreHandler> + Send + Sync + 'static>;
type PanicHookFunc = Box<dyn Fn(&std::panic::PanicHookInfo<'_>) + Send + Sync + 'static>;

static HOOK_PROLOGUE: OnceLock<String> = OnceLock::new();
static HOOK_EPILOGUE: OnceLock<String> = OnceLock::new();

struct VermilionEyreHook {
	inner: EyreHookFunc,
}

struct VermilionPanicHook {
	inner: PanicHookFunc,
}

struct VermilionEyreHander {
	inner: Box<dyn EyreHandler>,
}

impl VermilionEyreHook {
	fn build_handler(&self, e: &(dyn std::error::Error + 'static)) -> VermilionEyreHander {
		VermilionEyreHander { inner: (*self.inner)(e) }
	}

	pub fn install(self) -> Result<(), InstallError> {
		color_eyre::eyre::set_hook(self.into_eyre_hook())
	}

	pub fn into_eyre_hook(self) -> EyreHookFunc {
		Box::new(move |e| Box::new(self.build_handler(e)))
	}
}

impl VermilionPanicHook {
	pub fn install(self) {
		std::panic::set_hook(self.into_panic_hook());
	}

	pub fn into_panic_hook(self) -> PanicHookFunc {
		Box::new(move |info| {
			// SAFETY:
			// `HOOK_PROLOGUE` and `HOOK_EPILOGUE` *should* always be initialized by the time we get
			// here and if not, we're boned either way...
			#[allow(clippy::unwrap_used)]
			let prologue = HOOK_PROLOGUE.get().unwrap();
			#[allow(clippy::unwrap_used)]
			let epilogue = HOOK_EPILOGUE.get().unwrap();

			eprintln!("{}", prologue);
			(*self.inner)(info);
			eprintln!("{}", epilogue);
		})
	}
}

impl EyreHandler for VermilionEyreHander {
	fn debug(
		&self,
		error: &(dyn std::error::Error + 'static),
		f: &mut core::fmt::Formatter<'_>,
	) -> core::fmt::Result {
		// SAFETY:
		// `HOOK_PROLOGUE` and `HOOK_EPILOGUE` *should* always be initialized by the time we get
		// here and if not, we're boned either way...
		#[allow(clippy::unwrap_used)]
		writeln!(f, "{}", HOOK_PROLOGUE.get().unwrap())?;
		self.inner.debug(error, f)?;
		#[allow(clippy::unwrap_used)]
		writeln!(f, "\n{}", HOOK_EPILOGUE.get().unwrap())?;

		Ok(())
	}

	fn track_caller(&mut self, location: &'static std::panic::Location<'static>) {
		self.inner.track_caller(location);
	}
}

pub(crate) fn install() -> eyre::Result<()> {
	let default_hooks = HookBuilder::default().display_env_section(false);
	let (panic_hook, eyre_hook) = default_hooks.try_into_hooks()?;

	HOOK_PROLOGUE.get_or_init(|| {
		cformat!(
			"\
			\n\n<blue>{}</> v{} has encountered an unhandled error\n\n<bright-black>------------[ ✂ cut \
			 here ✂ ]------------</>\n",
			env!("CARGO_PKG_NAME"),
			env!("CARGO_PKG_VERSION")
		)
	});
	HOOK_EPILOGUE.get_or_init(|| {
		cformat!(
			"\
			\n<bright-black>------------[ ✂ cut here ✂ ]------------</>\n\nPlease report this issue at: \
			 <red>{}/issues</>",
			env!("CARGO_PKG_REPOSITORY")
		)
	});

	VermilionPanicHook { inner: panic_hook.into_panic_hook() }.install();
	VermilionEyreHook { inner: eyre_hook.into_eyre_hook() }.install()?;

	Ok(())
}
