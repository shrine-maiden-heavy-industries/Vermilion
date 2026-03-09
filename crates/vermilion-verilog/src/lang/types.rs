// SPDX-License-Identifier: BSD-3-Clause

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NetType {
	Supply0,
	Supply1,
	Tri,
	Tri0,
	Tri1,
	TriAnd,
	TriOr,
	Wire,
	WireAnd,
	WireOr,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LowStrength {
	Supply,
	Strong,
	Pull,
	Weak,
	HighZ,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum HighStrength {
	Supply,
	Strong,
	Pull,
	Weak,
	HighZ,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DriveStrength {
	HighLow(HighStrength, LowStrength),
	LowHigh(LowStrength, HighStrength),
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ChargeStrength {
	Small,
	#[default] // IEEE 1364-1995 § 3.4.1
	Medium,
	Large,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EdgeDescriptor {
	PosEdge,
	NegEdge,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EdgeIdentifier {
	PosEdge,
	NegEdge,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimeUnit {
	Seconds,
	Milliseconds,
	Microseconds,
	Nanoseconds,
	Picoseconds,
	Femtoseconds,
}
