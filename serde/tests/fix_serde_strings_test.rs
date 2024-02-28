use fix_model_core::prelude::*;
use fix_model_test::unittest::setup;
use fix_serde::unittest::{from_slice_unittest, to_bytes_unittest};
use log::info;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

fix_model_generator::prelude::fix_string!(Account, 1); // <field number='1' name='Account' type='STRING' />
fix_model_generator::prelude::fix_string!(AdvId, 2); // <field number='2' name='AdvId' type='STRING' />
fix_model_generator::prelude::fix_usize!(BeginSeqNo, 7); // <field number='7' name='BeginSeqNo' type='INT' />
fix_model_generator::prelude::fix_ascii_char_enum!(AdvSide, 4, Buy:"B", Sell:"S", Trade:"T", Cross:"X" ); // <field number='4' name='AdvSide' type='CHAR'> // <value enum='B' description='BUY' /> // <value enum='S' description='SELL' /> // <value enum='T' description='TRADE' /> // <value enum='X' description='CROSS' /> // </field>

fix_model_generator::prelude::fix_char_any!(IOIOthSvc, 24); // <field number='24' name='IOIOthSvc' type='CHAR' />

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Msg<S, C> {
    #[serde(rename = "1")]
    #[serde(alias = "Account")]
    account: Account<S>,
    #[serde(rename = "2")]
    #[serde(alias = "AdvId")]
    adv_id: AdvId<S>,
    #[serde(rename = "7")]
    #[serde(alias = "BeginSeqNo")]
    begin_seq_no: BeginSeqNo,
    #[serde(rename = "4")]
    #[serde(alias = "AdvSide")]
    adv_side: AdvSide,

    #[serde(rename = "24")]
    #[serde(alias = "IOIOthSvc")]
    ioioth_svc: IOIOthSvc<C>,
}

impl<S, C> std::default::Default for Msg<S, C>
where
    Account<S>: Default,
    AdvId<S>: Default,
    IOIOthSvc<C>: Default,
{
    fn default() -> Self {
        Self {
            account: Default::default(),
            adv_id: Default::default(),
            begin_seq_no: Default::default(),
            adv_side: Default::default(),
            ioioth_svc: Default::default(),
        }
    }
}
#[test]
fn test_stand_alone_enum_char() {
    setup::log::configure();

    let inp_enum = AdvSide::Buy;
    info!("inp_enum: {:?}", inp_enum);

    let fix_ser = to_bytes_unittest(&inp_enum).unwrap();
    info!("fix_ser: {}", fix_ser);
    let json_ser = serde_json::to_string(&inp_enum).unwrap();
    info!("json_ser: {}", json_ser);

    let out_enum_fix: AdvSide = from_slice_unittest(&fix_ser).unwrap();
    info!("out_enum_fix: {:?}", out_enum_fix);
    let out_enum_json: AdvSide = serde_json::from_str(&json_ser).unwrap();
    info!("out_enum_json: {:?}", out_enum_json);
    assert_eq!(out_enum_fix, out_enum_json);
}

#[test]
fn test_stand_alone_ascii_and_asc() {
    setup::log::configure();
    // Ascii
    let inp_fix_ascii = Ascii::try_from(b"123").unwrap();

    let fix_ser = to_bytes_unittest(&inp_fix_ascii).unwrap();
    info!("fix_ser: {}", fix_ser);
    let json_ser = serde_json::to_string(&inp_fix_ascii).unwrap();
    info!("json_ser: {}", json_ser);

    let out_fix_ascii: Ascii = from_slice_unittest(&fix_ser.as_slice()).unwrap();
    info!("out_fix_ascii: {:?}", out_fix_ascii);
    let out_json_ascii: Ascii = serde_json::from_str(&json_ser).unwrap();
    info!("out_json_ascii: {:?}", out_json_ascii);
    assert_eq!(out_fix_ascii, out_json_ascii);
    assert_eq!(out_fix_ascii, inp_fix_ascii);

    // asc
    let inp_fix_asc = asc::try_from_slice(b"123").unwrap();

    let fix_ser = to_bytes_unittest(&inp_fix_asc).unwrap();
    info!("fix_ser: {}", fix_ser);
    let json_ser = serde_json::to_string(&inp_fix_asc).unwrap();
    info!("json_ser: {}", json_ser);

    let out_fix_asc: &asc = from_slice_unittest(&fix_ser).unwrap();
    info!("out_fix_asc: {:?}", out_fix_asc);
    let out_json_asc: &asc = serde_json::from_str(&json_ser).unwrap();
    info!("out_json_asc: {:?}", out_json_asc);
    assert_eq!(out_fix_asc, out_json_asc);
    assert_eq!(out_fix_asc, inp_fix_asc);
}

