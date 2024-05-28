// THIS IS A SYM LINK

pub use fix_serde::prelude::{BeginString, BodyLength, Header1EnvelopeSequence};
pub use fix_serde::prelude::{Header2TypeCompIdSequence, MsgType, SenderCompID, TargetCompID};

pub use fix_serde::prelude::CheckSum;

include!(concat!(env!("OUT_DIR"), "/fields.rs"));
include!(concat!(env!("OUT_DIR"), "/messages_defs.rs"));
include!(concat!(env!("OUT_DIR"), "/messages_impls.rs"));
include!(concat!(env!("OUT_DIR"), "/repgrps_defs.rs"));
include!(concat!(env!("OUT_DIR"), "/repgrps_impls.rs"));
include!(concat!(env!("OUT_DIR"), "/msg_enums.rs"));
include!(concat!(env!("OUT_DIR"), "/index.rs"));
include!(concat!(env!("OUT_DIR"), "/helpers.rs"));
