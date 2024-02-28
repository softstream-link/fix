#[macro_export]
macro_rules! fix_string {
    ($NAME:ident, $ID:literal) => {
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone, Default)]
        pub struct $NAME(String);
        impl $NAME {
            pub const ID: u32 = $ID;
            pub const NAME: &'static str = stringify!($NAME);
            pub fn new(value: String) -> Self {
                Self(value)
            }
        }
        $crate::_debug!($NAME);
        $crate::_display!($NAME);
    };
}

#[macro_export]
macro_rules! fix_int {
    ($NAME:ident, $ID:literal) => {
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone, Default)]
        pub struct $NAME(i32);
        impl $NAME {
            pub const ID: u32 = $ID;
            pub const NAME: &'static str = stringify!($NAME);
            pub fn new(value: i32) -> Self {
                Self(value)
            }
        }

        $crate::_debug!($NAME);
        $crate::_display!($NAME);
    };
}
#[macro_export]
macro_rules! fix_char {
    ($NAME:ident, $ID:literal) => {
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone, Default)]
        pub struct $NAME(char);
        impl $NAME {
            pub const ID: u32 = $ID;
            pub const NAME: &'static str = stringify!($NAME);
            pub fn new(value: char) -> Self {
                Self(value)
            }
        }

        $crate::_debug!($NAME);
        $crate::_display!($NAME);
    };
}
#[macro_export]
macro_rules! fix_country {
    ($NAME:ident, $ID:literal) => {
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone, Default)]
        pub struct $NAME(String);
        impl $NAME {
            pub const ID: u32 = $ID;
            pub const NAME: &'static str = stringify!($NAME);
            pub fn new(value: String) -> Self {
                debug_assert!(value.len() == 3, "Country code must be 3 characters");
                Self(value)
            }
        }

        $crate::_debug!($NAME);
        $crate::_display!($NAME);
    };
}

#[macro_export]
macro_rules! _debug {
    ($NAME:ident) => {
        impl std::fmt::Debug for $NAME {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(
                    std::any::type_name::<Self>()
                        .split("::")
                        .last()
                        .ok_or(std::fmt::Error)?,
                )
                .field(&self.0)
                .finish()
            }
        }
    };
}
#[macro_export]
macro_rules! _display {
    ($NAME:ident) => {
        impl std::fmt::Display for $NAME {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if f.sign_plus() {
                    write!(f, "{}={}", $NAME::NAME, self.0)
                } else if f.sign_minus() {
                    write!(f, "{}", self.0)
                } else {
                    write!(f, "{}={}", $NAME::ID, self.0)
                }
            }
        }
    };
}
