use serde::{Deserialize, Serialize};

use super::message_def::QFMessagePart;

#[derive(Debug, Deserialize, Serialize)] 
#[serde(rename = "header")]
pub struct QFHeaderDef {
    #[serde(rename = "$value")]
    pub(super) parts: Vec<QFMessagePart>,
}


#[derive(Debug, Deserialize, Serialize)] 
#[serde(rename = "trailer")]
pub struct QFTrailerDef {
    #[serde(rename = "$value")]
    pub(super) parts: Vec<QFMessagePart>,
}