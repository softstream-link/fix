#[macro_export]
macro_rules! fix_string {
    ($NAME:ident, $TAG:literal) => {
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone, Copy)]
        pub struct $NAME<S>(S);
        impl<S> $NAME<S> {
            #[inline]
            pub fn new(value: S) -> Self {
                Self(value)
            }
        }
        impl<S: AsRef<str>> AsRef<str> for $NAME<S> {
            fn as_ref(&self) -> &str {
                self.0.as_ref()
            }
        }
        impl $NAME<&str> {
            /// Ownwing inner Type<&str> -> Type<String> instead of &Type<&str> -> Type<&str>
            #[inline]
            pub fn to_owned(&self) -> $NAME<String> {
                $NAME(self.0.to_owned())
            }
            /// Borrowing inner Type<&str> -> Type<&str> instead of Type<&str> -> &Type<&str>
            #[inline]
            pub fn borrow(&self) -> $NAME<&str> {
                $NAME(self.0)
            }
            /// Borrowing inner &str as &str
            #[inline]
            pub fn value(&self) -> &str {
                self.0
            }
        }
        impl<'a> Default for $NAME<&'a str> {
            #[inline]
            fn default() -> Self {
                Self(concat!(stringify!($NAME), ":", stringify!($TAG), "@Default"))
            }
        }
        impl $NAME<String> {
            /// Borrowing inner Type<String> -> Type<&str> instead of Type<String> -> &Type<String>
            #[inline]
            pub fn borrow(&self) -> $NAME<&str> {
                $NAME(self.0.as_str())
            }
            /// Ownwing inner Type<String> -> Type<String> instead of &Type<String> -> Type<String>
            #[inline]
            pub fn to_owned(&self) -> $NAME<String> {
                $NAME(self.0.clone())
            }
            /// Borrowing inner String as &str
            #[inline]
            pub fn value(&self) -> &str {
                self.0.as_str()
            }
        }
        impl Default for $NAME<String> {
            #[inline]
            fn default() -> Self {
                <$NAME<&str>>::default().to_owned()
            }
        }
        impl $NAME<fix_model_core::prelude::Ascii> {
            /// Borrowing inner Type<Ascii> -> Type<&asc> instead of Type<Ascii> -> &Type<Ascii>
            #[inline]
            pub fn borrow(&self) -> $NAME<&fix_model_core::prelude::asc> {
                $NAME(self.0.as_asc())
            }
            /// Ownwing inner Type<Ascii> -> Type<Ascii> instead of &Type<Ascii> -> Type<Ascii>
            #[inline]
            pub fn to_owned(&self) -> $NAME<fix_model_core::prelude::Ascii> {
                $NAME(self.0.clone())
            }
            /// Borrowing inner Ascii as &asc
            #[inline]
            pub fn value(&self) -> &fix_model_core::prelude::asc {
                self.0.as_asc()
            }
        }
        impl Default for $NAME<fix_model_core::prelude::Ascii> {
            #[inline]
            fn default() -> Self {
                <$NAME<&fix_model_core::prelude::asc>>::default().to_owned()
            }
        }
        impl $NAME<&fix_model_core::prelude::asc> {
            /// Ownwing inner Type<&asc> -> Type<Ascii> instead of &Type<&asc> -> Type<&asc>
            #[inline]
            pub fn to_owned(&self) -> $NAME<fix_model_core::prelude::Ascii> {
                $NAME(self.0.to_owned())
            }
            /// Borrowing inner Type<&asc> -> Type<&asc> instead of Type<&asc> -> &Type<&asc>
            #[inline]
            pub fn borrow(&self) -> $NAME<&fix_model_core::prelude::asc> {
                $NAME(self.0)
            }
            /// Borrowing inner &asc as &asc
            #[inline]
            pub fn value(&self) -> &fix_model_core::prelude::asc {
                self.0
            }
        }
        impl<'a> Default for $NAME<&'a fix_model_core::prelude::asc> {
            #[inline]
            fn default() -> Self {
                $NAME(fix_model_core::prelude::asc::try_from_str(concat!(stringify!($NAME), ":", stringify!($TAG), "@Default")).unwrap())
            }
        }
        impl<'a> From<&'a str> for $NAME<&'a str> {
            #[inline]
            fn from(value: &'a str) -> Self {
                Self::new(value)
            }
        }
        impl From<String> for $NAME<String> {
            #[inline]
            fn from(value: String) -> Self {
                Self::new(value)
            }
        }
        impl From<&str> for $NAME<String> {
            #[inline]
            fn from(value: &str) -> Self {
                Self::new(value.to_owned())
            }
        }
        impl From<fix_model_core::prelude::Ascii> for $NAME<fix_model_core::prelude::Ascii> {
            #[inline]
            fn from(value: fix_model_core::prelude::Ascii) -> Self {
                Self::new(value)
            }
        }
        impl TryFrom<&str> for $NAME<fix_model_core::prelude::Ascii> {
            type Error = fix_model_core::prelude::Error;
            #[inline]
            fn try_from(value: &str) -> fix_model_core::prelude::Result<Self> {
                fix_model_core::prelude::Ascii::try_from(value).map(Self::new)
            }
        }
        impl<const N: usize> TryFrom<&[u8; N]> for $NAME<fix_model_core::prelude::Ascii> {
            type Error = fix_model_core::prelude::Error;
            #[inline]
            fn try_from(value: &[u8; N]) -> fix_model_core::prelude::Result<Self> {
                fix_model_core::prelude::Ascii::try_from(value.as_slice()).map(Self::new)
            }
        }
        impl TryFrom<&str> for $NAME<&fix_model_core::prelude::asc> {
            type Error = fix_model_core::prelude::Error;
            #[inline]
            fn try_from(value: &str) -> fix_model_core::prelude::Result<Self> {
                <&fix_model_core::prelude::asc>::try_from(value).map(Self::new)
            }
        }
        impl<const N: usize> TryFrom<&[u8; N]> for $NAME<&fix_model_core::prelude::asc> {
            type Error = fix_model_core::prelude::Error;
            #[inline]
            fn try_from(value: &[u8; N]) -> fix_model_core::prelude::Result<Self> {
                <&fix_model_core::prelude::asc>::try_from(value.as_slice()).map(Self::new)
            }
        }
        $crate::_debug!($NAME, <S>);
        $crate::_display!($NAME, <S>);
        $crate::_impl_field_meta!($NAME, $TAG, <S>);
    };
}
pub use fix_string;

