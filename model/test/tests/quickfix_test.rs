// // #[test]
// // fn generate_fields() {
// //     use fix_model_generator::schema::rust::Field;
// //     let f = Field {
// //         name: "Account".to_string(),
// //         id: 1,
// //         field_type: fix_model_generator::schema::rust::FieldType::Str,
// //     };
// //     let rust_code = f.into_rust();
// //     assert_eq!(
// //         rust_code,
// //         r#"
// //                 fix_model_generator::fix_string!(Account, 1);
// //                 "#
// //         .to_string()
// //     );
// // }

// use fix_model_generator::{fix_string, prelude::*};

// use fix_model_test::unittest::setup;
// use fix_model_test::{create_target_dir, resource_to_string};
// use log::info;
// use std::error::Error;

// #[test]
// fn test_fix_string() {
//     setup::log::configure();
//     fix_string!(Account, 1);
//     let val = Account::new("value".to_owned());
//     let to_string_n = format!("{}", val);
//     let to_string_m = format!("{:-}", val);
//     let to_string_p = format!("{:+}", val);

//     info!("'{}' \t\t- {{}}", to_string_n);
//     assert_eq!(to_string_n, "1=value".to_string());

//     info!("'{}' \t\t- {{:-}}", to_string_m);
//     assert_eq!(to_string_m, "value".to_string());

//     info!("'{}' \t- {{:+}}", to_string_p);
//     assert_eq!(to_string_p, "Account=value".to_string());
// }

// #[test]
// fn test_fix_int() {
//     setup::log::configure();
//     fix_int!(ListSeqNo, 67);
//     let val = ListSeqNo::new(1);
//     let to_string_n = format!("{}", val);
//     let to_string_p = format!("{:-}", val);
//     let to_string_m = format!("{:+}", val);

//     info!("'{}' \t- {{}}", to_string_n);
//     assert_eq!(to_string_n, "67=1".to_string());

//     info!("'{}' \t\t- {{:-}}", to_string_p);
//     assert_eq!(to_string_p, "1".to_string());

//     info!("'{}' - {{:+}}", to_string_m);
//     assert_eq!(to_string_m, "ListSeqNo=1".to_string());
// }
// #[test]
// fn test_fix_char() {
//     setup::log::configure();
//     fix_char!(AdvSide, 4);
//     let val = AdvSide::new('A');
//     let to_string_n = format!("{}", val);
//     let to_string_p = format!("{:-}", val);
//     let to_string_m = format!("{:+}", val);

//     info!("'{}' \t- {{}}", to_string_n);
//     assert_eq!(to_string_n, "4=A".to_string());

//     info!("'{}' \t\t- {{:-}}", to_string_p);
//     assert_eq!(to_string_p, "A".to_string());

//     info!("'{}' \t- {{:+}}", to_string_m);
//     assert_eq!(to_string_m, "AdvSide=A".to_string());
// }

// #[test]
// fn test_fix_country() {
//     setup::log::configure();
//     fix_country!(Country, 421);
//     let val = Country::new("US".to_string());
//     let to_string_n = format!("{}", val);
//     let to_string_p = format!("{:-}", val);
//     let to_string_m = format!("{:+}", val);

//     info!("'{}' \t- {{}}", to_string_n);
//     assert_eq!(to_string_n, "421=US".to_string());

//     info!("'{}' \t\t- {{:-}}", to_string_p);
//     assert_eq!(to_string_p, "US".to_string());

//     info!("'{}' \t- {{:+}}", to_string_m);
//     assert_eq!(to_string_m, "Country=US".to_string());
// }

// #[test]
// fn test_fix_bool() {
//     setup::log::configure();
//     fix_bool!(PossDupFlag, 43);
//     let val = PossDupFlag::new(true);
//     let to_string_n = format!("{}", val);
//     let to_string_p = format!("{:-}", val);
//     let to_string_m = format!("{:+}", val);

//     info!("'{}' \t\t- {{}}", to_string_n);
//     assert_eq!(to_string_n, "43=Y".to_string());

//     info!("'{}' \t\t\t- {{:-}}", to_string_p);
//     assert_eq!(to_string_p, "Y".to_string());

//     info!("'{}' \t- {{:+}}", to_string_m);
//     assert_eq!(to_string_m, "PossDupFlag=Y".to_string());
// }

// #[test]
// fn test_fix_int_length() {
//     setup::log::configure();
//     fix_length!(BodyLength, 9);
//     let val = BodyLength::new(1);
//     let to_string_n = format!("{}", val);
//     let to_string_p = format!("{:-}", val);
//     let to_string_m = format!("{:+}", val);

//     info!("'{}' \t\t- {{}}", to_string_n);
//     assert_eq!(to_string_n, "9=1".to_string());

//     info!("'{}' \t\t\t- {{:-}}", to_string_p);
//     assert_eq!(to_string_p, "1".to_string());

//     info!("'{}' \t- {{:+}}", to_string_m);
//     assert_eq!(to_string_m, "BodyLength=1".to_string());
// }

// #[test]
// fn test_fix_seq_num() {
//     setup::log::configure();
//     fix_seq_num!(BeginSeqNo, 7);
//     let val = BeginSeqNo::new(1);
//     let to_string_n = format!("{}", val);
//     let to_string_p = format!("{:-}", val);
//     let to_string_m = format!("{:+}", val);

//     info!("'{}' \t- {{}}", to_string_n);
//     assert_eq!(to_string_n, "7=1".to_string());

//     info!("'{}' \t\t- {{:-}}", to_string_p);
//     assert_eq!(to_string_p, "1".to_string());

//     info!("'{}' - {{:+}}", to_string_m);
//     assert_eq!(to_string_m, "BeginSeqNo=1".to_string());
// }

// #[test]
// fn test_fix_number_in_group() {
//     setup::log::configure();
//     fix_number_in_group!(NoMsgTypes, 384);
//     let val = NoMsgTypes::new(1);
//     let to_string_n = format!("{}", val);
//     let to_string_p = format!("{:-}", val);
//     let to_string_m = format!("{:+}", val);

//     info!("'{}' \t\t- {{}}", to_string_n);
//     assert_eq!(to_string_n, "384=1".to_string());

//     info!("'{}' \t\t- {{:-}}", to_string_p);
//     assert_eq!(to_string_p, "1".to_string());

//     info!("'{}' \t- {{:+}}", to_string_m);
//     assert_eq!(to_string_m, "NoMsgTypes=1".to_string());
// }

// #[test]
// fn test_resource() -> Result<(), Box<dyn Error>> {
//     setup::log::configure();
//     let xml = resource_to_string!("quickfix/FIX-5.0.xml")?;
//     let schema = QuickFixRoot::from(xml);
//     info!("schema: {:#?}", schema);
//     Ok(())
// }

// #[test]
// fn test_50_2_rust() -> Result<(), Box<dyn Error>> {
//     setup::log::configure();
//     // let xml = resource_to_string!("quickfix/FIX-5.0.xml")?;
//     let xml = resource_to_string!("quickfix/FIXT-1.1.xml")?;
//     let schema = QuickFixRoot::from(xml);
//     let rust_model = RustFixModel::from(&schema);
//     let out = create_target_dir!("generator").join("out.rs");

//     info!("out: {:?}", out);
//     let res = vec![rust_model.to_code(), "fn main(){}".to_string()];
//     save_vec(&res, &out)?;
//     let t = trybuild::TestCases::new();
//     t.pass(out);
//     Ok(())
// }
