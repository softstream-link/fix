pub mod de;
pub mod error;
pub(crate) mod macros;
pub mod prelude;
pub mod ser;
pub mod framing;

const SOH: u8 = 0x01;
const EQS: u8 = b'=';

#[cfg(debug_assertions)]
pub mod unittest;

use bytes::BytesMut;
use de::deserializer::Deserializer;
use de::read::{Read, SliceRead};
use error::Result;
use fix_model_core::schema::{NoBinaryDataSchema, Schema};
use ser::serializer::Serializer;
use ser::write::{BytesWrite, Write};

// ////////////////////////////////////////////////////////////////////////////
fn se_from_trait<W: Write, T: serde::Serialize, S: Schema>(write: W, value: T, schema: S) -> Result<Serializer<W, S>> {
    let mut serializer = Serializer::new(write, false, schema);
    value.serialize(&mut serializer)?;
    Ok(serializer)
}

pub fn to_bytes_with_schema<T: serde::Serialize, S: Schema>(value: &T, capacity: Option<usize>, schema: S) -> Result<Serializer<BytesWrite, S>> {
    let write = BytesWrite::new(BytesMut::with_capacity(capacity.unwrap_or_default()));
    let ser = se_from_trait(write, value, schema);
    ser
}
/// Will not preallocate capacity for the output buffer and will generate invalid output if T contains binary data as it requires a Schema.
pub fn to_bytes<T: serde::Serialize>(value: &T) -> Result<Serializer<BytesWrite, NoBinaryDataSchema>> {
    to_bytes_with_schema(value, None, NoBinaryDataSchema)
}

// ////////////////////////////////////////////////////////////////////////////
fn de_from_trait<'de, R: Read<'de>, T: serde::Deserialize<'de>, S: Schema>(read: R, schema: S) -> Result<(T, Deserializer<R, S>)> {
    let mut deserializer = Deserializer::new(read, false, schema);
    let t = T::deserialize(&mut deserializer)?;
    Ok((t, deserializer))
}
pub fn from_slice_with_schema<'de, T: serde::Deserialize<'de>, S: Schema>(slice: &'de [u8], schema: S) -> Result<T> {
    let (t, mut d) = de_from_trait(SliceRead::new(slice), schema)?;
    d.end()?;
    Ok(t)
}
pub fn from_slice<'de, T: serde::Deserialize<'de>>(slice: &'de [u8]) -> Result<T> {
    let (t, mut d) = de_from_trait(SliceRead::new(slice), NoBinaryDataSchema)?;
    d.end()?;
    Ok(t)
}

// ////////////////////////////////////////////////////////////////////////////