#[macro_export]
macro_rules! fix_char_any {
    ($NAME:ident, $TAG:literal) => {
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone, Copy)]
        pub struct $NAME<C>(C);
        impl<C> $NAME<C> {
            #[inline]
            pub fn new(value: C) -> Self {
                Self(value)
            }
        }
        impl<C: AsRef<str>> AsRef<str> for $NAME<C> {
            #[inline]
            fn as_ref(&self) -> &str {
                self.0.as_ref()
            }
        }
        impl $NAME<char> {
            #[inline]
            pub fn value(&self) -> char {
                self.0
            }
        }
        impl Default for $NAME<char> {
            #[inline]
            fn default() -> Self {
                Self('?')
            }
        }
        impl $NAME<fix_model_core::prelude::aschar> {
            #[inline]
            pub fn value(&self) -> fix_model_core::prelude::aschar {
                self.0
            }
        }
        impl Default for $NAME<fix_model_core::prelude::aschar> {
            #[inline]
            fn default() -> Self {
                Self(unsafe{fix_model_core::prelude::aschar::from_u8_unchecked(b'?')})
            }
        }
        impl From<char> for $NAME<char> {
            #[inline]
            fn from(value: char) -> Self {
                Self(value)
            }
        }
        impl From<fix_model_core::prelude::aschar> for $NAME<fix_model_core::prelude::aschar> {
            #[inline]
            fn from(value: fix_model_core::prelude::aschar) -> Self {
                Self(value)
            }
        }
        impl TryFrom<char> for $NAME<fix_model_core::prelude::aschar> {
            type Error = fix_model_core::prelude::Error;
            #[inline]
            fn try_from(value: char) -> fix_model_core::prelude::Result<Self> {
                fix_model_core::prelude::aschar::try_from(value).map(Self::new)
            }
        }
        impl TryFrom<u8> for $NAME<fix_model_core::prelude::aschar> {
            type Error = fix_model_core::prelude::Error;
            #[inline]
            fn try_from(value: u8) -> fix_model_core::prelude::Result<Self> {
                fix_model_core::prelude::aschar::try_from(value).map(Self::new)
            }
        }
        impl TryFrom<&str> for $NAME<fix_model_core::prelude::aschar> {
            type Error = fix_model_core::prelude::Error;
            #[inline]
            fn try_from(value: &str) -> fix_model_core::prelude::Result<Self> {
                fix_model_core::prelude::aschar::try_from(value).map(Self::new)
            }
        }
        $crate::_impl_field_meta!($NAME, $TAG, <C>);
        $crate::_debug!($NAME, <C>);
        $crate::_display!($NAME, <C>);
    };
}
pub use fix_char_any;

