#[macro_export]
macro_rules! fix_string {
    ($NAME:ident, $TAG:literal) => {
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Default, Debug, Clone, Copy)]
        pub struct $NAME<S: fix_model_core::prelude::StringValue>(S);
        impl<S: fix_model_core::prelude::StringValue> $NAME<S> {
            pub const TAG: fix_model_core::prelude::Tag = fix_model_core::prelude::Tag::new($TAG);
            pub const NAME: &'static str = stringify!($NAME);
            #[inline(always)]
            pub fn new(value: S) -> Self {
                Self(value)
            }
            pub fn to_owned(&self) -> $NAME<String> {
                $NAME(self.0.to_string())
            }
        }
        impl<S: fix_model_core::prelude::StringValue> fix_model_core::prelude::Field for $NAME<S> {
            #[inline(always)]
            fn tag(&self) -> fix_model_core::prelude::Tag {
                $NAME::<S>::TAG
            }
            #[inline(always)]
            fn value(&self) -> &impl fix_model_core::prelude::Value {
                &self.0
            }
        }
        impl<S: fix_model_core::prelude::StringValue> fix_model_core::prelude::Serialize for $NAME<S> {
            #[inline(always)]
            fn serialize(&self, ser: &mut impl fix_model_core::prelude::Serializer) {
                use fix_model_core::prelude::Field;
                self.tag().serialize(ser);
                self.value().serialize(ser);
            }
        }
        impl<S: fix_model_core::prelude::StringValue> std::fmt::Display for $NAME<S> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if f.sign_plus() {
                    write!(f, "{}={}", $NAME::<S>::NAME, self.0.as_ref())
                } else if f.sign_minus() {
                    write!(f, "{}={}", $NAME::<S>::TAG, self.0.as_ref())
                } else {
                    write!(f, "{}", self.0)
                }
            }
        }
        impl<S: fix_model_core::prelude::StringValue> From<S> for $NAME<S> {
            #[inline(always)]
            fn from(value: S) -> Self {
                Self::new(value)
            }
        }
    };
}

#[macro_export]
macro_rules! fix_int {
    ($NAME:ident, $ID:literal) => {
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone, Default)]
        pub struct $NAME(i32);
        impl $NAME {
            pub const TAG: u32 = $ID;
            pub const NAME: &'static str = stringify!($NAME);
            #[inline(always)]
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
    ($NAME:ident, $TAG:literal) => {
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone, Default)]
        pub struct $NAME(char);
        impl $NAME {
            pub const TAG: fix_model_core::prelude::Tag = fix_model_core::prelude::Tag::new($TAG);
            pub const NAME: &'static str = stringify!($NAME);
            #[inline(always)]
            pub fn new(value: char) -> Self {
                Self(value)
            }
        }
        impl fix_model_core::prelude::Serialize for $NAME {
            #[inline(always)]
            fn serialize(&self, ser: &mut impl fix_model_core::prelude::Serializer) {
                use fix_model_core::prelude::Field;
                self.tag().serialize(ser);
                self.value().serialize(ser);
            }
        }
        impl fix_model_core::prelude::Field for $NAME {
            #[inline(always)]
            fn tag(&self) -> fix_model_core::prelude::Tag {
                Self::TAG
            }
            #[inline(always)]
            fn value(&self) -> &impl fix_model_core::prelude::Value {
                &self.0
            }
        }
        impl From<char> for $NAME {
            #[inline(always)]
            fn from(value: char) -> Self {
                Self::new(value)
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
            pub const TAG: u32 = $ID;
            pub const NAME: &'static str = stringify!($NAME);
            #[inline(always)]
            pub fn new(value: String) -> Self {
                debug_assert!(value.len() == 2, "Country code must be 2 characters");
                Self(value)
            }
        }

        $crate::_debug!($NAME);
        $crate::_display!($NAME);
    };
}
#[macro_export]
macro_rules! fix_bool {
    ($NAME:ident, $ID:literal) => {
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone, Default)]
        pub struct $NAME(char);
        impl $NAME {
            pub const TAG: u32 = $ID;
            pub const NAME: &'static str = stringify!($NAME);
            #[inline(always)]
            pub fn new(value: bool) -> Self {
                match value {
                    true => Self('Y'),
                    false => Self('N'),
                }
            }
        }

        $crate::_debug!($NAME);
        $crate::_display!($NAME);
    };
}

#[macro_export]
macro_rules! fix_seq_num {
    ($NAME:ident, $ID:literal) => {
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone, Default)]
        pub struct $NAME(u64);
        impl $NAME {
            pub const TAG: u32 = $ID;
            pub const NAME: &'static str = stringify!($NAME);
            #[inline(always)]
            pub fn new(value: u64) -> Self {
                Self(value)
            }
        }

        $crate::_debug!($NAME);
        $crate::_display!($NAME);
    };
}
#[macro_export]
macro_rules! fix_length {
    ($NAME:ident, $ID:literal) => {
        $crate::fix_seq_num!($NAME, $ID);
    };
}

#[macro_export]
macro_rules! fix_number_in_group {
    ($NAME:ident, $ID:literal) => {
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone, Default)]
        pub struct $NAME(u16);
        impl $NAME {
            pub const TAG: u32 = $ID;
            pub const NAME: &'static str = stringify!($NAME);
            #[inline(always)]
            pub fn new(value: u16) -> Self {
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
                f.debug_tuple(std::any::type_name::<Self>().split("::").last().ok_or(std::fmt::Error)?)
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
                    write!(f, "{}={}", $NAME::TAG, self.0)
                } else {
                    write!(f, "{}", self.0)
                }
            }
        }
    };
}
