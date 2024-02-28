#[derive(Debug)]
pub enum Error {
    NotAsciiChar(char),
    NotAsciiString(String),
    NotAsciiBytes(Vec<u8>),
    NotAsciiStrOrNotSingleChar(String),
    NotBase64String(String),
}
impl Error {
    #[cold]
    pub fn not_ascii_char(c: char) -> Self {
        Self::NotAsciiChar(c)
    }
    #[cold]
    pub fn not_ascii_string(s: String) -> Self {
        Self::NotAsciiString(s)
    }
    #[cold]
    pub fn not_ascii_bytes(b: Vec<u8>) -> Self {
        Self::NotAsciiBytes(b)
    }
    #[cold]
    pub fn not_ascii_str_or_not_single_char(s: String) -> Self {
        Self::NotAsciiStrOrNotSingleChar(s)
    }
}
impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotAsciiChar(c) => write!(f, r#"Not an ASCII character: '{}'"#, c),
            Error::NotAsciiString(s) => write!(f, r#"Not an ASCII string: "{}""#, s),
            Error::NotAsciiBytes(b) => write!(f, "Not an ASCII bytes: {:?}", b),
            Error::NotAsciiStrOrNotSingleChar(s) => write!(f, r#"Not an ASCII string or not a single character: "{}""#, s),
            Error::NotBase64String(s) => write!(f, r#"Not a base64 string: "{}""#, s),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
