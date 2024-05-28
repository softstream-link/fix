use crate::error::Error;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, Copy)]
pub struct aschar(u8);
impl aschar {
    #[inline]
    pub fn as_char(&self) -> char {
        self.0 as char
    }
    #[inline]
    pub fn as_u8(&self) -> u8 {
        self.0
    }
    #[inline]
    /// # Safety
    /// Caller must ensure that the input u8 is valid ascii
    pub unsafe fn from_u8_unchecked(c: u8) -> Self {
        Self(c)
    }
}
impl std::fmt::Debug for aschar {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.as_char())
    }
}
impl std::fmt::Display for aschar {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_char())
    }
}
impl TryFrom<char> for aschar {
    type Error = Error;
    #[inline]
    fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
        if !c.is_ascii() {
            return Err(Error::not_ascii_char(c));
        }
        Ok(Self(c as u8))
    }
}
impl TryFrom<u8> for aschar {
    type Error = Error;
    #[inline]
    fn try_from(c: u8) -> std::result::Result<Self, Self::Error> {
        if !c.is_ascii() {
            return Err(Error::not_ascii_char(c as char));
        }
        Ok(Self(c))
    }
}
impl TryFrom<&str> for aschar {
    type Error = Error;
    #[inline]
    fn try_from(s: &str) -> std::result::Result<Self, Self::Error> {
        if s.len() != 1 || !s.is_ascii() {
            return Err(Error::not_ascii_str_or_not_single_char(s.to_owned()));
        }
        Ok(Self(s.as_bytes()[0]))
    }
}

impl serde::ser::Serialize for aschar {
    #[inline]
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_char(self.as_char())
    }
}
impl<'de> serde::de::Deserialize<'de> for aschar {
    #[inline]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        struct AsciiCharVisitor;
        impl<'de> serde::de::Visitor<'de> for AsciiCharVisitor {
            type Value = aschar;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an aschar")
            }

            fn visit_borrowed_str<E: serde::de::Error>(self, v: &'de str) -> std::result::Result<Self::Value, E> {
                aschar::try_from(v).map_err(|e| E::custom(e))
            }
        }
        deserializer.deserialize_char(AsciiCharVisitor)
    }
}

#[cfg(test)]
mod test {
    use log::info;

    use fix_model_test::unittest::setup;

    use super::*;
    #[test]
    fn test_ascii_char() {
        setup::log::configure();

        let c: aschar = 'A'.try_into().unwrap();

        info!("c: {:?}", c);
        assert_eq!(c.as_char(), 'A');
    }
}
