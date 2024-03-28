macro_rules! deserialize_unimplemented {
    // deserialie_u8, etc
    ($struct:expr, $($fn:ident(self, $visitor:ident: V),)*) => {
        $(
            #[cold]
            fn $fn<V: de::Visitor<'de>>(self, $visitor: V) -> Result<V::Value> {
                use serde::de::Error as DeError; // needs to get trait in scope to get access to custom
                Err(Error::custom(format!("{}::{} is Not Implmented", $struct, stringify!($fn))))
            }
        )*
    };
    // deserialize_unit_struct & deserialize_newtype_struct
    ($struct:expr, $($fn:ident(self, $name:ident: &'static str, $visitor:ident: V)),*) => {
        $(
            #[cold]
            fn $fn<V: de::Visitor<'de>>(self, $name: &'static str, $visitor: V) -> Result<V::Value> {
                use serde::de::Error as DeError; // needs to get trait in scope to get access to custom
                Err(Error::custom(format!("{}::{} is Not Implmented", $struct, stringify!($fn))))
            }
        )*
    };
    // deserialize_tuple
    ($struct:expr, $($fn:ident(self, $len:ident : usize, $visitor:ident: V)),*) => {
        $(
            #[cold]
            fn $fn<V: de::Visitor<'de>>(self, $len: usize, $visitor: V) -> Result<V::Value> {
                use serde::de::Error as DeError; // needs to get trait in scope to get access to custom
                Err(Error::custom(format!("{}::{} is Not Implmented", $struct, stringify!($fn))))
            }
        )*
    };
    // deserialize_tuple_struct
    ($struct:expr, $($fn:ident(self, $name:ident: &'static str, $len:ident: usize,  $visitor:ident: V)),*) => {
        $(
            #[cold]
            fn $fn<V: de::Visitor<'de>>(self, $name: &'static str, $len: usize, $visitor: V) -> Result<V::Value> {
                use serde::de::Error as DeError; // needs to get trait in scope to get access to custom
                Err(Error::custom(format!("{}::{} is Not Implmented", $struct, stringify!($fn))))
            }
        )*
    };
    // deserialize_struct & deserialize_enum
    ($struct:expr, $($fn:ident(self, $name:ident : &'static str, $flds_vars:ident : &'static [&'static str], $visitor:ident : V)),*) => {
        $(
            #[cold]
            fn $fn<V: de::Visitor<'de>>(self, $name: &'static str, $flds_vars: &'static [&'static str], $visitor: V) -> Result<V::Value> {
                use serde::de::Error as DeError; // needs to get trait in scope to get access to custom
                Err(Error::custom(format!("{}::{} is Not Implmented", $struct, stringify!($fn))))
            }
        )*
    };

}

pub(crate) use deserialize_unimplemented;