// TODO how to serialize ENUM with json using Variant Name instead of Variant Value
#[macro_export]
macro_rules! fix_ascii_char_enum {
    ($NAME:ident, $TAG:literal, $($VARIANT:tt : $VALUE:literal),*, ) => {
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone, Copy, Default)]
        pub enum $NAME{
            #[default]
            $(
                #[serde(rename = $VALUE)]
                // #[serde(alias = "$VARIANT")]
                $VARIANT
            ),*
        }
        impl $NAME{
            #[inline]
            pub fn value(&self) -> char {
                $(debug_assert!($VALUE.len() == 1 && $VALUE.is_ascii());)*
                match self {
                    $(
                        $NAME::$VARIANT => $VALUE.as_bytes()[0] as char,
                    )*
                }
            }
        }
        // $crate::_impl_new_and_value!($NAME, char);
        $crate::_impl_field_meta!($NAME, $TAG);
        $crate::_debug!($NAME, $($VARIANT: $VALUE),+);
        $crate::_display!($NAME, $($VARIANT: $VALUE),+);
    };
    ($NAME:ident, $TAG:literal, $($VARIANT:tt : $VALUE:literal),* ) => {
        $crate::fix_ascii_char_enum!($NAME, $TAG, $($VARIANT: $VALUE),* , );
    }
}
pub use fix_ascii_char_enum;

