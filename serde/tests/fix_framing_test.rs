use bytes::BytesMut;
use fix_model_core::prelude::MsgTypeCode;
use fix_model_test::unittest::setup;
use fix_serde::prelude::*;
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
struct Logon {
    // fields
    #[serde(rename = "98")]
    pub encrypt_method: EncryptMethod,
    // ....
    // more fields
    #[serde(rename = "108")]
    pub heart_bt_int: HeartBtInt,
}
impl MsgTypeCode for Logon {
    const MSG_TYPE_CODE: &'static str = "A";
    fn is_app(&self) -> bool {
        true
    }
    fn is_adm(&self) -> bool {
        true
    }
}

// <field name='BeginString' required='Y' />
// <field name='BodyLength' required='Y' />
// <field name='MsgType' required='Y' />
// <field name='SenderCompID' required='Y' />
// <field name='TargetCompID' required='Y' />
#[test]
fn test_send_recv_frame() {
    setup::log::configure_level(log::LevelFilter::Info);

    let logon_inp = Logon {
        encrypt_method: EncryptMethod(0),
        heart_bt_int: HeartBtInt(30),
    };

    info!("logon_inp: {:?}", logon_inp);

    let header1 = Header1EnvelopeSequence::new("FIX.4.4".into());
    let header2 = Header2TypeCompIdSequence::new(logon_inp.msg_type().into(), "source".into(), "dest".into());
    let mut send_frame = SendFrame::with_capacity(1024, header1, header2, UnitTestSchema);
    send_frame.serialize(&logon_inp).unwrap();
    let ser = send_frame.complete(true).unwrap();
    info!("ser: {:?}", ser);


    let mut recv_frame = RecvFrame::new(&ser, UnitTestSchema);
    recv_frame.check_sum().unwrap();
    let header1 = recv_frame.deserialize::<Header1EnvelopeSequence<&str>>().unwrap();
    info!("header1: {:?}", header1);
    assert_eq!(header1.begin_string, "FIX.4.4".into());
    assert_eq!(header1.body_length, 12.into());
    recv_frame.complete().unwrap_err();

    let header2 = recv_frame.deserialize::<Header2TypeCompIdSequence<&str>>().unwrap();
    info!("header2: {:?}", header2);
    assert_eq!(header2.msg_type, "A".into());
    assert_eq!(header2.sender_comp_id, "source".into());
    assert_eq!(header2.target_comp_id, "dest".into());
    recv_frame.complete().unwrap_err();

    let logon_out = recv_frame.deserialize::<Logon>().unwrap();
    info!("logon_inp: {:?}", logon_out);
    assert_eq!(logon_inp, logon_out);
    recv_frame.complete().unwrap();

}
