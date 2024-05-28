use core::str;

use fix_model_core::prelude::*;
use fix_model_test::unittest::setup;
use fix_model_v42::*;
use log::info;

#[test]
fn test_logon() {
    setup::log::configure_level(log::LevelFilter::Info);
    let msg = Logon::<&str, dat_codec> {
        encrypt_method: 0.into(),
        heart_bt_int: 30.into(),
        raw_data: Some(b"abc".into()),
        max_message_size: None,
        reset_seq_num_flag: Some(true.into()),
        logon_msg_types_grp: Some(vec![LogonMsgTypesGrp {
            ref_msg_type: Some("0".into()),
            msg_direction: MsgDirection::Receive.into(),
        }]),
        ..Default::default()
    };

    info!("msg_type: {:?}", msg.msg_type());
    info!("msg: {:?}", msg);

    let fix = fix_serde::to_bytes_with_schema::<_, Fix42Schema>(&msg, None).unwrap();
    info!("fix: {}", fix);
    let json = serde_json::to_string(&msg).unwrap();
    info!("json: {}", json);

    let fix_out: Logon<&str, dat_codec> = fix_serde::from_slice_with_schema::<_, Fix42Schema>(&fix).unwrap();
    assert_eq!(msg, fix_out);

    let mut json_out: Logon<&str, dat_codec> = serde_json::from_str(&json).unwrap();
    if let Some(x) = &mut json_out.raw_data {
        x.decode().unwrap();
    }
    assert_eq!(msg, json_out);
}
#[test]
fn test_generated_test_request() {
    setup::log::configure();
    let msg = TestRequest::<&str>::default();
    info!("msg: {:?}", msg);

    let fix = fix_serde::to_bytes(&msg).unwrap();
    info!("fix: {}", fix);
    let json = serde_json::to_string(&msg).unwrap();
    info!("json: {}", json);
}
#[test]
fn test_generated_new_order_single() {
    setup::log::configure();
    let s = NewOrderSingle::<String, char, Data> {
        cl_ord_id: "cl_ord_id".into(),
        symbol: "IBM".into(),
        handl_inst: HandlInst::AutomatedExecutionOrderPrivateNoBrokerIntervention,
        ..Default::default()
    };
    info!("s: {:?}", s);
}
