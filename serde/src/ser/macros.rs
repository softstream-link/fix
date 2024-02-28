macro_rules! serialize_unimplemented {
    ($struct:ty, $($fn:ident(self, $value:ident: $ty:ty ),)*) => {
        $(
            #[cold]
            fn $fn(self, $value: $ty) -> $crate::error::Result<()> {
                Err(serde::ser::Error::custom(format!("{}::{} is Not Implmented", stringify!($struct), stringify!($fn))))
            }
        )*
    };
    ($struct:ty, $($fn:ident(self),)*) => {
        $(
            #[cold]
            fn $fn(self) -> $crate::error::Result<()> {
                Err(serde::ser::Error::custom(format!("{}::{} is Not Implmented", stringify!($struct), stringify!($fn))))
            }
        )*
    };

}

pub(crate) use serialize_unimplemented;

macro_rules! impl_serialize_integer {
    ( $($fn:ident(self, $value:ident: $ty:ty),)* ) => {
        $(
        #[inline(always)]
        fn $fn(self, $value: $ty) -> $crate::error::Result<()> {
            self.write.write_value(itoa::Buffer::new().format($value).as_bytes())
        }
    )*
    };
}
pub(crate) use impl_serialize_integer;
