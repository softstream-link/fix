macro_rules! impl_deserialize_unimplemented {
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

pub(crate) use impl_deserialize_unimplemented;

macro_rules! impl_deserialize_unsigned {
    ( $($NAME:ident),* ) => {
        $(
            #[inline]
            fn $NAME<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
                let int = self.read.parse_value_as_number::<u64>()?;
                visitor.visit_u64(int)
            }
        )*
    };
}
pub(crate) use impl_deserialize_unsigned;

macro_rules! impl_deserialize_signed {
    ( $($NAME:ident),* ) => {
        $(
            #[inline]
            fn $NAME<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
                let int = self.read.parse_value_as_number::<i64>()?;
                visitor.visit_i64(int)
            }
        )*
    };
}
pub(crate) use impl_deserialize_signed;

macro_rules! impl_deserialize_float {
    ( $($NAME:ident),* ) => {
        $(
            #[inline]
            fn $NAME<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
                let float = self.read.parse_value_as_number::<f64>()?;
                visitor.visit_f64(float)
            }
        )*
    };
}
pub(crate) use impl_deserialize_float;

// macro_rules! impl_deserialize_delegate {
//     // deserialie_u8, etc
//     ($struct:expr, $($fn:ident(self, $visitor:ident: V),)*) => {
//         $(
//             fn $fn<V: de::Visitor<'de>>(self, $visitor: V) -> Result<V::Value> {
//                 self.deserializer.$fn($visitor)
//             }
//         )*
//     };
//     // // deserialize_unit_struct & deserialize_newtype_struct
//     // ($struct:expr, $($fn:ident(self, $name:ident: &'static str, $visitor:ident: V)),*) => {
//     //     $(
//     //         #[cold]
//     //         fn $fn<V: de::Visitor<'de>>(self, $name: &'static str, $visitor: V) -> Result<V::Value> {
//     //             use serde::de::Error as DeError; // needs to get trait in scope to get access to custom
//     //             Err(Error::custom(format!("{}::{} is Not Implmented", $struct, stringify!($fn))))
//     //         }
//     //     )*
//     // };
//     // // deserialize_tuple
//     // ($struct:expr, $($fn:ident(self, $len:ident : usize, $visitor:ident: V)),*) => {
//     //     $(
//     //         #[cold]
//     //         fn $fn<V: de::Visitor<'de>>(self, $len: usize, $visitor: V) -> Result<V::Value> {
//     //             use serde::de::Error as DeError; // needs to get trait in scope to get access to custom
//     //             Err(Error::custom(format!("{}::{} is Not Implmented", $struct, stringify!($fn))))
//     //         }
//     //     )*
//     // };
//     // // deserialize_tuple_struct
//     // ($struct:expr, $($fn:ident(self, $name:ident: &'static str, $len:ident: usize,  $visitor:ident: V)),*) => {
//     //     $(
//     //         #[cold]
//     //         fn $fn<V: de::Visitor<'de>>(self, $name: &'static str, $len: usize, $visitor: V) -> Result<V::Value> {
//     //             use serde::de::Error as DeError; // needs to get trait in scope to get access to custom
//     //             Err(Error::custom(format!("{}::{} is Not Implmented", $struct, stringify!($fn))))
//     //         }
//     //     )*
//     // };
//     // deserialize_struct & deserialize_enum
//     ($struct:expr, $($fn:ident(self, $name:ident : &'static str, $flds_vars:ident : &'static [&'static str], $visitor:ident : V)),*) => {
//         $(
//             fn $fn<V: de::Visitor<'de>>(self, $name: &'static str, $flds_vars: &'static [&'static str], $visitor: V) -> Result<V::Value> {
//                 self.deserializer.$fn($name, $flds_vars, $visitor)
//             }
//         )*
//     };

// }

// pub(crate) use impl_deserialize_delegate;
