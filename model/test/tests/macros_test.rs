use fix_model_core::prelude::*;
use fix_model_core::unittest::setup;
use fix_model_generator::{fix_string, prelude::*};
use log::info;
use serde_json::{from_str, to_string};

#[test]
fn test_serialize_field_fix_string() -> serde_json::Result<()> {
    setup::log::configure();
    let mut ser = HeapSerializer::with_capacity(1024);

    fix_string!(Account, 1);
    let a1 = Account::new("ABC".to_owned());
    info!("a1: {:?}", a1);
    let v = format!("{:-}", a1);
    assert_eq!(v, "1=ABC");
    let v = format!("{:+}", a1);
    assert_eq!(v, "Account=ABC");
    let v = format!("{}", a1);
    assert_eq!(v, "ABC");
    let v = format!("{:?}", a1);
    assert_eq!(v, "Account(\"ABC\")");
    let v = to_string(&a1)?;
    assert_eq!(v, "\"ABC\"");

    ser.serialize_field(&a1);

    let a2 = Account::new("ABC");
    info!("a2: {:?}", a2);
    let v = to_string(&a2)?;
    assert_eq!(v, "\"ABC\"");

    ser.serialize_field(&a2);

    info!("ser: {:?}", ser);
    info!("ser: {}", ser);
    assert_eq!(ser.body().to_string(), String::from("1=ABC|1=ABC|"));

    Ok(())
}

#[test]
fn test_serialize_field_fix_char() -> serde_json::Result<()> {
    setup::log::configure();
    let mut ser = HeapSerializer::with_capacity(1024);

    fix_char!(AdvSide, 4);
    let a1 = AdvSide::new('B');
    info!("a1: {:?}", a1);
    let v = format!("{:-}", a1);
    assert_eq!(v, "4=B");
    let v = format!("{:+}", a1);
    assert_eq!(v, "AdvSide=B");
    let v = format!("{}", a1);
    assert_eq!(v, "B");
    let v = format!("{:?}", a1);
    assert_eq!(v, "AdvSide('B')");
    let v = to_string(&a1)?;
    assert_eq!(v, "\"B\"");

    ser.serialize_field(&a1);
    info!("ser: {:?}", ser);
    info!("ser: {}", ser);

    Ok(())
}

#[test]
fn test_serialize_msg() -> serde_json::Result<()> {
    fix_string!(Account, 1);

    fix_char!(AdvSide, 4);

    // fix_message!(TestMessage, account: Account<S>, adv_side: AdvSide);
    fix_message!(Message<S: StringValue>, account: Account<S>, adv_side: AdvSide);
    // #[derive(serde::Serialize, Debug, PartialEq, Clone)]
    // #[serde(rename_all = "PascalCase")]
    // struct Message<S: StringValue> {
    //     account: Account<S>,
    //     adv_side: AdvSide,
    // }
    // impl<S: StringValue> Message<S> {
    //     pub fn to_owned(&self) -> Message<String> {
    //         Message {
    //             account: self.account.to_owned(),
    //             adv_side: self.adv_side.to_owned(),
    //         }
    //     }
    // }
    // impl<S: StringValue> Serialize for Message<S> {
    //     fn serialize(&self, ser: &mut impl Serializer) {
    //         self.account.serialize(ser);
    //         self.adv_side.serialize(ser);
    //     }
    // }

    setup::log::configure();
    let mut ser = HeapSerializer::with_capacity(1024);

    let m1 = Message {
        account: "STR".into(),
        adv_side: 'C'.into(),
    };
    info!("t: {:?}", m1);
    m1.serialize(&mut ser);

    info!("ser: {:?}", ser);
    info!("ser: {}", ser);

    assert_eq!(ser.body().to_string(), String::from("1=STR|4=C|"));

    let mut m2 = m1.clone();
    assert_eq!(m1, m2);

    m2.account = "STR2".into();
    info!("m1: {:?}", m1);
    info!("m2: {:?}", m2);

    assert_ne!(m1, m2);

    let m3_owned = m2.to_owned();
    info!("m3_owned: {:?}", m3_owned);

    let m1_to_json = to_string(&m1)?;
    info!("m1_to_json: {}", m1_to_json);

    let m1_from_json = from_str::<Message<&str>>(&m1_to_json)?;
    info!("m1_from_json: {:?}", m1_from_json);
    assert_eq!(m1, m1_from_json);

    Ok(())
}

#[test]
fn test_as_char() {
    setup::log::configure();
    let f = |c| {
        println!("c:c '{:}'", c as char);
        println!("c: #{:}", c);
        println!("c: x{:x}", c);
        println!("c: b{:>08b}", c);
        let ch = c as char;
        
        print!("b: ");
        for b in ch.to_string().as_bytes() {
            print!("{:>08b}-", b);
        }
        println!();
    };
    let c = 0x61_u8;
    f(c);

    let c = 0x7f_u8;
    f(c);

    let c = 0xff_u8;
    f(c);
}