#[macro_export]
macro_rules! fix_data {
    ($NAME_LEN:ident, $TAG_LEN:literal, $NAME_DATA:ident, $TAG_DATA:literal) => {
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone, Copy, Default)]
        pub struct $NAME_DATA<D>(D);
        impl<D> $NAME_DATA<D> {
            #[inline]
            pub fn new(value: D) -> Self {
                Self(value)
            }
        }
        impl $NAME_DATA<&fix_model_core::prelude::dat> {
            /// Ownwing inner Type<&str> -> Type<String> instead of &Type<&str> -> Type<&str>
            #[inline]
            pub fn to_owned(&self) -> $NAME_DATA<fix_model_core::prelude::Data> {
                $NAME_DATA(self.0.to_owned())
            }
            /// Borrowing inner Type<&str> -> Type<&str> instead of Type<&str> -> &Type<&str>
            #[inline]
            pub fn borrow(&self) -> $NAME_DATA<&fix_model_core::prelude::dat> {
                $NAME_DATA(self.0)
            }
            /// Borrowing inner &str as &str
            #[inline]
            pub fn value(&self) -> &fix_model_core::prelude::dat {
                self.0
            }
        }
        impl $NAME_DATA<fix_model_core::prelude::Data> {
            /// Ownwing inner Type<&str> -> Type<String> instead of &Type<&str> -> Type<&str>
            #[inline]
            pub fn to_owned(&self) -> $NAME_DATA<fix_model_core::prelude::Data> {
                $NAME_DATA(self.0.to_owned())
            }
            /// Borrowing inner Type<&str> -> Type<&str> instead of Type<&str> -> &Type<&str>
            #[inline]
            pub fn borrow(&self) -> $NAME_DATA<&fix_model_core::prelude::dat> {
                $NAME_DATA(self.0.as_dat())
            }
            /// Borrowing inner &str as &str
            #[inline]
            pub fn value(&self) -> &fix_model_core::prelude::dat {
                self.0.as_dat()
            }
        }
        impl<'a> $NAME_DATA<fix_model_core::prelude::dat_codec<'a>> {
            /// Borrowing inner Type<Ascii> -> Type<&asc> instead of Type<Ascii> -> &Type<Ascii>
            #[inline]
            pub fn borrow(&self) -> $NAME_DATA<&fix_model_core::prelude::dat> {
                $NAME_DATA(self.0.as_dat())
            }
            /// Ownwing inner Type<Ascii> -> Type<Ascii> instead of &Type<Ascii> -> Type<Ascii>
            #[inline]
            pub fn to_owned(&self) -> $NAME_DATA<fix_model_core::prelude::Data> {
                $NAME_DATA(self.0.as_dat().to_owned())
            }
            /// Borrowing inner Ascii as &asc
            #[inline]
            pub fn value(&self) -> &fix_model_core::prelude::dat {
                self.0.as_dat()
            }
            #[inline]
            pub fn decode(&mut self) -> Result<(), fix_model_core::prelude::Error> {
                self.0.decode()
            }
        }
        impl From<Vec<u8>> for $NAME_DATA<fix_model_core::prelude::Data> {
            #[inline]
            fn from(value: Vec<u8>) -> Self {
                $NAME_DATA(value.into())
            }
        }
        impl<const N: usize> From<&[u8; N]> for $NAME_DATA<fix_model_core::prelude::Data> {
            #[inline]
            fn from(value: &[u8; N]) -> Self {
                $NAME_DATA(value.to_vec().into())
            }
        }
        impl<'a> From<&'a [u8]> for $NAME_DATA<&'a fix_model_core::prelude::dat> {
            #[inline]
            fn from(value: &'a [u8]) -> Self {
                $NAME_DATA(value.into())
            }
        }
        impl<'a, const N: usize> From<&'a [u8; N]> for $NAME_DATA<&'a fix_model_core::prelude::dat> {
            #[inline]
            fn from(value: &'a [u8; N]) -> Self {
                $NAME_DATA(value.as_slice().into())
            }
        }
        impl<'a> From<&'a [u8]> for $NAME_DATA<fix_model_core::prelude::dat_codec<'a>> {
            #[inline]
            fn from(value: &'a [u8]) -> Self {
                $NAME_DATA(value.into())
            }
        }
        impl<'a, const N: usize> From<&'a [u8; N]> for $NAME_DATA<fix_model_core::prelude::dat_codec<'a>> {
            #[inline]
            fn from(value: &'a [u8; N]) -> Self {
                $NAME_DATA(value.as_slice().into())
            }
        }
        // impl Default for $NAME_DATA<fix_model_core::prelude::Data> {
        //     #[inline]
        //     fn default() -> Self {
        //         <$NAME_DATA<&fix_model_core::prelude::dat>>::default().to_owned()
        //     }
        // }
        // impl Default for $NAME_DATA<&fix_model_core::prelude::dat> {
        //     #[inline]
        //     fn default() -> Self {
        //         $NAME_DATA(fix_model_core::prelude::dat::from_slice(concat!(stringify!($NAME_DATA), ":", stringify!($TAG_DATA), "@Default").as_bytes()))
        //     }
        // }
        // impl<'a> Default for $NAME_DATA<fix_model_core::prelude::dat_codec<'a>> {
        //     #[inline]
        //     fn default() -> Self {
        //         $NAME_DATA(fix_model_core::prelude::dat_codec::from_slice(concat!(stringify!($NAME_DATA), ":", stringify!($TAG_DATA), "@Default").as_bytes()))
        //     }
        // }

        impl<D: std::fmt::Display + std::fmt::Debug> std::fmt::Display for $NAME_DATA<D>{
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use fix_model_core::prelude::FieldMeta;
                let mut bin = String::new();
                use std::fmt::Write;
                write!(bin, "{}", self.0)?;

                if f.sign_plus() {
                    // write!(f, "{}={}", $NAME_DATA::<D>::TAG_NAME, self.0)
                    write!(f, "{}={}|{}={}", $TAG_LEN, bin.len(), $TAG_DATA, bin)
                } else if f.sign_minus() {
                    // write!(f, "{}={}", $NAME_DATA::<D>::SHORT_NAME, self.0)
                    write!(f, "{}={}|{}={}", stringify!($NAME_LEN), bin.len(), stringify!($NAME_DATA), bin)
                } else {
                    write!(f, "{}", bin)
                }
            }
        }
        $crate::_debug!($NAME_DATA, <D>);
        $crate::_impl_field_meta!($NAME_DATA, $TAG_DATA, <D>);
    };
}
pub use fix_data;

