pub mod de;
pub mod error;
pub mod framing;
pub(crate) mod macros;
pub mod prelude;
pub mod ser;

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
fn se_from_trait<W: Write, T: serde::Serialize, S: Schema>(write: W, value: T) -> Result<Serializer<W, S>> {
    let mut serializer = Serializer::new(write);
    value.serialize(&mut serializer)?;
    Ok(serializer)
}

pub fn to_bytes_with_schema<T: serde::Serialize, S: Schema>(value: &T, capacity: Option<usize>) -> Result<Serializer<BytesWrite, S>> {
    let write = BytesWrite::new(BytesMut::with_capacity(capacity.unwrap_or_default()));
    se_from_trait(write, value)
}
/// Will not preallocate capacity for the output buffer and will generate invalid output if T contains binary data as it requires a Schema.
pub fn to_bytes<T: serde::Serialize>(value: &T) -> Result<Serializer<BytesWrite, NoBinaryDataSchema>> {
    to_bytes_with_schema::<_, NoBinaryDataSchema>(value, None)
}

// ////////////////////////////////////////////////////////////////////////////
fn de_from_trait<'de, R: Read<'de>, T: serde::Deserialize<'de>, X: Schema>(read: R) -> Result<(T, Deserializer<R, X>)> {
    let mut deserializer = Deserializer::new(read);
    let t = T::deserialize(&mut deserializer)?;
    Ok((t, deserializer))
}
pub fn from_slice_with_schema<'de, T: serde::Deserialize<'de>, X: Schema>(slice: &'de [u8]) -> Result<T> {
    let (t, mut d) = de_from_trait::<_, T, X>(SliceRead::new(slice))?;
    d.end()?;
    Ok(t)
}

pub fn from_slice<'de, T: serde::Deserialize<'de>>(slice: &'de [u8]) -> Result<T> {
    let (t, mut d) = de_from_trait::<_, T, NoBinaryDataSchema>(SliceRead::new(slice))?;
    d.end()?;
    Ok(t)
}
// ////////////////////////////////////////////////////////////////////////////
pub fn new_serializer_with_capacity(capacity: usize) -> Serializer<BytesWrite, NoBinaryDataSchema> {
    Serializer::new(BytesWrite::new(BytesMut::with_capacity(capacity)))
}
pub fn new_serializer_with_schema<X: Schema>(capacity: usize) -> Serializer<BytesWrite, X> {
    Serializer::new(BytesWrite::new(BytesMut::with_capacity(capacity)))
}
pub fn new_deserializer(slice: &[u8]) -> Deserializer<SliceRead, NoBinaryDataSchema> {
    Deserializer::<_, NoBinaryDataSchema>::new(SliceRead::new(slice))
}
pub fn new_deserializer_with_schema<'de, T: serde::Deserialize<'de>, X: Schema>(slice: &'de [u8]) -> Deserializer<SliceRead, X> {
    Deserializer::new(SliceRead::new(slice))
}
