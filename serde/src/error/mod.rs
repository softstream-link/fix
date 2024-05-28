#[derive(Debug)]
pub struct IssueAtPosition(pub usize);
impl From<usize> for IssueAtPosition {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value)
    }
}

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
    SliceIsNotValidAscii,
    UnexpectedEof(IssueAtPosition),
    TrailingBytes,
    EmptyValue(IssueAtPosition),
    InvalidInteger,
    InvalidFixFrame(IssueAtPosition),
    NotSupported(&'static str),
    InvalidChecksum(IssueAtPosition),
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
            Error::SliceIsNotValidAscii => write!(f, "SliceIsNotValidAscii"),
            Error::TrailingBytes => write!(f, "TrailingBytes"),
            Error::UnexpectedEof(pos) => write!(f, "UnexpectedEof: {:?}", pos),
            Error::EmptyValue(pos) => write!(f, "EmptyValue {:?}", pos),
            Error::InvalidInteger => write!(f, "InvalidInteger"),
            Error::NotSupported(s) => write!(f, "NotSupported: {}", s),
            Error::InvalidFixFrame(pos) => write!(f, "InvalidFixFrame {:?}", pos),
            Error::InvalidChecksum(pos) => write!(f, "InvalidChecksum {:?}", pos),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
