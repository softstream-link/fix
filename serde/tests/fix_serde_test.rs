use fix_serde::prelude::*;
use fix_serde::unittest::setup;
use serde::{Deserialize, Serialize};

use log::info;

#[test]
fn test_fix_string_deserialize() {
    setup::log::configure();
    let inp_fix_string = FixString::try_from(b"123").unwrap();

    let mut fix_ser = to_bytes(&inp_fix_string).unwrap();
    fix_ser.serialize_soh().unwrap();
    info!("fix_ser: {}", fix_ser);
    let json_ser = serde_json::to_string(&inp_fix_string).unwrap();
    info!("json_ser: {}", json_ser);

    let out_fix_string_fix: FixString = from_slice(&fix_ser.as_slice()).unwrap();
    info!("out_fix_string_fix: {:?}", out_fix_string_fix);
    let out_fix_string_json: FixString = serde_json::from_str(&json_ser).unwrap();
    info!("out_fix_string_json: {:?}", out_fix_string_json);
    assert_eq!(out_fix_string_fix, out_fix_string_json);
    assert_eq!(out_fix_string_fix, inp_fix_string);
}

#[test]
fn test_usize_deserialize() {
    setup::log::configure();
    let inp_usize = 365_usize;

    let mut fix_ser = to_bytes(&inp_usize).unwrap();
    fix_ser.serialize_soh().unwrap();
    info!("fix_ser: {}", fix_ser);
    let json_ser = serde_json::to_string(&inp_usize).unwrap();
    info!("json_ser: {}", json_ser);

    let out_usize_fix: usize = from_slice(&fix_ser).unwrap();
    info!("out_usize_fix: {:?}", out_usize_fix);
    let out_usize_json: usize = serde_json::from_str(&json_ser).unwrap();
    info!("out_usize_json: {:?}", out_usize_json);
    assert_eq!(out_usize_fix, out_usize_json);
    assert_eq!(out_usize_fix, inp_usize);
}

#[test]
fn test_enum() {
    setup::log::configure();

    // <field number='4' name='AdvSide' type='CHAR'>
    //   <value enum='B' description='BUY' />
    //   <value enum='S' description='SELL' />
    //   <value enum='X' description='CROSS' />
    //   <value enum='T' description='TRADE' />
    // </field>

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    enum AdvSide {
        #[serde(rename = "B")]
        #[serde(alias = "BUY")]
        Buy,
        #[serde(rename = "S")]
        #[serde(alias = "SELL")]
        Sell,
        #[serde(rename = "X")]
        #[serde(alias = "CROSS")]
        Cross,
        #[serde(rename = "T")]
        #[serde(alias = "TRADE")]
        Trade,
    }
    let inp_enum = AdvSide::Buy;
    info!("inp_enum: {:?}", inp_enum);

    let mut fix_ser = to_bytes(&inp_enum).unwrap();
    fix_ser.serialize_soh().unwrap();
    info!("fix_ser: {}", fix_ser);
    let json_ser = serde_json::to_string(&inp_enum).unwrap();
    info!("json_ser: {}", json_ser);

    let out_enum_fix: AdvSide = from_slice(&fix_ser).unwrap();
    info!("out_enum_fix: {:?}", out_enum_fix);
    let out_enum_json: AdvSide = serde_json::from_str(&json_ser).unwrap();
    info!("out_enum_json: {:?}", out_enum_json);
    assert_eq!(out_enum_fix, out_enum_json);
}

#[test]
fn test_msg_deserialize() {
    setup::log::configure();

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Account<T: FixStringLike>(T);
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct AdvId<T: FixStringLike>(T);
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct BeginSeqNo(usize);

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Msg<T: FixStringLike> {
        #[serde(rename = "1")]
        #[serde(alias = "Account")]
        account: Account<T>,
        #[serde(rename = "2")]
        #[serde(alias = "AdvId")]
        adv_id: AdvId<T>,
        #[serde(rename = "7")]
        #[serde(alias = "BeginSeqNo")]
        begin_seq_no: BeginSeqNo,
    }

    let account = Account(b"ACC".try_into().unwrap());
    let adv_id = AdvId(b"ADB".try_into().unwrap());
    let begin_seq_no = BeginSeqNo(100);
    let inp_msg = Msg::<FixString> {
        account,
        adv_id,
        begin_seq_no,
    };
    info!("inp_msg: {:?}", inp_msg);

    let fix_ser = to_bytes(&inp_msg).unwrap();
    info!("fix_ser: {}", fix_ser);
    let json_ser = serde_json::to_string(&inp_msg).unwrap();
    info!("json_ser: {}", json_ser);
    let out_msg_fix: Msg<FixString> = from_slice(&fix_ser).unwrap();
    info!("out_msg_fix: {:?}", out_msg_fix);
    let out_msg_json: Msg<FixString> = serde_json::from_str(&json_ser).unwrap();
    info!("out_msg_json: {:?}", out_msg_json);
    assert_eq!(out_msg_fix, out_msg_json);
}
