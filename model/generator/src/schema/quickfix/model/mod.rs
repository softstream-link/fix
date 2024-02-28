pub mod component_def;
pub mod field_def;
pub mod message_def;
pub mod refs;
pub mod repgroup_def;
pub mod root;
pub mod header_trailer_def;

#[derive(Debug, Clone)]
pub enum Error {
    QuickFixFieldTypeNotMapped(String),
    QuickFixMessageCategoryNotMapped(String),
    QuickFixMessageMissingPart { msg: String, name: String },
}
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::QuickFixFieldTypeNotMapped(s) => write!(f, "QuickFixFieldTypeNotMapped '{}' will be skipped", s),
            Error::QuickFixMessageCategoryNotMapped(s) => write!(f, "QuickFixMessageCategoryNotMapped '{}' will be skipped", s),
            Error::QuickFixMessageMissingPart { msg, name: fld } => {
                write!(f, "QuickFixMessageMissingPart message: '{}' missing field: '{}'", msg, fld)
            }
        }
    }
}
