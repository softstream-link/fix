// THIS IS A SYM LINK in SRC tree

pub use fix_serde::prelude::{BeginString, BodyLength, Header1EnvelopeSequence};
pub use fix_serde::prelude::{Header2CompIdSequence, Header2TypeCompIdSequence, MsgType, SenderCompID, TargetCompID};

pub use fix_serde::prelude::CheckSum;

pub type FrameEnchoder<S> = fix_serde::prelude::FrameEnchoder<S, Fix44Schema>;
pub type FrameDecoder<'de> = fix_serde::prelude::FrameDecoder<'de, Fix44Schema>;

include!(concat!(env!("OUT_DIR"), "/fields.rs"));
include!(concat!(env!("OUT_DIR"), "/messages_defs.rs"));
include!(concat!(env!("OUT_DIR"), "/messages_impls.rs"));
include!(concat!(env!("OUT_DIR"), "/repgrps_defs.rs"));
include!(concat!(env!("OUT_DIR"), "/repgrps_impls.rs"));
include!(concat!(env!("OUT_DIR"), "/msg_enums.rs"));
include!(concat!(env!("OUT_DIR"), "/schema.rs"));
include!(concat!(env!("OUT_DIR"), "/helpers.rs"));
