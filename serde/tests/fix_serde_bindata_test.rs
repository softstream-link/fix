use std::{
    borrow::Borrow,
    fmt::{Debug, Display},
};

use fix_model_core::prelude::*;
use fix_model_test::unittest::setup;
use fix_serde::unittest::{from_slice_unittest, to_bytes_unittest};
use serde::{Deserialize, Serialize};

use log::info;

#[test]
fn test_dat_codec() {
    setup::log::configure_level(log::LevelFilter::Info);
    let mut bin = Vec::new();
    for i in 0..255_u8 {
        bin.push(i)
    }
    let data = Data::from(bin);

    let inp_dat_codec = dat_codec::from_slice(&data);
    let fix_ser_dat_codec = to_bytes_unittest(&inp_dat_codec).unwrap();
    info!("fix_ser_dat_codec: {}", fix_ser_dat_codec);
    let json_ser_dat_codec = serde_json::to_string(&inp_dat_codec).unwrap();
    info!("json_ser_dat_codec: {}", json_ser_dat_codec);

    let out_fix_dat_codec: dat_codec = from_slice_unittest(&fix_ser_dat_codec).unwrap();
    info!("out_fix_dat_codec: {}", out_fix_dat_codec);

    let mut out_json_dat_codec: dat_codec = serde_json::from_str(&json_ser_dat_codec).unwrap();
    out_json_dat_codec.decode().unwrap();
    info!("out_json_dat_codec: {}", out_json_dat_codec);
    assert_eq!(inp_dat_codec, out_json_dat_codec);
}

// define with struct instead of stand alone
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Msg<D> {
    #[serde(rename = "95")]
    data: D,
}
impl<D: Display> Display for Msg<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

#[test]
fn test_data_and_dat() {
    setup::log::configure_level(log::LevelFilter::Info);
    let mut bin = Vec::new();
    for i in 0..255_u8 {
        bin.push(i)
    }
    let data = Data::from(bin);

    {
        // Data
        let inp_msg_data = Msg { data: data.clone() };
        info!("inp_msg_data: {}", inp_msg_data);

        let fix_ser_data = to_bytes_unittest(&inp_msg_data).unwrap();
        info!("fix_ser_data: {}", fix_ser_data);

        let json_ser_data = serde_json::to_string(&inp_msg_data).unwrap();
        info!("json_ser_data: {}", json_ser_data);

        let out_json_data: Msg<Data> = serde_json::from_str(&json_ser_data).unwrap();
        info!("out_json_data: {}", out_json_data);
        assert_eq!(inp_msg_data, out_json_data);

        let out_fix_data: Msg<Data> = from_slice_unittest(&fix_ser_data).unwrap();
        info!("out_fix_data: {}", out_fix_data);
        assert_eq!(inp_msg_data, out_fix_data);
    }

    {
        // dat
        let data: &dat = data.borrow();
        let imp_msg_dat = Msg { data };
        info!("imp_msg_dat: {}", imp_msg_dat);

        let fix_ser_dat = to_bytes_unittest(&imp_msg_dat).unwrap();
        info!("fix_ser_dat: {}", fix_ser_dat);

        let json_ser_dat = serde_json::to_string(&imp_msg_dat).unwrap();
        info!("json_ser_dat: {}", json_ser_dat);

        let out_fix_dat: Msg<&dat> = from_slice_unittest(&fix_ser_dat).unwrap();
        info!("out_fix_dat: {}", out_fix_dat);
        assert_eq!(imp_msg_dat, out_fix_dat);

        let out_json_dat: Msg<&dat> = serde_json::from_str(&json_ser_dat).unwrap();
        info!("out_json_dat: {}", out_json_dat);
        assert_eq!(imp_msg_dat.data.as_slice(), out_json_dat.data.decode_base64().unwrap());
    }

    {
        // dat_lazybase64
        let data = dat_codec::from_slice(data.as_slice());
        let imp_msg_dat_maybe = Msg { data };
        info!("imp_msg_dat_maybe: {}", imp_msg_dat_maybe);

        let fix_ser_dat_maybe = to_bytes_unittest(&imp_msg_dat_maybe).unwrap();
        info!("fix_ser_dat_maybe: {}", fix_ser_dat_maybe);

        let json_ser_dat_maybe = serde_json::to_string(&imp_msg_dat_maybe).unwrap();
        info!("json_ser_dat: {}", json_ser_dat_maybe);

        let out_fix_dat_maybe: Msg<dat_codec> = from_slice_unittest(&fix_ser_dat_maybe).unwrap();
        info!("out_fix_dat_maybe: {}", out_fix_dat_maybe);

        assert_eq!(imp_msg_dat_maybe, out_fix_dat_maybe);

        let mut out_json_dat_maybe: Msg<dat_codec> = serde_json::from_str(&json_ser_dat_maybe).unwrap();
        info!("out_json_dat_maybe: {}", out_json_dat_maybe);
        assert_ne!(imp_msg_dat_maybe, out_json_dat_maybe);
        out_json_dat_maybe.data.decode().unwrap();
        info!("out_json_dat_maybe: {}", out_json_dat_maybe);
        assert_eq!(imp_msg_dat_maybe, out_json_dat_maybe);
    }
}
