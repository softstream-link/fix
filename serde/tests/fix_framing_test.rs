use bytes::BytesMut;
use fix_model_core::types::fixmsgtype::MsgType;
use fix_model_test::unittest::setup;
use fix_serde::prelude::SendFrame;
use fix_serde::unittest::UnitTestSchema;
use log::info;
use serde::{Deserialize, Serialize};

#[test]
fn test_bytes() {
    setup::log::configure();
    let mut root = BytesMut::with_capacity(1024);
    log(&root, "root");
    let tail = root.split_off(10);
    log(&root, "root");
    log(&tail, "tail");
}

fn log(buf: &BytesMut, name: &str) {
    info!("name: {name}, len: {}, cap: {}, buf: {:?}", buf.len(), buf.capacity(), buf);
}
fix_model_generator::prelude::fix_string!(RefMsgType, 372);
fix_model_generator::prelude::fix_ascii_char_enum!(
    MsgDirection, 385, Receive :  "R", Send :  "S",
);
fix_model_generator::prelude::fix_usize!(EncryptMethod, 98);
fix_model_generator::prelude::fix_usize!(HeartBtInt, 108);

// <field name='EncryptMethod' required='Y' />
// <field name='HeartBtInt' required='Y' />
// <field name='RawDataLength' required='N' />
// <field name='RawData' required='N' />
// <field name='ResetSeqNumFlag' required='N' />
// <field name='NextExpectedMsgSeqNum' required='N' />
// <field name='MaxMessageSize' required='N' />
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct LogonMsg {
    // fields
    #[serde(rename = "98")]
    pub encrypt_method: EncryptMethod,
    // ....
    // more fields
    #[serde(rename = "108")]
    pub heart_bt_int: HeartBtInt,
}
impl MsgType for LogonMsg {
    const MSG_TYPE: &'static str = "A";
}

// <field name='BeginString' required='Y' />
// <field name='BodyLength' required='Y' />
// <field name='MsgType' required='Y' />
// <field name='SenderCompID' required='Y' />
// <field name='TargetCompID' required='Y' />
#[test]
fn test_send_frame() {
    setup::log::configure();

    let msg = LogonMsg {
        encrypt_method: EncryptMethod(0),
        heart_bt_int: HeartBtInt(30),
    };

    info!("msg: {:?}", msg);
    let mut ser = SendFrame::with_capacity(1024, "FIX.4.4", msg.msg_type(), "source", "dest", UnitTestSchema);
    msg.serialize(&mut *ser).unwrap();
    // msg.serialize(ser.deref_mut()).unwrap();
    let ser = ser.envelope(true).unwrap();
    info!("ser: {:?}", ser);
}