#[macro_export]
macro_rules! _fix_numeric_fixed_length {
    ($NAME:ident, $TAG:literal, $TY:tt, $LEN:literal) => {
        #[derive(serde::Deserialize, PartialEq, Clone, Default)]
        pub struct $NAME($TY);
        impl serde::Serialize for $NAME {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
                if serializer.is_human_readable() {
                    serializer.serialize_u64(self.0 as u64)
                } else {
                    let mut buf = itoa::Buffer::new();
                    let value = buf.format(self.0);
                    use std::io::Write;
                    let mut buf_pad = [0u8; $LEN];
                    write!(&mut buf_pad[..], concat!("{:0>", stringify!($LEN), "}"), value)
                        .expect(concat!("Failed serialize ", stringify!($NAME), "(" , stringify!($TY), ")", " into a buffer of size ", stringify!($LEN)));
                    serializer.serialize_bytes(&buf_pad)
                }
            }
        }
        impl From<$TY> for $NAME {
            #[inline]
            fn from(value: $TY) -> Self {
                Self::new(value)
            }
        }
        $crate::_impl_new_and_value!($NAME, $TY);
        $crate::_impl_field_meta!($NAME, $TAG);
        $crate::_debug!($NAME);
        $crate::_display!($NAME);
    };
}
#[macro_export]
macro_rules! fix_usize_fixed_length {
    ($NAME:ident, $TAG:literal) => {
        $crate::_fix_numeric_fixed_length!($NAME, $TAG, usize, 20);
    };
}
pub use fix_usize_fixed_length;
#[macro_export]
macro_rules! fix_u8_fixed_length {
    ($NAME:ident, $TAG:literal) => {
        $crate::_fix_numeric_fixed_length!($NAME, $TAG, u8, 3);
    };
}
pub use fix_u8_fixed_length;

#[macro_export]
macro_rules! _fix_numeric {
    ($NAME:ident, $TAG:literal, $TY:tt) => {
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone, Default)]
        pub struct $NAME($TY);
        impl From<$TY> for $NAME {
            #[inline]
            fn from(value: $TY) -> Self {
                Self::new(value)
            }
        }
        $crate::_impl_new_and_value!($NAME, $TY);
        $crate::_impl_field_meta!($NAME, $TAG);
        $crate::_debug!($NAME);
        $crate::_display!($NAME);
    };
}

#[macro_export]
macro_rules! fix_usize {
    ($NAME:ident, $TAG:literal) => {
        $crate::_fix_numeric!($NAME, $TAG, usize);
    };
}
pub use fix_usize;

#[macro_export]
macro_rules! fix_isize {
    ($NAME:ident, $TAG:literal) => {
        $crate::_fix_numeric!($NAME, $TAG, isize);
    };
}
pub use fix_isize;

#[macro_export]
macro_rules! fix_float64 {
    ($NAME:ident, $TAG:literal) => {
        $crate::_fix_numeric!($NAME, $TAG, f64);
    };
}
pub use fix_float64;
#[macro_export]
macro_rules! fix_float32 {
    ($NAME:ident, $TAG:literal) => {
        $crate::_fix_numeric!($NAME, $TAG, f32);
    };
}
pub use fix_float32;

