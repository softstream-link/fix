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

    let comp_ids = Header2CompIdSequence::new("sender_comp_id".into(), "target_comp_id".into());
    let header = Header3OperationalSequence::<&str, &dat> {
        // target_sub_id: Some("target_sub_id".into()),
        sending_time: "yyyyMMdd-HH:mm:ss.SSS".into(),
        ..Default::default()
    };

    let msg = Logon::<&asc, &dat> {
        heart_bt_int: 30.into(),
        ..Default::default()
    };
    info!("msg: {:?}", msg);

    frame_enchoder.serialize(&comp_ids, &header, &msg).unwrap();
    let ser = frame_enchoder.envelope(true).unwrap();
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
}
