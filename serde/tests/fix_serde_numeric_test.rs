use fix_model_test::unittest::setup;
use fix_serde::unittest::{from_slice_unittest, to_bytes_unittest};
use log::info;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    any::type_name,
    fmt::{Debug, Display},
};

fn test_numeric_serde<T: Display + Debug + PartialEq + Serialize + DeserializeOwned>(inp: T) -> T {
    info!("inp: {}, type: {}", inp, type_name::<T>());
    let fix_ser = to_bytes_unittest(&inp).unwrap();
    info!("fix_ser: {}", fix_ser);
    let json_ser = serde_json::to_string(&inp).unwrap();
    info!("json_ser: {}", json_ser);

    let out_fix: T = from_slice_unittest(&fix_ser).unwrap();
    info!("out_fix: {:?}", out_fix);
    let out_json: T = serde_json::from_str(&json_ser).unwrap();
    info!("out_json: {:?}", out_json);
    assert_eq!(out_fix, out_json);
    assert_eq!(out_fix, inp);
    out_fix
}

#[test]
fn test_numerics() {
    setup::log::configure();

    assert_eq!(test_numeric_serde(usize::MAX), usize::MAX);
    assert_eq!(test_numeric_serde(usize::MIN), usize::MIN);
    assert_eq!(test_numeric_serde(u64::MAX), u64::MAX);
    assert_eq!(test_numeric_serde(u64::MIN), u64::MIN);
    assert_eq!(test_numeric_serde(u32::MAX), u32::MAX);
    assert_eq!(test_numeric_serde(u32::MIN), u32::MIN);
    assert_eq!(test_numeric_serde(u16::MAX), u16::MAX);
    assert_eq!(test_numeric_serde(u16::MIN), u16::MIN);
    assert_eq!(test_numeric_serde(u8::MAX), u8::MAX);
    assert_eq!(test_numeric_serde(u8::MIN), u8::MIN);

    assert_eq!(test_numeric_serde(i64::MAX), i64::MAX);
    assert_eq!(test_numeric_serde(i64::MIN), i64::MIN);
    assert_eq!(test_numeric_serde(i32::MAX), i32::MAX);
    assert_eq!(test_numeric_serde(i32::MIN), i32::MIN);
    assert_eq!(test_numeric_serde(i16::MAX), i16::MAX);
    assert_eq!(test_numeric_serde(i16::MIN), i16::MIN);
    assert_eq!(test_numeric_serde(i8::MAX), i8::MAX);
    assert_eq!(test_numeric_serde(i8::MIN), i8::MIN);

    assert_eq!(test_numeric_serde(f32::MIN), f32::MIN);
    assert_eq!(test_numeric_serde(f64::MAX), f64::MAX);
}