#[macro_export]
macro_rules! fix_bool {
    ($NAME:ident, $TAG:literal) => {
        /// FIX boolean field, represented as a char 'Y' or 'N' because we want both fix serializer to
        /// output a char and not have a special boolean serialization/deserialization logic
        #[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone, Default)]
        pub struct $NAME(bool);
        impl From<bool> for $NAME {
            #[inline]
            fn from(value: bool) -> Self {
                Self::new(value)
            }
        }
        $crate::_impl_new_and_value!($NAME, bool);
        $crate::_impl_field_meta!($NAME, $TAG);
        $crate::_debug!($NAME);
        impl std::fmt::Display for $NAME {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use fix_model_core::prelude::FieldMeta;
                let value = if self.0 { 'Y' } else { 'N' };
                if f.sign_plus() {
                    write!(f, "{}={}", $NAME::TAG_NAME, value)
                } else if f.sign_minus() {
                    write!(f, "{}={}", $NAME::SHORT_NAME, value)
                } else {
                    write!(f, "{}", value)
                }
            }
        }
    };
}
pub use fix_bool;

// ########################################################################################################

#[macro_export]
macro_rules! _impl_new_and_value {
    ($NAME:ident, $TYPE:tt) => {
        impl $NAME {
            #[inline]
            pub fn new(value: $TYPE) -> Self {
                Self(value)
            }
            #[inline]
            pub fn value(&self) -> $TYPE {
                self.0
            }
        }
    };
}
#[macro_export]
macro_rules! _impl_field_meta {
    ($NAME:ident, $TAG:literal) => {
        impl fix_model_core::prelude::FieldMeta for $NAME {
            const TAG: usize = $TAG;
            const TAG_NAME: &'static str = stringify!($TAG);
            const SHORT_NAME: &'static str = stringify!($NAME);
            const SHORT_NAME_WITH_TAG_NAME: &'static str = concat!(stringify!($NAME), ":", stringify!($TAG));
        }
    };
    ($NAME:ident, $TAG:literal, <$GEN:tt>) => {
        impl<$GEN> fix_model_core::prelude::FieldMeta for $NAME<$GEN> {
            const TAG: usize = $TAG;
            const TAG_NAME: &'static str = stringify!($TAG);
            const SHORT_NAME: &'static str = stringify!($NAME);
            const SHORT_NAME_WITH_TAG_NAME: &'static str = concat!(stringify!($NAME), ":", stringify!($TAG));
        }
    };
    ($NAME_LEN:ident, $TAG_LEN:literal, $NAME_DATA:ident, $TAG_DATA:literal, <$GEN:tt>) => {
        impl<$GEN> fix_model_core::prelude::FieldMeta for $NAME<$GEN> {
            const TAG: usize = $TAG_LEN;
            const TAG_NAME: &'static str = stringify!($TAG_LEN);
            const SHORT_NAME: &'static str = concat!(stringify!($NAME_LEN), "/", stringify!($NAME_DATA));
            const SHORT_NAME_WITH_TAG_NAME: &'static str = concat!(
                stringify!($NAME_LEN),
                ":",
                stringify!($TAG_LEN),
                "/",
                stringify!($NAME_DATA),
                ":",
                stringify!($TAG_DATA)
            );
        }
    };
}
#[macro_export]
macro_rules! _debug {
    // vanila new type
    ($NAME:ident) => {
        impl std::fmt::Debug for $NAME {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use fix_model_core::prelude::FieldMeta;
                if f.sign_plus() {
                    // f.debug_tuple($NAME::TAG_NAME).field(&self.0).finish()
                    write!(f, "{}({:?})", $NAME::TAG_NAME, self.0) // affects if + is printed
                } else if f.sign_minus() {
                    // f.debug_tuple($NAME::SHORT_NAME).field(&self.0).finish()
                    write!(f, "{}({:?})", $NAME::SHORT_NAME, self.0) // for consistency with +
                } else {
                    f.debug_tuple($NAME::SHORT_NAME_WITH_TAG_NAME).field(&self.0).finish()
                }
            }
        }
    };
    // new type String/str/Ascii/asc with geneneric
    ($NAME:ident, <$GEN:tt>) => {
        impl<$GEN: std::fmt::Debug> std::fmt::Debug for $NAME<$GEN> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use fix_model_core::prelude::FieldMeta;
                if f.sign_plus() {
                    // f.debug_tuple($NAME::TAG_NAME).field(&self.0).finish()
                    write!(f, "{}({:?})", $NAME::<$GEN>::TAG_NAME, self.0) // affects if + is printed
                } else if f.sign_minus() {
                    // f.debug_tuple($NAME::SHORT_NAME).field(&self.0).finish()
                    write!(f, "{}({:?})", $NAME::<$GEN>::SHORT_NAME, self.0) // for consistency with +
                } else {
                    f.debug_tuple($NAME::<$GEN>::SHORT_NAME_WITH_TAG_NAME).field(&self.0).finish()
                }
            }
        }
    };
    // enum type
    ($NAME:ident, $($VARIANT:tt: $VALUE:literal),*) => {
        impl std::fmt::Debug for $NAME {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use fix_model_core::prelude::FieldMeta;
                $(debug_assert!($VALUE.len() == 1 && $VALUE.is_ascii());)*
                if f.sign_plus() {
                    // f.debug_tuple($NAME::TAG_NAME).field(&self.0).finish()
                    match self { // affects if + is printed
                        $(
                            $NAME::$VARIANT => write!(f, "{}('{}')", $NAME::TAG_NAME, $VALUE),
                        )*
                    }

                } else if f.sign_minus() {
                    // f.debug_tuple($NAME::SHORT_NAME).field(&self.0).finish()
                    match self{ // for consistency with +
                        $(
                            $NAME::$VARIANT => write!(f, "{}('{}')", $NAME::SHORT_NAME, stringify!($VARIANT)),
                        )*
                    }
                } else {
                    // f.debug_tuple($NAME::SHORT_NAME_WITH_TAG_NAME).field(&self).finish()
                    match self { // affects if + is printed
                        $(
                            $NAME::$VARIANT => write!(f, "{}({}:'{}')", $NAME::SHORT_NAME_WITH_TAG_NAME, stringify!($VARIANT), $VALUE),
                        )*
                    }
                }
            }
        }
    };
}
#[macro_export]
macro_rules! _display {
    // vanila new type
    ($NAME:ident) => {
        impl std::fmt::Display for $NAME {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use fix_model_core::prelude::FieldMeta;
                if f.sign_plus() {
                    write!(f, "{}={}", $NAME::TAG_NAME, self.0)
                } else if f.sign_minus() {
                    write!(f, "{}={}", $NAME::SHORT_NAME, self.0)
                } else {
                    write!(f, "{}", self.0)
                }
            }
        }
    };
    // new type with geneneric
    ($NAME:ident, <$GEN:tt>) => {
        impl<$GEN: std::fmt::Display> std::fmt::Display for $NAME<$GEN> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use fix_model_core::prelude::FieldMeta;
                if f.sign_plus() {
                    write!(f, "{}={}", $NAME::<$GEN>::TAG_NAME, self.0)
                } else if f.sign_minus() {
                    write!(f, "{}={}", $NAME::<$GEN>::SHORT_NAME, self.0)
                } else {
                    write!(f, "{}", self.0)
                }
            }
        }
    };
    // enum type
    ($NAME:ident, $($VARIANT:tt: $VALUE:literal),*) => {
        impl std::fmt::Display for $NAME {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use fix_model_core::prelude::FieldMeta;
                $(debug_assert!($VALUE.len() == 1 && $VALUE.is_ascii());)*
                if f.sign_plus() {
                    // f.debug_tuple($NAME::TAG_NAME).field(&self.0).finish()
                    match self { // affects if + is printed
                        $(
                            $NAME::$VARIANT => write!(f, "{}={}", $NAME::TAG_NAME, $VALUE),
                        )*
                    }

                } else if f.sign_minus() {
                    match self{
                        $(
                            $NAME::$VARIANT => write!(f, "{}={}", $NAME::SHORT_NAME, $VALUE),
                        )*
                    }
                } else {
                    match self {
                        $(
                            $NAME::$VARIANT =>  write!(f, "{}", $VALUE),
                        )*
                    }
                }
            }
        }
    };
}
