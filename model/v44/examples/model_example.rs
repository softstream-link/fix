use fix_model_core::prelude::*;
use fix_serde::prelude::*;
use fix_model_v44::*;

use fix_model_test::unittest::setup;

use log::info;
type FrameEnchoder44 = FrameEnchoder<Fix44Schema>;
type FrameDecoder44<'a> = FrameDecoder<'a, Fix44Schema>;

pub fn from_fix<'de, T: serde::Deserialize<'de>>(slice: &'de [u8]) -> fix_serde::prelude::Result<T> {
    fix_serde::prelude::from_slice_with_schema::<_, Fix44Schema>(slice)
}
pub fn to_fix<T: serde::Serialize>(value: &T, capacity: Option<usize>) -> fix_serde::prelude::Result<Serializer<BytesWrite, Fix44Schema>> {
    fix_serde::prelude::to_bytes_with_schema::<_, Fix44Schema>(value, capacity)
}

fn main() {
    setup::log::configure_level(log::LevelFilter::Info);
    example_frame();
    example_msg()
}
#[test]
fn test_example() {
    main();
}

fn example_frame() {
    let mut frame_enchoder = FrameEnchoder44::with_capacity(1024, Header1EnvelopeSequence::new("FIX.4.4".to_owned().into()));

    let comp_ids = Header2CompIdSequence::new("sender_comp_id".to_owned().into(), "target_comp_id".to_owned().into());
    let header = Header3OperationalSequence::<&str, &dat> {
        sending_time: "yyyyMMdd-HH:mm:ss.SSS".into(),
        ..Default::default()
    };

    // ADMIN FRAME
    let msg_inp = Logon::<&asc, &dat> {
        heart_bt_int: 30.into(),
        ..Default::default()
    };
    // info!("msg_inp: {:?}", msg_inp);

    let ser = frame_enchoder.serialize(&comp_ids, &header, &msg_inp, true).unwrap();
    info!("ser: {:?}", ser);

    let mut frame_decoder = FrameDecoder44::new(&ser);
    let (adm, app) = frame_decoder.deserialize_msg::<&str, char, &dat>().unwrap();
    assert_eq!(adm.is_some(), app.is_none());
    match adm.unwrap() {
        MsgAdm::Logon(msg) => {
            assert_eq!(msg.heart_bt_int, 30.into());
        }
        msg_adm => assert!(false, "unexpected msg: {:?}", msg_adm),
    }

    let header1 = frame_decoder.deserialize_header1().unwrap();
    info!("header1: {:?}", header1);
    let header2 = frame_decoder.deserialize_header2().unwrap();
    info!("header2: {:?}", header2);
    let header3 = frame_decoder.deserialize_header3().unwrap();
    info!("header3: {:?}", header3);

    // APP FRAME
    let msg_inp = NewOrderSingle::<&asc, aschar, &dat> {
        cl_ord_id: "cl_ord_id".try_into().unwrap(),
        // symbol: Some("symbol".try_into().unwrap()),
        side: Side::Buy,
        order_qty: Some(100_f64.into()),
        price: Some(99.99.into()),
        ..Default::default()
    };

    let ser = frame_enchoder.serialize(&comp_ids, &header, &msg_inp, true).unwrap();
    info!("ser: {:?}", ser);

    let mut frame_decoder = FrameDecoder44::new(&ser);
    let (adm, app) = frame_decoder.deserialize_msg::<&str, char, &dat>().unwrap();
    assert_eq!(app.is_some(), adm.is_none());
    match app.unwrap() {
        MsgApp::NewOrderSingle(msg) => {
            assert_eq!(msg.cl_ord_id.as_ref(), "cl_ord_id");
        }
        msg_app => assert!(false, "unexpected msg: {:?}", msg_app),
    }

    let header1 = frame_decoder.deserialize_header1().unwrap();
    info!("header1: {:?}", header1);
    let header2 = frame_decoder.deserialize_header2().unwrap();
    info!("header2: {:?}", header2);
    let header3 = frame_decoder.deserialize_header3().unwrap();
    info!("header3: {:?}", header3);
}

fn example_msg() {
    // allocated Ascii & Data
    let msg_inp = NewOrderSingle::<Ascii, aschar, Data> {
        cl_ord_id: "cl_ord_id".to_owned().try_into().unwrap(),
        side: Side::Buy,
        order_qty: Some(100_f64.into()),
        price: Some(99.99.into()),
        ..Default::default()
    };
    let fix = to_fix(&msg_inp, 1024.into()).unwrap();
    info!("fix: {}", fix);

    // borrowed Ascii & Data
    let msg_out: NewOrderSingle<&asc, aschar, &dat> = from_fix(&fix).unwrap();
    let msg_out = msg_out.to_owned_inner_if_ref();
    assert_eq!(msg_inp, msg_out);

    let json = serde_json::to_string(&msg_inp).unwrap();
    info!("json: {}", json);

    // allocated String & Data
    let msg_inp = NewOrderSingle::<String, char, Data> {
        cl_ord_id: "cl_ord_id".to_owned().into(),
        symbol: Some("symbol".to_owned().into()),
        side: Side::Buy,
        order_qty: Some(100_f64.into()),
        price: Some(99.99.into()),
        ..Default::default()
    };

    let fix = to_fix(&msg_inp, 1024.into()).unwrap();
    info!("fix: {}", fix);

    // borrowed String & Data
    let msg_out: NewOrderSingle<&str, char, &dat> = from_fix(&fix).unwrap();
    let msg_out = msg_out.to_owned_inner_if_ref();
    assert_eq!(msg_inp, msg_out);


}
