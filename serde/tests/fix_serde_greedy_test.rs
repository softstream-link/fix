use fix_model_test::unittest::setup;
use fix_serde::{
    new_deserializer, new_serializer_with_capacity,
    unittest::{from_slice_unittest, to_bytes_unittest},
};
use log::info;
use serde::{Deserialize, Serialize};

// <header>
//     <field name='BeginString' required='Y' />
//     <field name='BodyLength' required='Y' />
//     <field name='MsgType' required='Y' />
//     <field name='SenderCompID' required='Y' />
//     <field name='TargetCompID' required='Y' />
//     <field name='OnBehalfOfCompID' required='N' />
//     <field name='DeliverToCompID' required='N' />
fix_model_generator::prelude::fix_string!(BeginString, 8);
fix_model_generator::prelude::fix_usize_fixed_length!(BodyLength, 9);
fix_model_generator::prelude::fix_string!(MsgType, 35);
fix_model_generator::prelude::fix_string!(OnBehalfOfCompID, 115);
fix_model_generator::prelude::fix_string!(DeliverToCompID, 128);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TaggedBeginString<S> {
    #[serde(rename = "8")]
    begin_string: BeginString<S>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TaggedBodyLength {
    #[serde(rename = "9")]
    body_length: BodyLength,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TaggedMsgType<S> {
    #[serde(rename = "35")]
    msg_type: MsgType<S>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TaggedOnBehalfOfCompID<S> {
    #[serde(rename = "49")]
    sender_comp_id: OnBehalfOfCompID<S>,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TaggedDeliverToCompID<S> {
    #[serde(rename = "56")]
    target_comp_id: DeliverToCompID<S>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Header<S> {
    // basic fields
    #[serde(rename = "8")]
    pub begin_string: BeginString<S>,
    #[serde(rename = "9")]
    pub body_length: BodyLength,
    #[serde(rename = "35")]
    pub msg_type: MsgType<S>,
    #[serde(rename = "49")]
    pub sender_comp_id: OnBehalfOfCompID<S>,
    #[serde(rename = "56")]
    pub target_comp_id: DeliverToCompID<S>,
}

// <message name='Logon' msgtype='A' msgcat='admin'>
//     <field name='EncryptMethod' required='Y' />
//     <field name='HeartBtInt' required='Y' />
fix_model_generator::prelude::fix_usize!(EncryptMethod, 98);
fix_model_generator::prelude::fix_usize!(HeartBtInt, 108);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct LogonMsg {
    // basic fields
    #[serde(rename = "98")]
    pub encrypt_method: EncryptMethod,
    // basic fields
    #[serde(rename = "108")]
    pub heart_bt_int: HeartBtInt,
}

#[test]
fn test_greedy_tagged_and_header() -> Result<(), Box<dyn std::error::Error>> {
    setup::log::configure_level(log::LevelFilter::Info);
    let header = Header {
        begin_string: BeginString("FIX.4.4"),
        body_length: BodyLength(0),
        msg_type: MsgType("A"),
        sender_comp_id: OnBehalfOfCompID("source"),
        target_comp_id: DeliverToCompID("dest"),
    };
    let body = LogonMsg {
        encrypt_method: 0.into(),
        heart_bt_int: 30.into(),
    };
    let mut ser = new_serializer_with_capacity(1024);
    header.serialize(&mut ser)?;
    body.serialize(&mut ser)?;

    info!("ser: {:?}", ser);
    let mut des = new_deserializer(&ser);

    // //////////////////////////////////// TAGGED
    // good first element
    let begin_string = TaggedBeginString::<&str>::deserialize(&mut des)?;
    info!("begin_string: {:?}, des: {:?}", begin_string, des);
    assert!(matches!(des.end().unwrap_err(), fix_serde::prelude::Error::TrailingBytes));

    // out of order
    let on_behalf_of = TaggedOnBehalfOfCompID::<&str>::deserialize(&mut des).unwrap_err();
    info!("on_behalf_of: {:?}, des: {:?}", on_behalf_of, des);

    // good second element
    let body_length = TaggedBodyLength::deserialize(&mut des)?;
    info!("body_length: {:?}, des: {:?}", body_length, des);
    des.end().unwrap_err();

    // good third element
    let msg_type = TaggedMsgType::<&str>::deserialize(&mut des)?;
    info!("msg_type: {:?}, des: {:?}", msg_type, des);
    des.end().unwrap_err();

    // good fourth element
    let on_behalf_of = TaggedOnBehalfOfCompID::<&str>::deserialize(&mut des)?;
    info!("on_behalf_of: {:?}, des: {:?}", on_behalf_of, des);
    des.end().unwrap_err();

    // good fifth element
    let deliver_to_comp_id = TaggedDeliverToCompID::<&str>::deserialize(&mut des)?;
    info!("deliver_to_comp_id: {:?}, des: {:?}", deliver_to_comp_id, des);
    des.end().unwrap_err();

    // body greedy
    let body = LogonMsg::deserialize(&mut des)?;
    info!("body: {:?}, des: {:?}", body, des);
    des.end().unwrap();

    // ////////////////////////////////////// HEADER
    let mut des = new_deserializer(&ser);
    let header = Header::<&str>::deserialize(&mut des)?;
    info!("header: {:?}, des: {:?}", header, des);
    des.end().unwrap_err();

    let body = LogonMsg::deserialize(&mut des)?;
    info!("body: {:?}, des: {:?}", body, des);
    des.end().unwrap();
    Ok(())
}
