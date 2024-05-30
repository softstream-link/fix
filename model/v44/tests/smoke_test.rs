use core::str;

use fix_model_core::prelude::*;
use fix_model_test::unittest::setup;
use fix_model_v44::*;
use log::info;

#[test]
fn test_tag_value_plain() {
    setup::log::configure_level(log::LevelFilter::Info);

    let msg = TagValueAccount::<String> {
        account: "account".to_owned().into(),
    };
    info!("msg: {:?}", msg);

    let fix = fix_model_v44::to_fix(&msg, None).unwrap();
    info!("fix: {}", fix);
    let json = serde_json::to_string(&msg).unwrap();
    info!("json: {}", json);

    let fix_out: TagValueAccount<String> = fix_model_v44::from_fix(&fix).unwrap();
    assert_eq!(msg, fix_out);
    let json_out: TagValueAccount<String> = serde_json::from_str(&json).unwrap();
    assert_eq!(msg, json_out);
}

#[test]
fn test_tag_value_data() {
    setup::log::configure_level(log::LevelFilter::Info);

    let msg = TagValueEncodedAllocText::<dat_codec> {
        encoded_alloc_text: b"encoded_alloc_text".into(),
    };
    info!("msg: {:?}", msg);

    let fix = fix_model_v44::to_fix(&msg, None).unwrap();
    info!("fix: {}", fix);
    let json = serde_json::to_string(&msg).unwrap();
    info!("json: {}", json);

    let fix_out: TagValueEncodedAllocText<dat_codec> = fix_model_v44::from_fix(&fix).unwrap();
    assert_eq!(msg, fix_out);
    let mut json_out: TagValueEncodedAllocText<dat_codec> = serde_json::from_str(&json).unwrap();
    json_out.encoded_alloc_text.decode().unwrap();
    assert_eq!(msg, json_out);
}

#[test]
fn test_rep_grp() {
    setup::log::configure_level(log::LevelFilter::Info);

    let msg = AllocGrp::<String, char, Data> {
        alloc_account: Some("alloc_account".to_owned().into()),
        ..Default::default()
    };
    info!("msg: {:?}", msg);

    let fix = fix_model_v44::to_fix(&msg, None).unwrap();
    info!("fix: {}", fix);
    let json = serde_json::to_string(&msg).unwrap();
    info!("json: {}", json);

    let fix_out: AllocGrp<String, char, Data> = fix_model_v44::from_fix(&fix).unwrap();
    assert_eq!(msg, fix_out);
    let json_out: AllocGrp<String, char, Data> = serde_json::from_str(&json).unwrap();
    assert_eq!(msg, json_out);
}

#[test]
fn test_generated_logon() {
    setup::log::configure_level(log::LevelFilter::Info);
    let msg = Logon::<&str, dat_codec> {
        encrypt_method: 0.into(),
        heart_bt_int: 30.into(),
        raw_data: Some(b"abc".into()),
        max_message_size: None,
        reset_seq_num_flag: Some(true.into()),
        ..Default::default()
    };

    info!("msg: {:?}", msg);

    let fix = fix_model_v44::to_fix(&msg, None).unwrap();
    info!("fix: {}", fix);
    let json = serde_json::to_string(&msg).unwrap();
    info!("json: {}", json);

    let fix_out: Logon<&str, dat_codec> = fix_model_v44::from_fix(&fix).unwrap();
    assert_eq!(msg, fix_out);
    let mut json_out: Logon<&str, dat_codec> = serde_json::from_str(&json).unwrap();
    if let Some(raw_data) = &mut json_out.raw_data {
        raw_data.decode().unwrap();
    }
    assert_eq!(msg, json_out);

    let msg_app = MsgAdm::Logon(msg);
    info!("msg_app: {:?}", msg_app);
    let fix_app = fix_model_v44::to_fix(&msg_app, None).unwrap();
    info!("fix_app: {}", fix_app);

    let json_app: String = serde_json::to_string(&msg_app).unwrap();
    info!("json_app: {}", json_app);
}

#[test]
fn test_new_order_single() {
    setup::log::configure_level(log::LevelFilter::Info);
    let msg = NewOrderSingle::<&str, char, dat_codec<'_>> {
        cl_ord_id: "cl_ord_id".into(),
        encoded_text: Some(b"encoded_text".into()),
        // encoded_text: Some(b"encoded_text".into()),
        // symbol: "IBM".into(),
        handl_inst: HandlInst::ManualOrderBestExecution.into(),
        // TODO "Side":"1" shoudl say buy/sell and not 1/2
        // TODO "TransactTime":"TransactTime:60@Default" does not make sense
        ..Default::default()
    };
    info!("msg: {:?}", msg);

    let fix = fix_model_v44::to_fix(&msg, None).unwrap();
    info!("fix: {}", fix);
    let json = serde_json::to_string(&msg).unwrap();
    info!("json: {}", json);

    let out_fix: NewOrderSingle<&str, char, dat_codec> = fix_model_v44::from_fix(&fix).unwrap();
    assert_eq!(out_fix, msg);

    let mut out_json_msg: NewOrderSingle<&str, char, dat_codec> = serde_json::from_str(&json).unwrap();
    if let Some(encoded_text) = &mut out_json_msg.encoded_text {
        encoded_text.decode().unwrap();
    }
    assert_eq!(out_fix, out_json_msg);
}
