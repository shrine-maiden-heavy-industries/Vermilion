// SPDX-License-Identifier: BSD-3-Clause

#[macro_export]
macro_rules! test_xml_deserialize {
	(spirit_1_0, $test_name:ident, $xml:literal, $obj:expr) => {
		paste::paste! {
			$crate::test_xml_deserialize!([<spirit_1_0_ $test_name>], $xml, $obj);
		}
	};
	(spirit_1_1, $test_name:ident, $xml:literal, $obj:expr) => {
		paste::paste! {
			$crate::test_xml_deserialize!([<spirit_1_1_ $test_name>], $xml, $obj);
		}
	};
	(spirit_1_2, $test_name:ident, $xml:literal, $obj:expr) => {
		paste::paste! {
			$crate::test_xml_deserialize!([<spirit_1_2_ $test_name>], $xml, $obj);
		}
	};
	(spirit_1_4, $test_name:ident, $xml:literal, $obj:expr) => {
		paste::paste! {
			$crate::test_xml_deserialize!([<spirit_1_4_ $test_name>], $xml, $obj);
		}
	};
	(spirit_1_5, $test_name:ident, $xml:literal, $obj:expr) => {
		paste::paste! {
			$crate::test_xml_deserialize!([<spirit_1_5_ $test_name>], $xml, $obj);
		}
	};
	(ipxact_2009, $test_name:ident, $xml:literal, $obj:expr) => {
		paste::paste! {
			$crate::test_xml_deserialize!([<ipxact_2009_ $test_name>], $xml, $obj);
		}
	};
	(ipxact_2014, $test_name:ident, $xml:literal, $obj:expr) => {
		paste::paste! {
			$crate::test_xml_deserialize!([<ipxact_2014_ $test_name>], $xml, $obj);
		}
	};
	(ipxact_2022, $test_name:ident, $xml:literal, $obj:expr) => {
		paste::paste! {
			$crate::test_xml_deserialize!([<ipxact_2022_ $test_name>], $xml, $obj);
		}
	};
	($test_name:ident, $xml:literal, $obj:expr) => {
		paste::paste! {
			#[test]
			fn [<test_xml_deserialize_ $test_name>] () {
				let gold = $obj;

				#[inline(always)]
				fn _assert<'de, 'a, T>(xml: &'de str, gold: &'a T)
				where
					T: PartialEq + serde::Deserialize<'de> + std::fmt::Debug
				{
					let check: T = quick_xml::de::from_str(xml).unwrap();
					assert_eq!(&check, gold);
				}

				_assert($xml, &gold);
			}
		}
	};
}

#[macro_export]
macro_rules! test_xml_serialize {
	(spirit_1_0, $test_name:ident, $obj:expr, $xml:literal) => {
		paste::paste! {
			$crate::test_xml_serialize!([<spirit_1_0_ $test_name>], $obj, $xml);
		}
	};
	(spirit_1_1, $test_name:ident, $obj:expr, $xml:literal) => {
		paste::paste! {
			$crate::test_xml_serialize!([<spirit_1_1_ $test_name>], $obj, $xml);
		}
	};
	(spirit_1_2, $test_name:ident, $obj:expr, $xml:literal) => {
		paste::paste! {
			$crate::test_xml_serialize!([<spirit_1_2_ $test_name>], $obj, $xml);
		}
	};
	(spirit_1_4, $test_name:ident, $obj:expr, $xml:literal) => {
		paste::paste! {
			$crate::test_xml_serialize!([<spirit_1_4_ $test_name>], $obj, $xml);
		}
	};
	(spirit_1_5, $test_name:ident, $obj:expr, $xml:literal) => {
		paste::paste! {
			$crate::test_xml_serialize!([<spirit_1_5_ $test_name>], $obj, $xml);
		}
	};
	(ipxact_2009, $test_name:ident, $obj:expr, $xml:literal) => {
		paste::paste! {
			$crate::test_xml_serialize!([<ipxact_2009_ $test_name>], $obj, $xml);
		}
	};
	(ipxact_2014, $test_name:ident, $obj:expr, $xml:literal) => {
		paste::paste! {
			$crate::test_xml_serialize!([<ipxact_2014_ $test_name>], $obj, $xml);
		}
	};
	(ipxact_2022, $test_name:ident, $obj:expr, $xml:literal) => {
		paste::paste! {
			$crate::test_xml_serialize!([<ipxact_2022_ $test_name>], $obj, $xml);
		}
	};
	($test_name:ident, $obj:expr, $xml:literal) => {
		paste::paste! {
			#[test]
			fn [<test_xml_serialize_ $test_name>] () {
				let gold = $obj;
				assert_eq!(quick_xml::se::to_string(&gold).unwrap(), $xml);
			}
		}
	};
}

#[macro_export]
macro_rules! test_xml_serdes {
	(spirit_1_0, $test_name:ident, $xml:literal, $obj:expr) => {
		$crate::test_xml_deserialize!(spirit_1_0, $test_name, $xml, $obj);
		$crate::test_xml_serialize!(spirit_1_0, $test_name, $obj, $xml);
	};
	(spirit_1_1, $test_name:ident, $xml:literal, $obj:expr) => {
		$crate::test_xml_deserialize!(spirit_1_1, $test_name, $xml, $obj);
		$crate::test_xml_serialize!(spirit_1_1, $test_name, $obj, $xml);
	};
	(spirit_1_2, $test_name:ident, $xml:literal, $obj:expr) => {
		$crate::test_xml_deserialize!(spirit_1_2, $test_name, $xml, $obj);
		$crate::test_xml_serialize!(spirit_1_2, $test_name, $obj, $xml);
	};
	(spirit_1_4, $test_name:ident, $xml:literal, $obj:expr) => {
		$crate::test_xml_deserialize!(spirit_1_4, $test_name, $xml, $obj);
		$crate::test_xml_serialize!(spirit_1_4, $test_name, $obj, $xml);
	};
	(spirit_1_5, $test_name:ident, $xml:literal, $obj:expr) => {
		$crate::test_xml_deserialize!(spirit_1_5, $test_name, $xml, $obj);
		$crate::test_xml_serialize!(spirit_1_5, $test_name, $obj, $xml);
	};
	(ipxact_2009, $test_name:ident, $xml:literal, $obj:expr) => {
		$crate::test_xml_deserialize!(ipxact_2009, $test_name, $xml, $obj);
		$crate::test_xml_serialize!(ipxact_2009, $test_name, $obj, $xml);
	};
	(ipxact_2014, $test_name:ident, $xml:literal, $obj:expr) => {
		$crate::test_xml_deserialize!(ipxact_2014, $test_name, $xml, $obj);
		$crate::test_xml_serialize!(ipxact_2014, $test_name, $obj, $xml);
	};
	(ipxact_2022, $test_name:ident, $xml:literal, $obj:expr) => {
		$crate::test_xml_deserialize!(ipxact_2022, $test_name, $xml, $obj);
		$crate::test_xml_serialize!(ipxact_2022, $test_name, $obj, $xml);
	};
	($test_name:ident, $xml:literal, $obj:expr) => {
		$crate::test_xml_deserialize!($test_name, $xml, $obj);
		$crate::test_xml_serialize!($test_name, $obj, $xml);
	};
}
