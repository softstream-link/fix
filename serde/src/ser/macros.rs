macro_rules! serialize_unimplemented {
    ($struct:ty, $($fn:ident(self, $value:ident: $ty:ty ),)*) => {
        $(
            #[cold]
            fn $fn(self, $value: $ty) -> Result<()> {
                use serde::ser::Error as SerError; // needs to get trait in scope to get access to custom
                Err(Error::custom(format!("{}::{} is Not Implmented", stringify!($struct), stringify!($fn))))
            }
        )*
    };

}

pub(crate) use serialize_unimplemented;
