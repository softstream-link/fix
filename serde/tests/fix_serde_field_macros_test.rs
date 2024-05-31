use fix_model_core::{
    prelude::{asc, Ascii},
    types::{asciichar::aschar, dat::dat, dat_codec::dat_codec, data::Data},
};
use fix_model_generator::{
    fix_u8_fixed_length, fix_usize_fixed_length,
    prelude::{fix_ascii_char_enum, fix_bool, fix_char_any, fix_data, fix_string, fix_usize},
};
use fix_model_test::unittest::setup;
use fix_serde::unittest::{from_slice_unittest, to_bytes_unittest};
use log::info;

use serde_json::{from_str, to_string};

#[test]
fn test_data_dat_codec_dat() {
    setup::log::configure();

    fix_data!(RawDataLength, 95, RawData, 96);

    let inp = RawData::new(dat::from_slice(b"BINBIN"));
    let fix_ser = to_bytes_unittest(&inp).unwrap();
    info!("fix_ser: {}", fix_ser);
    assert_eq!(&fix_ser[..], b"7BINBIN");
    let jsn_ser = to_string(&inp).unwrap();
    info!("jsn_ser: {}", jsn_ser);
    assert_eq!(jsn_ser, r#""QklOAUJJTg==""#);

    let fix_out = from_slice_unittest::<RawData<&dat>>(&fix_ser).unwrap();
    info!("fix_out: {:?}", fix_out);
    assert_eq!(inp, fix_out);

    let inp = RawData::new(Data::from_slice(b"BIN\x00BIN"));
    let fix_ser = to_bytes_unittest(&inp).unwrap();
    info!("fix_ser: {}", fix_ser);
    assert_eq!(&fix_ser[..], b"7BIN\x00BIN");

    let jsn_ser = to_string(&inp).unwrap();
    info!("jsn_ser: {}", jsn_ser);
    assert_eq!(jsn_ser, r#""QklOAEJJTg==""#);

    let fix_out = from_slice_unittest::<RawData<Data>>(&fix_ser).unwrap();
    info!("fix_out: {:?}", fix_out);
    assert_eq!(inp, fix_out);


}

#[test]
fn test_fix_string_str_ascii_asc() {
    setup::log::configure_level(log::LevelFilter::Info);

    fix_string!(Account, 1);

    let inp = Account::new("ABC".to_owned());

    let fix_ser = to_bytes_unittest(&inp).unwrap();
    info!("fix_ser: {}", fix_ser);
    assert_eq!(&fix_ser[..], b"ABC");
    let jsn_ser = to_string(&inp).unwrap();
    info!("jsn_ser: {}", jsn_ser);

    info!("Account<String>");
    let fix_out = from_slice_unittest::<Account<String>>(&fix_ser).unwrap();
    let jsn_out = serde_json::from_str::<Account<String>>(&jsn_ser).unwrap();
    info!("jsn_out: {:?}", jsn_out);
    assert_eq!(jsn_out, fix_out);
    assert_eq!(fix_out.value(), "ABC");

    info!("Account<&str>");
    let fix_out = from_slice_unittest::<Account<&str>>(&fix_ser).unwrap();
    info!("fix_out: {:?}", fix_out);
    let jsn_out = from_str::<Account<&str>>(&jsn_ser).unwrap();
    info!("jsn_out: {:?}", jsn_out);
    assert_eq!(jsn_out, fix_out);
    assert_eq!(fix_out.value(), "ABC");

    info!("Account<Ascii>");
    let fix_out = from_slice_unittest::<Account<Ascii>>(&fix_ser).unwrap();
    info!("fix_out: {:?}", fix_out);
    let jsn_out = from_str::<Account<Ascii>>(&jsn_ser).unwrap();
    info!("jsn_out: {:?}", jsn_out);
    assert_eq!(jsn_out, fix_out);
    assert_eq!(fix_out.value(), asc::try_from_str("ABC").unwrap());

    info!("Account<&asc>");
    let fix_out = from_slice_unittest::<Account<&asc>>(&fix_ser).unwrap();
    info!("fix_out: {:?}", fix_out);
    let jsn_out = from_str::<Account<&asc>>(&jsn_ser).unwrap();
    info!("jsn_out: {:?}", jsn_out);
    assert_eq!(jsn_out, fix_out);
    assert_eq!(fix_out.value(), asc::try_from_slice(b"ABC").unwrap());
}

#[test]
fn test_char_any() {
    setup::log::configure_level(log::LevelFilter::Info);

    fix_char_any!(IOIOthSvc, 24);

    let inp: IOIOthSvc<char> = 'A'.into();

    let fix_ser = to_bytes_unittest(&inp).unwrap();
    info!("fix_ser: {}", fix_ser);
    assert_eq!(&fix_ser[..], b"A");

    let jsn_ser = serde_json::to_string(&inp).unwrap();
    info!("jsn_ser: {}", jsn_ser);

    let fix_out = from_slice_unittest::<IOIOthSvc<aschar>>(&fix_ser).unwrap();
    info!("fix_out: {:?}", fix_out);
    let jsn_out = serde_json::from_str::<IOIOthSvc<aschar>>(&jsn_ser).unwrap();
    info!("jsn_out: {:?}", jsn_out);
    assert_eq!(jsn_out, fix_out);
}

#[test]
fn test_char_enum() {
    setup::log::configure_level(log::LevelFilter::Info);

    fix_ascii_char_enum!(AdvSide, 4, Buy: "B", Sell: "S", Trade: "T", Cross: "X" );
    let inp = AdvSide::Buy;

    let fix_ser = to_bytes_unittest(&inp).unwrap();
    info!("fix_ser: {}", fix_ser);
    assert_eq!(&fix_ser[..], b"B");

    let jsn_ser = serde_json::to_string(&inp).unwrap();
    info!("jsn_ser: {}", jsn_ser);

    let fix_out = from_slice_unittest::<AdvSide>(&fix_ser).unwrap();
    info!("fix_out: {:?}", fix_out);
    let jsn_out = serde_json::from_str::<AdvSide>(&jsn_ser).unwrap();
    info!("jsn_out: {:?}", jsn_out);
    assert_eq!(jsn_out, fix_out);
}

#[test]
fn test_fix_u8_fixed_len() {
    setup::log::configure();

    // 88::MAX 255 len 3
    fix_u8_fixed_length!(CheckSum, 10);
    let _inp = CheckSum::new(2);
    let inp: CheckSum = 2.into();

    let fix_ser = to_bytes_unittest(&inp).unwrap();
    info!("fix_ser: {}", fix_ser);
    assert_eq!(&fix_ser[..], b"002");

    let jsn_ser = to_string(&inp).unwrap();
    info!("jsn_ser: {}", jsn_ser);

    let fix_out = from_slice_unittest::<CheckSum>(&fix_ser).unwrap();
    let jsn_out = serde_json::from_str::<CheckSum>(&jsn_ser).unwrap();
    assert_eq!(jsn_out, fix_out);
    assert_eq!(jsn_out.value(), 2);
}
#[test]
fn test_fix_usize_fixed_len() {
    setup::log::configure_level(log::LevelFilter::Info);

    fix_usize_fixed_length!(BodyLength, 9);

    let inp: BodyLength = 10_usize.into();

    let fix_ser = to_bytes_unittest(&inp).unwrap();
    info!("fix_ser: {}", fix_ser);
    assert_eq!(&fix_ser[..], b"00000000000000000010");

    let jsn_ser = to_string(&inp).unwrap();
    info!("jsn_ser: {}", jsn_ser);

    let fix_out = from_slice_unittest::<BodyLength>(&fix_ser).unwrap();
    let jsn_out = serde_json::from_str::<BodyLength>(&jsn_ser).unwrap();
    info!("jsn_out: {:?}", jsn_out);
    assert_eq!(jsn_out, fix_out);
}

#[test]
fn test_fix_usize() {
    setup::log::configure();

    fix_usize!(BeginSeqNo, 7);
    let inp: BeginSeqNo = 2.into();

    let fix_ser = to_bytes_unittest(&inp).unwrap();
    info!("fix_ser: {}", fix_ser);
    assert_eq!(&fix_ser[..], b"2");

    let jsn_ser = to_string(&inp).unwrap();
    info!("jsn_ser: {}", jsn_ser);

    let fix_out = from_slice_unittest::<BeginSeqNo>(&fix_ser).unwrap();
    let jsn_out = serde_json::from_str::<BeginSeqNo>(&jsn_ser).unwrap();
    info!("jsn_out: {:?}", jsn_out);
    assert_eq!(jsn_out, fix_out);
}

#[test]
fn test_fix_bool() {
    setup::log::configure();

    fix_bool!(PossDupFlag, 43);

    let inp: PossDupFlag = false.into();

    let fix_ser = to_bytes_unittest(&inp).unwrap();
    info!("fix_ser: {}", fix_ser);
    assert_eq!(&fix_ser[..], b"N");
    let jsn_ser = serde_json::to_string(&inp).unwrap();
    info!("jsn_ser: {}", jsn_ser);

    let fix_out = from_slice_unittest::<PossDupFlag>(&fix_ser).unwrap();
    info!("fix_out: {:?}", fix_out);
    let jsn_out = serde_json::from_str::<PossDupFlag>(&jsn_ser).unwrap();
    info!("jsn_out: {:?}", jsn_out);
    assert_eq!(jsn_out, fix_out);

    let inp: PossDupFlag = true.into();

    let fix_ser = to_bytes_unittest(&inp).unwrap();
    info!("fix_ser: {}", fix_ser);
    assert_eq!(&fix_ser[..], b"Y");
}
