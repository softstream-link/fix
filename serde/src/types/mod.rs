pub mod fixstr;
pub mod fixstring;
pub mod fixtag;

// pub trait Value: Serialize + Display + Debug + Clone + PartialEq {}

pub trait FixStringLike: AsRef<crate::prelude::FixStr> {}
