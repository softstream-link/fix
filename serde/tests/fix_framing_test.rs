use bytes::BytesMut;
use fix_model_core::types::dat::dat;
use fix_model_test::unittest::setup;
use fix_serde::prelude::*;
use fix_serde::unittest::{AdminMsg, EncryptMethod, Header3, HeartBtInt, Logon, UnitTestSchema};
use log::info;

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

// <field name='BeginString' required='Y' />
// <field name='BodyLength' required='Y' />
// <field name='MsgType' required='Y' />
// <field name='SenderCompID' required='Y' />
// <field name='TargetCompID' required='Y' />
#[test]
fn test_send_recv_frame() {
    setup::log::configure_level(log::LevelFilter::Info);

    let logon_inp = Logon {
        encrypt_method: 0.into(),
        username: Some("username".into()),
        heart_bt_int: 30.into(),
    };

    info!("logon_inp: {:?}", logon_inp);

    let header1 = Header1EnvelopeSequence::new("FIX.4.4".into());
    let mut enchoder = FrameEnchoder::<_, UnitTestSchema>::with_capacity(1024, header1);

    let header2 = Header2CompIdSequence::new("source".into(), "dest".into());
    let header3 = Header3 {
        on_behalf_of_comp_id: "on_behalf_of_comp_id".into(),
    };
    enchoder.serialize(&header2, &header3, &logon_inp).unwrap();
    let ser = enchoder.envelope(true).unwrap();
    info!("ser: {:?}", ser);

    let mut decoder = FrameDecoder::<UnitTestSchema>::new(&ser);
    let check_sum = decoder.validate_check_sum().unwrap();
    assert_eq!(check_sum, 19);
    let header1 = decoder.deserialize_header1().unwrap();
    info!("header1: {:?}", header1);
    assert_eq!(header1.begin_string, "FIX.4.4".into());
    assert_eq!(header1.body_length, 73.into());
    decoder.complete().unwrap_err(); // still more to dserialize hence should fail

    let header2 = decoder.deserialize_header2().unwrap();
    info!("header2: {:?}", header2);
    assert_eq!(header2.sender_comp_id, "source".into());
    assert_eq!(header2.target_comp_id, "dest".into());
    decoder.complete().unwrap_err(); // still more to dserialize hence should fail

    let header3 = decoder.deserialize_header3().unwrap();
    info!("header3: {:?}", header3);
    assert_eq!(header3.on_behalf_of_comp_id, "on_behalf_of_comp_id".into());
    decoder.complete().unwrap_err(); // still more to dserialize hence should fail

    // should complete the frame by ignoring header3
    let logon_out = decoder.deserialize::<Logon<&str>>().unwrap();
    info!("logon_out: {:?}", logon_out);
    assert_eq!(logon_inp, logon_out);
    decoder.complete().unwrap();
}

#[test]
fn test_send_recv_frame_msg() {
    setup::log::configure_level(log::LevelFilter::Info);

    let logon_inp = Logon {
        encrypt_method: EncryptMethod::new(0),
        username: Some("username".into()),
        heart_bt_int: HeartBtInt::new(30),
    };

    info!("logon_inp: {:?}", logon_inp);

    let header1 = Header1EnvelopeSequence::new("FIX.4.4".into());
    let mut enchoder = FrameEnchoder::<_, UnitTestSchema>::with_capacity(1024, header1);

    let header2 = Header2CompIdSequence::new("source".into(), "dest".into());
    let header3 = Header3 {
        on_behalf_of_comp_id: "on_behalf_of_comp_id".into(),
    };
    enchoder.serialize(&header2, &header3, &logon_inp).unwrap();
    let ser = enchoder.envelope(true).unwrap();
    info!("ser: {:?}", ser);

    ///////////////////////////
    let mut decoder = FrameDecoder::<UnitTestSchema>::new(&ser);
    let check_sum = decoder.validate_check_sum().unwrap();
    assert_eq!(check_sum, 19);

    let header = decoder.deserialize_header3().unwrap();
    info!("header: {:?}", header);
    let (adm, _) = decoder.deserialize_msg::<&str, char, &dat>().unwrap();
    info!("adm: {:?}", adm);
    match adm.unwrap() {
        AdminMsg::Logon(logon_out) => {
            info!("logon_out: {:?}", logon_out);
            assert_eq!(logon_inp, logon_out);
        }
    }

    decoder.complete().unwrap();
}
