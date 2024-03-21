pub mod de;
pub mod prelude;
pub mod ser;
pub mod error;
pub mod types;


const SOH_U8: u8 = 0x01;
const SOH_CHAR: char = SOH_U8 as char;
const PIPE_STR: &'static str = "|";
const EQS_U8: u8 = b'=';

#[cfg(feature = "unittest")]
pub mod unittest;