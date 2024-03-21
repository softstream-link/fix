use crate::prelude::FixString;
use std::error::Error;

#[derive(Debug)]
pub enum FixErrorKind {
    TagStringIsNotNumeric(FixString),
}
impl std::fmt::Display for FixErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FixErrorKind::TagStringIsNotNumeric(tag) => write!(f, "Tag is not numeric: {:?}", tag),
        }
    }
}

#[derive(Debug)]
pub struct FixError {
    pub kind: FixErrorKind,
}
impl FixError {
    pub fn kind(&self) -> &FixErrorKind {
        &self.kind
    }
    pub fn from_kind(kind: FixErrorKind) -> Self {
        kind.into()
    }
}
impl From<FixErrorKind> for FixError {
    fn from(kind: FixErrorKind) -> Self {
        Self { kind }
    }
}
impl std::fmt::Display for FixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}
impl Error for FixError {}
