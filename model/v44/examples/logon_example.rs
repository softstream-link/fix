use fix_model_core::prelude::*;
use fix_model_test::unittest::setup;
use fix_model_v44::*;
use log::info;

fn main() {
    example();
}
#[test]
fn test_example() {
    example();
}

fn example() {
    setup::log::configure_level(log::LevelFilter::Info);

    let mut frame_enchoder = FrameEnchoder::with_capacity(1024, Header1EnvelopeSequence::new("FIX.4.4".into()));

    let header2 = Header2CompIdSequence::new("sender_comp_id".into(), "target_comp_id".into());
    let header3 = Header3OperationalSequence::<&str, &dat> {
        // target_sub_id: Some("target_sub_id".into()),
        sending_time: "yyyyMMdd-HH:mm:ss.SSS".into(),
        ..Default::default()
    };

    let msg = Logon::<&asc, &dat> {
        heart_bt_int: 30.into(),
        ..Default::default()
    };
    info!("msg: {:?}", msg);

    frame_enchoder.serialize(&header2, &header3, &msg).unwrap();
    let ser = frame_enchoder.complete(true).unwrap();
    info!("ser: {:?}", ser);

    let mut frame_decoder = FrameDecoder::new(&ser);

    let (adm, _) = frame_decoder.deserialize_msg::<&str, char, &dat>().unwrap();
    info!("adm: {:?}", adm);
    let header1 = frame_decoder.deserialize_header1().unwrap();
    info!("header1: {:?}", header1);
    let header2 = frame_decoder.deserialize_header2().unwrap();
    info!("header2: {:?}", header2);
    let header3 = frame_decoder.deserialize_header3().unwrap();
    info!("header3: {:?}", header3);

    // let header2 = Header2TypeSequence::<&asc> {
    //     msg_type: msg.msg_type().try_into().unwrap(),         // TODO should msg_type() be &asc
    //     sender_comp_id: "sender_comp_id".try_into().unwrap(), // note &asc  requires try_into()
    //     target_comp_id: "target_comp_id".try_into().unwrap(),
    // };
    // let header3 = Header3OperationalSequence::<&asc, &dat> {
    //     target_sub_id: Some("target_sub_id".try_into().unwrap()),
    //     sending_time: "yyyyMMdd-HH:mm:ss.SSS".try_into().unwrap(), // TODO add date handling Utc::now().try_into().unwrap(),
    //     ..Default::default()
    // };

    // let mut ser_body = to_fix(&header2, 1024.into()).unwrap();
    // header3.serialize(&mut ser_body).unwrap();
    // msg.serialize(&mut ser_body).unwrap();
    // info!("ser_body: {:?}", ser_body);

    // let header1 = Header1EnvelopeSequence::<&asc> {
    //     begin_string: "FIX.4.4".try_into().unwrap(),
    //     body_length: ser_body.len().into(),
    // };
    // let mut ser_header = to_fix(&header1, 1024.into()).unwrap();
    // ser_header.join(ser_body);
    // // ser_body.serialize(&mut ser_header).unwrap();
    // info!("ser_header: {:?}", ser_header);
}
