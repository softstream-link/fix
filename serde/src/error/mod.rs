use crate::prelude::FixString;

#[derive(Debug)]
pub enum Error {
    // One or more variants that can be created by data structures through the
    // `ser::Error` and `de::Error` traits. For example the Serialize impl for
    // Mutex<T> might return an error because the mutex is poisoned, or the
    // Deserialize impl for a struct may return an error because a required
    // field is missing.
    Message(String),

    // Zero or more variants that can be created directly by the Serializer and
    // Deserializer without going through `ser::Error` and `de::Error`. These
    // are specific to the format, in this case FIX.
    TagStringIsNotNumeric(FixString),
    SliceIsNotValidAscii,
    Eof,
    TrailingBytes,
    EmptyValue,
    InvalidUnsignedInteger,
    TODO,
}
impl std::error::Error for Error {}
impl serde::ser::Error for Error {
    #[inline(always)]
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}
impl serde::de::Error for Error {
    #[inline(always)]
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Message(msg) => write!(f, "Message: {}", msg),
            Error::TagStringIsNotNumeric(val) => write!(f, "TagStringIsNotNumeric: {}", val),
            Error::SliceIsNotValidAscii => write!(f, "SliceIsNotValidAscii"),
            Error::TrailingBytes => write!(f, "TrailingBytes"),
            Error::Eof => write!(f, "StreamEndsWithOutSOH"),
            Error::EmptyValue => write!(f, "EmptyValue"),
            Error::InvalidUnsignedInteger => write!(f, "InvalidUnsignedInteger"),
            Error::TODO => write!(f, "TODO"),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod test {

    use super::*;
    use crate::unittest::setup;
    #[test]
    fn test_error() {
        setup::log::configure();
        // info!("err: {:?}", err);
        // assert!(matches!(x, Error::Message(_)));
    }
}
