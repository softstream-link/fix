use serde::{Deserialize, Serialize};

use super::message_def::QFMessageParts;

#[derive(Debug, Deserialize, Serialize)] 
#[serde(rename = "header")]
pub struct QFHeaderDef {
    #[serde(rename = "$value")]
    pub(super) parts: Vec<QFMessageParts>,
}


#[derive(Debug, Deserialize, Serialize)] 
#[serde(rename = "trailer")]
pub struct QFTrailerDef {
    #[serde(rename = "$value")]
    pub(super) parts: Vec<QFMessageParts>,
}