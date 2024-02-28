use fix_model_core::{
    prelude::{asc, Ascii},
    types::{asciichar::aschar, dat::dat, data::Data},
};
use fix_model_generator::prelude::{fix_ascii_char_enum, fix_bool, fix_char_any, fix_data, fix_string, fix_usize};
use fix_model_test::unittest::setup;
use fix_serde::unittest::{from_slice_unittest, to_bytes_unittest};
use log::info;

use serde_json::{from_str, to_string};

#[test]
fn test_data_dat_codec_dat() {
    setup::log::configure();

    fix_data!(RawDataLength, 95, RawData, 96);

    let _inp = RawData::new(dat::from_slice(b"\x00BIN"));
    let _inp = RawData::new(dat::from_slice(&vec![1, 2]));
    let _inp: RawData<&dat> = b"BIN".as_slice().into();
    let _inp: RawData<&dat> = b"BIN".into();

    let _inp = RawData::new(Data::from_slice(b"\x00BIN"));
    let _inp = RawData::new(Data::from_slice(&vec![1, 2]));
    let _inp = RawData::new(Data::from_vec(vec![1, 2]));
    let _inp: RawData<Data> = vec![1, 2].into();
    let _inp: RawData<Data> = b"BIN".into();

    info!("RawData::<&dat>::default: '{}'", RawData::<&dat>::default());
    info!("RawData::<Data>::default: '{}'", RawData::<Data>::default());

    let inp = RawData::new(Data::from_slice(b"\x00BIN"));

    let debug_ = format!("{:?}", inp);
    info!("debug_: {}", debug_);
    assert_eq!(debug_, r#"RawData:96([00, 42, 49, 4E])"#);
    let debug_p = format!("{:+?}", inp);
    info!("debug_p: {}", debug_p);
    assert_eq!(debug_p, r#"96([00, 42, 49, 4E])"#);
    let debug_m = format!("{:-?}", inp);
    info!("debug_m: {}", debug_m);
    assert_eq!(debug_m, r#"RawData([00, 42, 49, 4E])"#);

    let display_ = format!("{}", inp);
    info!("display_: {}", display_);
    assert_eq!(display_, "\0BIN");
    let display_p = format!("{:+}", inp);
    info!("display_p: {}", display_p);
    assert_eq!(display_p, "95=4|96=\0BIN");
    let display_m = format!("{:-}", inp);
    info!("display_m: {}", display_m);
    assert_eq!(display_m, "RawDataLength=4|RawData=\0BIN");
}

#[test]
fn test_fix_string_str_ascii_asc() {
    setup::log::configure();

    fix_string!(Account, 1);

    let _inp = Account::new("ABC");
    let _inp: Account<&str> = "ABC".into();
    let _inp: Account<&str> = Default::default();

    let _inp = Account::new("ABC".to_owned());
    let _inp: Account<String> = Account::new("ABC".into());
    let _inp: Account<String> = "ABC".to_owned().into();
    let _inp: Account<String> = "ABC".into();
    let _inp: Account<String> = Default::default();

    let _inp = Account::new(asc::try_from_str("ABC").unwrap());
    let _inp: Account<&asc> = Account::new("ABC".try_into().unwrap());
    let _inp: Account<&asc> = "ABC".try_into().unwrap();
    let _inp: Account<&asc> = b"ABC".try_into().unwrap();
    let _inp: Account<&asc> = Default::default();

    let _inp = Account::new(Ascii::try_from_str("ABC").unwrap());
    let _inp: Account<Ascii> = "ABC".try_into().unwrap();
    let _inp: Account<Ascii> = b"ABC".try_into().unwrap();
    let _inp: Account<Ascii> = Default::default();
    info!("Account<Ascii>::default '{}'", Account::<Ascii>::default());

    let inp = Account::new("ABC".to_owned());
    let debug_ = format!("{:?}", inp);
    info!("debug_: {}", debug_);
    assert_eq!(debug_, r#"Account:1("ABC")"#);
    let debug_p = format!("{:+?}", inp);
    info!("debug_p: {}", debug_p);
    assert_eq!(debug_p, r#"1("ABC")"#);
    let debug_m = format!("{:-?}", inp);
    info!("debug_m: {}", debug_m);
    assert_eq!(debug_m, r#"Account("ABC")"#);

    let display_ = format!("{}", inp);
    info!("display_: {}", display_);
    assert_eq!(display_, "ABC");
    let display_p = format!("{:+}", inp);
    info!("display_p: {}", display_p);
    assert_eq!(display_p, "1=ABC");
    let display_m = format!("{:-}", inp);
    info!("display_m: {}", display_m);
    assert_eq!(display_m, "Account=ABC");

    let fix_ser = to_bytes_unittest(&inp).unwrap();
    info!("fix_ser: {}", fix_ser);
    let jsn_ser = to_string(&inp).unwrap();
    info!("jsn_ser: {}", jsn_ser);

    info!("Account<String>");
    let fix_out = from_slice_unittest::<Account<String>>(&fix_ser).unwrap();
    let jsn_out = serde_json::from_str::<Account<String>>(&jsn_ser).unwrap();
    info!("jsn_out: {:?}", jsn_out);
    assert_eq!(jsn_out, fix_out);
    assert_eq!(fix_out.value(), "ABC");
    let _own = fix_out.to_owned();
    let _borrow = fix_out.borrow();
    let _ref = fix_out.as_ref();

    info!("Account<&str>");
    let fix_out = from_slice_unittest::<Account<&str>>(&fix_ser).unwrap();
    info!("fix_out: {:?}", fix_out);
    let jsn_out = from_str::<Account<&str>>(&jsn_ser).unwrap();
    info!("jsn_out: {:?}", jsn_out);
    assert_eq!(jsn_out, fix_out);
    assert_eq!(fix_out.value(), "ABC");
    let _own = fix_out.to_owned();
    let _borrow = fix_out.borrow();
    let _ref = fix_out.as_ref();

    info!("Account<Ascii>");
    let fix_out = from_slice_unittest::<Account<Ascii>>(&fix_ser).unwrap();
    info!("fix_out: {:?}", fix_out);
    let jsn_out = from_str::<Account<Ascii>>(&jsn_ser).unwrap();
    info!("jsn_out: {:?}", jsn_out);
    assert_eq!(jsn_out, fix_out);
    assert_eq!(fix_out.value(), asc::try_from_str("ABC").unwrap());
    let _own = fix_out.to_owned();
    let _borrow = fix_out.borrow();
    let _ref = fix_out.as_ref();

    info!("Account<&asc>");
    let fix_out = from_slice_unittest::<Account<&asc>>(&fix_ser).unwrap();
    info!("fix_out: {:?}", fix_out);
    let jsn_out = from_str::<Account<&asc>>(&jsn_ser).unwrap();
    info!("jsn_out: {:?}", jsn_out);
    assert_eq!(jsn_out, fix_out);
    assert_eq!(fix_out.value(), asc::try_from_slice(b"ABC").unwrap());
    let _own = fix_out.to_owned();
    let _borrow = fix_out.borrow();
    let _ref = fix_out.as_ref();
}

#[test]
fn test_char_any() {
    setup::log::configure();

    fix_char_any!(IOIOthSvc, 24);

    let _inp = IOIOthSvc::new('A');
    let _inp = IOIOthSvc::<char>::new('A');
    let _inp: IOIOthSvc<char> = 'A'.into();
    let _inp: IOIOthSvc<char> = Default::default();

    let _inp = IOIOthSvc::new(aschar::try_from('A').unwrap());
    let _inp = IOIOthSvc::<aschar>::new('A'.try_into().unwrap());
    let _inp: IOIOthSvc<aschar> = 'A'.try_into().unwrap();
    let _inp: IOIOthSvc<aschar> = b'A'.try_into().unwrap();
    let _inp: IOIOthSvc<aschar> = Default::default();

    let inp: IOIOthSvc<char> = 'A'.into();
    assert_eq!(inp.value(), 'A');

    let debug_ = format!("{:?}", inp);
    info!("debug_: {}", debug_);
    assert_eq!(debug_, "IOIOthSvc:24('A')");

    let debug_p = format!("{:+?}", inp);
    info!("debug_p: {}", debug_p);
    assert_eq!(debug_p, "24('A')");

    let debug_m = format!("{:-?}", inp);
    info!("debug_m: {}", debug_m);
    assert_eq!(debug_m, "IOIOthSvc('A')");

    let display_ = format!("{}", inp);
    info!("display_: {}", display_);
    assert_eq!(display_, "A");

    let display_p = format!("{:+}", inp);
    info!("display_p: {}", display_p);
    assert_eq!(display_p, "24=A");

    let display_m = format!("{:-}", inp);
    info!("display_m: {}", display_m);
    assert_eq!(display_m, "IOIOthSvc=A");

    let fix_ser = to_bytes_unittest(&inp).unwrap();
    info!("fix_ser: {}", fix_ser);
    let jsn_ser = serde_json::to_string(&inp).unwrap();
    info!("jsn_ser: {}", jsn_ser);

    let fix_out = from_slice_unittest::<IOIOthSvc<aschar>>(&fix_ser).unwrap();
    info!("fix_out: {:?}", fix_out);
    let jsn_out = serde_json::from_str::<IOIOthSvc<aschar>>(&jsn_ser).unwrap();
    info!("jsn_out: {:?}", jsn_out);
    assert_eq!(jsn_out, fix_out);
}


// #[test]
// fn test_stringify_in_macro(){
//     macro_rules! create_enum {
//         ($NAME:ident, $($VARIANT:tt : $VALUE:literal),*) => {
//             #[derive(Debug, serde::Serialize)]
//             pub enum $NAME {
//                 $(
//                     #[serde(rename = $VALUE)]
//                     #[serde(alias = "stringify!($VARIANT)")] // how to stringify the variant for serde? is it possible?
//                     $VARIANT
//                 ),*
//             }
//         };
//     }
//     create_enum!(MyEnum, One: "1", Two: "2", Three: "3");
//     let x = MyEnum::One;
//     println!("{:?}", x);
// }

#[test]
fn test_char_enum() {
    setup::log::configure();

    fix_ascii_char_enum!(AdvSide, 4, Buy: "B", Sell: "S", Trade: "T", Cross: "X" );
    let inp = AdvSide::Buy;
    assert_eq!(inp.value(), 'B');

    let debug_ = format!("{:?}", inp);
    info!("debug_: {}", debug_);
    assert_eq!(debug_, "AdvSide:4(Buy:'B')");

    let debug_p = format!("{:+?}", inp);
    info!("debug_p: {}", debug_p);
    assert_eq!(debug_p, "4('B')");

    let debug_m = format!("{:-?}", inp);
    info!("debug_m: {}", debug_m);
    assert_eq!(debug_m, "AdvSide('Buy')");

    let display_ = format!("{}", inp);
    info!("display_: {}", display_);
    assert_eq!(display_, "B");

    let display_p = format!("{:+}", inp);
    info!("display_p: {}", display_p);
    assert_eq!(display_p, "4=B");

    let display_m = format!("{:-}", inp);
    info!("display_m: {}", display_m);
    assert_eq!(display_m, "AdvSide=B");

    let fix_ser = to_bytes_unittest(&inp).unwrap();
    info!("fix_ser: {}", fix_ser);
    let jsn_ser = serde_json::to_string(&inp).unwrap();
    info!("jsn_ser: {}", jsn_ser);

    let fix_out = from_slice_unittest::<AdvSide>(&fix_ser).unwrap();
    info!("fix_out: {:?}", fix_out);
    let jsn_out = serde_json::from_str::<AdvSide>(&jsn_ser).unwrap();
    info!("jsn_out: {:?}", jsn_out);
    assert_eq!(jsn_out, fix_out);
}

#[test]
fn test_fix_int() {
    setup::log::configure();

    fix_usize!(BeginSeqNo, 7);
    let _inp = BeginSeqNo::new(2);
    let inp: BeginSeqNo = 2.into();
    assert_eq!(inp.value(), 2);

    let debug_ = format!("{:?}", inp);
    info!("debug_: {}", debug_);
    assert_eq!(debug_, "BeginSeqNo:7(2)");

    let debug_p = format!("{:+?}", inp);
    info!("debug_p: {}", debug_p);
    assert_eq!(debug_p, "7(2)");

    let debug_m = format!("{:-?}", inp);
    info!("debug_m: {}", debug_m);
    assert_eq!(debug_m, "BeginSeqNo(2)");

    let display_ = format!("{}", inp);
    info!("display_: {}", display_);
    assert_eq!(display_, "2");

    let display_p = format!("{:+}", inp);
    info!("display_p: {}", display_p);
    assert_eq!(display_p, "7=2");

    let display_m = format!("{:-}", inp);
    info!("display_m: {}", display_m);
    assert_eq!(display_m, "BeginSeqNo=2");

    let fix_ser = to_bytes_unittest(&inp).unwrap();
    info!("fix_ser: {}", fix_ser);
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

    let _inp = PossDupFlag::new(false);
    let inp: PossDupFlag = false.into();
    assert_eq!(inp.value(), false);

    let debug_ = format!("{:?}", inp);
    info!("debug_: {}", debug_);
    assert_eq!(debug_, "PossDupFlag:43(false)");

    let debug_p = format!("{:+?}", inp);
    info!("debug_p: {}", debug_p);
    assert_eq!(debug_p, "43(false)");

    let debug_m = format!("{:-?}", inp);
    info!("debug_m: {}", debug_m);
    assert_eq!(debug_m, "PossDupFlag(false)");

    let display_ = format!("{}", inp);
    info!("display_: {}", display_);
    assert_eq!(display_, "N");

    let display_p = format!("{:+}", inp);
    info!("display_p: {}", display_p);
    assert_eq!(display_p, "43=N");

    let display_m = format!("{:-}", inp);
    info!("display_m: {}", display_m);
    assert_eq!(display_m, "PossDupFlag=N");

    let fix_ser = to_bytes_unittest(&inp).unwrap();
    info!("fix_ser: {}", fix_ser);
    let jsn_ser = serde_json::to_string(&inp).unwrap();
    info!("jsn_ser: {}", jsn_ser);

    let fix_out = from_slice_unittest::<PossDupFlag>(&fix_ser).unwrap();
    info!("fix_out: {:?}", fix_out);
    let jsn_out = serde_json::from_str::<PossDupFlag>(&jsn_ser).unwrap();
    info!("jsn_out: {:?}", jsn_out);
    assert_eq!(jsn_out, fix_out);
}

// #[test]
// fn test_logon() {
//     setup::log::configure();

//     fix_string!(BeginString, 8);
//     fix_usize!(BodyLength, 9);
//     fix_string!(MsgType, 35);
//     fix_string!(SenderCompID, 49);
//     fix_string!(TargetCompID, 56);
//     fix_string!(OnBehalfOfCompID, 115);
//     // fix_string!(DeliverToCompID, 128);
//     // fix_int!(MsgSeqNum, 34);
//     // fix_string!(SenderSubID, 50);
//     // fix_string!(SenderLocationID, 142);
//     // fix_string!(TargetSubID, 57);
//     // fix_string!(TargetLocationID, 144);
//     // fix_string!(OnBehalfOfSubID, 116);
//     // fix_string!(OnBehalfOfLocationID, 144);
//     // fix_string!(DeliverToSubID, 129);
//     // fix_string!(DeliverToLocationID, 145);
//     // fix_string!(PossDupFlag, 43);
//     // fix_string!(PossResend, 97);
//     // fix_string!(SendingTime, 52);
//     // fix_string!(OrigSendingTime, 122);
//     // fix_string!(XmlDataLen, 212);
//     // fix_string!(XmlData, 213);
//     // fix_string!(MessageEncoding, 347);
//     // fix_string!(LastMsgSeqNumProcessed, 369);
//     // fix_string!(OnBehalfOfSendingTime, 370);
//     // fix_int!(EncryptMethod, 98);
//     // fix_int!(HeartBtInt, 108);
//     // fix_string!(RawDataLength, 95);
//     // fix_string!(RawData, 96);
//     // fix_string!(ResetSeqNumFlag, 141);
//     // fix_string!(MaxMessageSize, 383);
//     // fix_string!(NoMsgTypes, 384);
//     // fix_string!(RefMsgType, 372);
//     // fix_string!(MsgDirection, 385);

//     fix_message!(
//         Logon<S: AsRef<str>>,
//         begin_string TAG "8" ALIAS "BeginString": BeginString<S>,
//         body_length TAG "9" ALIAS "BodyLength": BodyLength,
//         msg_type TAG "35" ALIAS "MsgType": MsgType<S>,
//         sender_comp_id TAG "49" ALIAS "SenderCompID": SenderCompID<S>,
//         target_comp_id TAG "56" ALIAS "TargetCompID": TargetCompID<S>,
//         on_behalf_of_comp_id TAG "115" ALIAS "OnBehalfOfCompID": Option<OnBehalfOfCompID<S>>
//     );
//     let begin_string = BeginString::new("FIX.4.4");
//     let logon = Logon::<&str> {
//         // begin_string: "FIX.4.4".to_owned().into(),
//         begin_string,
//         body_length: 100.into(),
//         msg_type: "A".into(),
//         sender_comp_id: "SENDER".into(),
//         target_comp_id: "TARGET".into(),
//         on_behalf_of_comp_id: None,
//     };

//     info!("logon:? {:?}", logon);
//     info!("logon:-? {:-?}", logon);
//     info!("logon:+? {:+?}", logon);
//     // info!("logon: {}", logon);
//     //     // #[derive(serde::Serialize, Debug, PartialEq, Clone)]
//     //     // #[serde(rename_all = "PascalCase")]
//     //     // struct Message<S: StringValue> {
//     //     //     account: Account<S>,
//     //     //     adv_side: AdvSide,
//     //     // }
// }

// #[test]
// fn test_serialize_msg() -> serde_json::Result<()> {
//     fix_string!(Account, 1);

//     fix_char!(AdvSide, 4);

//     // fix_message!(TestMessage, account: Account<S>, adv_side: AdvSide);
//     fix_message!(Message<S: StringValue>, account: Account<S>, adv_side: AdvSide);
//     // #[derive(serde::Serialize, Debug, PartialEq, Clone)]
//     // #[serde(rename_all = "PascalCase")]
//     // struct Message<S: StringValue> {
//     //     account: Account<S>,
//     //     adv_side: AdvSide,
//     // }
//     // impl<S: StringValue> Message<S> {
//     //     pub fn to_owned(&self) -> Message<String> {
//     //         Message {
//     //             account: self.account.to_owned(),
//     //             adv_side: self.adv_side.to_owned(),
//     //         }
//     //     }
//     // }
//     // impl<S: StringValue> Serialize for Message<S> {
//     //     fn serialize(&self, ser: &mut impl Serializer) {
//     //         self.account.serialize(ser);
//     //         self.adv_side.serialize(ser);
//     //     }
//     // }

//     setup::log::configure();
//     let mut ser = HeapSerializer::with_capacity(1024);

//     let m1 = Message {
//         account: "STR".into(),
//         adv_side: 'C'.into(),
//     };
//     info!("t: {:?}", m1);
//     m1.serialize(&mut ser);

//     info!("ser: {:?}", ser);
//     info!("ser: {}", ser);

//     assert_eq!(ser.body().to_string(), String::from("1=STR|4=C|"));

//     let mut m2 = m1.clone();
//     assert_eq!(m1, m2);

//     m2.account = "STR2".into();
//     info!("m1: {:?}", m1);
//     info!("m2: {:?}", m2);

//     assert_ne!(m1, m2);

//     let m3_owned = m2.to_owned();
//     info!("m3_owned: {:?}", m3_owned);

//     let m1_to_json = to_string(&m1)?;
//     info!("m1_to_json: {}", m1_to_json);

//     let m1_from_json = from_str::<Message<&str>>(&m1_to_json)?;
//     info!("m1_from_json: {:?}", m1_from_json);
//     assert_eq!(m1, m1_from_json);

//     Ok(())
// }
