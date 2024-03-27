pub mod de;
pub mod prelude;
pub mod ser;
pub mod error;
pub mod types;


const SOH: u8 = 0x01;
const EQS: u8 = b'=';
const SOH_CHAR: char = SOH as char;
const PIPE_STR: &'static str = "|";

#[cfg(feature = "unittest")]
pub mod unittest;