#[test]
fn test_msg_owned_ascii_and_utf8() {
    setup::log::configure_level(log::LevelFilter::Info);
    let x = Msg::<Ascii, aschar>::default();
    info!("x: {:?}", x);
    {
        // Ascii String
        let inp_msg_ascii = Msg::<Ascii, aschar> {
            account: "ACC".try_into().unwrap(),
            adv_id: "ADV".try_into().unwrap(),
            begin_seq_no: BeginSeqNo(100),
            adv_side: AdvSide::Buy,
            ioioth_svc: 'I'.try_into().unwrap(),
        };
        info!("inp_msg_ascii: {:?}", inp_msg_ascii);

        let fix_ser_ascii = to_bytes_unittest(&inp_msg_ascii).unwrap();
        info!("fix_ser_ascii: {}", fix_ser_ascii);
        let json_ser_ascii = serde_json::to_string(&inp_msg_ascii).unwrap();
        info!("json_ser_ascii: {}", json_ser_ascii);

        let out_msg_fix_ascii: Msg<Ascii, aschar> = from_slice_unittest(&fix_ser_ascii).unwrap();
        info!("out_msg_fix_ascii: {:?}", out_msg_fix_ascii);

        let out_msg_json_ascii: Msg<Ascii, aschar> = serde_json::from_str(&json_ser_ascii).unwrap();
        info!("out_msg_json_ascii: {:?}", out_msg_json_ascii);
        assert_eq!(out_msg_fix_ascii, out_msg_json_ascii);
    }

    {
        // Utf8 String
        let inp_msg_string = Msg::<String, char> {
            account: "ACC💖".to_owned().into(),
            adv_id: "ADV💖".to_owned().into(),
            begin_seq_no: BeginSeqNo(100),
            adv_side: AdvSide::Buy,
            ioioth_svc: 'I'.into(),
        };

        info!("inp_msg_string: {:?}", inp_msg_string);

        let fix_ser_string = to_bytes_unittest(&inp_msg_string).unwrap();
        info!("fix_ser_string: {}", fix_ser_string);
        let json_ser_string = serde_json::to_string(&inp_msg_string).unwrap();
        info!("json_ser_string: {}", json_ser_string);
        let out_msg_fix_string: Msg<String, char> = from_slice_unittest(&fix_ser_string).unwrap();
        info!("out_msg_fix_string: {:?}", out_msg_fix_string);
        let out_msg_json_string: Msg<String, char> = serde_json::from_str(&json_ser_string).unwrap();
        info!("out_msg_json_string: {:?}", out_msg_json_string);
        assert_eq!(out_msg_fix_string, out_msg_json_string);
    }
}

#[test]
fn test_msg_borrowed_ascii_and_utf8() {
    setup::log::configure_level(log::LevelFilter::Info);

    {
        // Ascii slice &asc
        let inp_msg = Msg::<&asc, aschar> {
            account: Account(b"ACC".try_into().unwrap()),
            adv_id: AdvId(b"ADV".try_into().unwrap()),
            begin_seq_no: BeginSeqNo(100),
            adv_side: AdvSide::Buy,
            ioioth_svc: IOIOthSvc(b'I'.try_into().unwrap()),
        };
        info!("inp_msg: {:?}", inp_msg);

        let fix_ser = to_bytes_unittest(&inp_msg).unwrap();
        info!("fix_ser: {}", fix_ser);
        let json_ser = serde_json::to_string(&inp_msg).unwrap();
        info!("json_ser: {}", json_ser);

        let out_msg_fix: Msg<&asc, aschar> = from_slice_unittest(&fix_ser).unwrap();
        // drop(fix_ser); // fix_ser is borrowed by from_slice_default hence can't be dropped
        info!("out_msg_fix: {:?}", out_msg_fix);
        let out_msg_json: Msg<&asc, aschar> = serde_json::from_str(&json_ser).unwrap();
        info!("out_msg_json: {:?}", out_msg_json);
        assert_eq!(out_msg_fix, out_msg_json);
    }
    {
        // Utf8 slice &str
        let inp_msg = Msg::<&str, char> {
            account: Account("ACC💖"),
            adv_id: AdvId("ADV💖"),
            begin_seq_no: BeginSeqNo(100),
            adv_side: AdvSide::Buy,
            ioioth_svc: IOIOthSvc(b'I'.try_into().unwrap()),
        };

        info!("inp_msg: {:?}", inp_msg);

        let fix_ser = to_bytes_unittest(&inp_msg).unwrap();
        info!("fix_ser: {}", fix_ser);
        let json_ser = serde_json::to_string(&inp_msg).unwrap();
        info!("json_ser: {}", json_ser);
        let out_msg_fix: Msg<&str, char> = from_slice_unittest(&fix_ser).unwrap();
        info!("out_msg_fix: {:?}", out_msg_fix);
        let out_msg_json: Msg<&str, char> = serde_json::from_str(&json_ser).unwrap();
        info!("out_msg_json: {:?}", out_msg_json);
        assert_eq!(out_msg_fix, out_msg_json);
    }
}
