use fix_model_core::unittest::setup;
use fix_model_core::prelude::*;
use fix_model_generator::prelude::*;
use log::info;


#[test]
fn test_write_fix_string() {
    setup::log::configure();
    let mut msg = Serializer::with_capacity(1024);

    fix_string!(Account, 1);
    let v = Account::new("ABC".to_owned());
    msg.serialize_field(&v);


    info!("msg: {:#?}", msg);
    info!("msg: {}", msg);
    // assert_eq!(msg.body().to_string(), String::from("1=str|2=string|3=array|4=slice|"));
}