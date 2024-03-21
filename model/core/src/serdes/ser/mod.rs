pub mod heap;
pub mod types;
pub mod ascii;

use crate::prelude::{Field, Tag, Value};

pub trait Serialize{
    fn serialize(&self, ser: &mut impl Serializer);
}

pub trait Serializer {
    // fn serialize_tag(&mut self, tag: impl Tag);
    fn serialize_slice(&mut self, value: &[u8]);
    fn serialize_u8(&mut self, value: u8);
    fn serialize_soh(&mut self);
    fn serialize_eqs(&mut self);
    fn serialize_value(&mut self, value: impl Value);
    fn serialize_tag_value(&mut self, tag: Tag, value: impl Value);
    fn serialize_field(&mut self, field: &impl Field);
}
