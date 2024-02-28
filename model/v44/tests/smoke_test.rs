use core::str;

use fix_model_core::prelude::*;
use fix_model_test::unittest::setup;
use fix_model_v44::*;
use log::info;

#[test]
fn test_default() {
    setup::log::configure();

    let x = AllocGrp::<String, char, Data> {
        alloc_account: Some("alloc_account".into()),
        ..Default::default()
    };
    info!("x: {:?}", x);
}

#[test]
fn test_generated_logon() {
    setup::log::configure();
    let msg = Logon::<&str, &dat> {
        encrypt_method: 0.into(),
        heart_bt_int: 30.into(),
        raw_data: Some(b"abc".into()),
        max_message_size: None,
        reset_seq_num_flag: Some(true.into()),
        ..Default::default()
    };

    info!("msg: {:?}", msg);

    let fix = fix_serde::to_bytes_with_schema(&msg, None, Fix44Schema).unwrap();
    info!("fix: {}", fix);
    let json = serde_json::to_string(&msg).unwrap();
    info!("json: {}", json);

    let msg_app = MsgAdm::Logon(msg);
    info!("msg_app: {:?}", msg_app);
    let fix_app = fix_serde::to_bytes_with_schema(&msg_app, None, Fix44Schema).unwrap();
    info!("fix_app: {}", fix_app);

    let json_app: String = serde_json::to_string(&msg_app).unwrap();
    info!("json_app: {}", json_app);
}
#[test]
fn test_generated_test_request() {
    setup::log::configure_level(log::LevelFilter::Info);
    let msg = TestRequest::<&str>::default();
    info!("msg: {:?}", msg);

    let fix = fix_serde::to_bytes(&msg).unwrap();
    info!("fix: {}", fix);
    let json = serde_json::to_string(&msg).unwrap();
    info!("json: {}", json);
}
#[test]
fn test_new_order_single() {
    setup::log::configure_level(log::LevelFilter::Info);
    let inp_msg = NewOrderSingle::<&str, char, dat_codec<'_>> {
        cl_ord_id: "cl_ord_id".into(),
        encoded_text: Some(b"encoded_text".into()),
        // encoded_text: Some(b"encoded_text".into()),
        // symbol: "IBM".into(),
        handl_inst: HandlInst::ManualOrderBestExecution.into(),
        // TODO "Side":"1" shoudl say buy/sell and not 1/2
        // TODO "TransactTime":"TransactTime:60@Default" does not make sense
        ..Default::default()
    };
    info!("msg: {:?}", inp_msg);

    let fix = fix_serde::to_bytes_with_schema(&inp_msg, None, Fix44Schema).unwrap();
    info!("fix: {}", fix);
    let json = serde_json::to_string(&inp_msg).unwrap();
    info!("json: {}", json);

    let out_fix_msg: NewOrderSingle<&str, char, dat_codec> = fix_serde::from_slice_with_schema(&fix, Fix44Schema).unwrap();
    assert_eq!(out_fix_msg, inp_msg);

    let mut out_json_msg: NewOrderSingle<&str, char, dat_codec> = serde_json::from_str(&json).unwrap();
    if let Some(encoded_text) = &mut out_json_msg.encoded_text {
        encoded_text.decode().unwrap();
    }

    assert_eq!(out_fix_msg, out_json_msg);
}
