use std::{fs, path::Path};

use fix_model_generator::prelude::*;
use fix_model_test::unittest::setup;
use log::{info, warn};

#[test]
fn test_root_42_fields() {
    setup::log::configure();
    let (path, content) = resource_to_string!("FIX-4.2.xml");
    info!("path: {:?}", path);
    let qf = QFModel::from(content);

    let field_type = "STRING";
    qf.fld_defs_plain
        .iter()
        .filter(|f| f.r#type == field_type && f.variants.is_none())
        .take(1)
        .for_each(|f| info!("{}: {:?}", field_type, f));
    qf.fld_defs_plain
        .iter()
        .filter(|f| f.r#type == field_type && f.variants.is_some())
        .take(1)
        .for_each(|f| info!("{}: {:?}", field_type, f));

    // Int
    let field_type = "INT";
    qf.fld_defs_plain
        .iter()
        .filter(|f| f.r#type == field_type && f.variants.is_none())
        .take(1)
        .for_each(|f| info!("{}: {:?}", field_type, f));

    qf.fld_defs_plain
        .iter()
        .filter(|f| f.r#type == field_type && f.variants.is_some())
        .take(1)
        .for_each(|f| info!("{}: {:?}", field_type, f));

    // Char
    let field_type = "CHAR";
    qf.fld_defs_plain
        .iter()
        .filter(|f| f.r#type == field_type && f.variants.is_none())
        .take(1)
        .for_each(|f| info!("{}: {:?}", field_type, f));

    qf.fld_defs_plain
        .iter()
        .filter(|f| f.r#type == field_type && f.variants.is_some())
        .take(1)
        .for_each(|f| info!("{}: {:?}", field_type, f));
    // qf.fields().iter().take(5).for_each(|f| info!("field: {:?}", f));
    // info!("qf: {:?}", qf);
}

#[test]
fn test_root_42_msgs() {
    setup::log::configure_compact(log::LevelFilter::Info);
    let (path, content) = resource_to_string!("FIX-4.2.xml");
    info!("path: {:?}", path);
    let qf = QFModel::from(content);

    // let (msg_type, msg_cat) = ("A", "admin"); // Logon
    // let (msg_type, msg_cat) = ("E", "app"); // NewOrderList
    // let display = qf.details(|m| m.msg_type == msg_type && m.msg_cat == msg_cat).unwrap();
    // info!("\n{}:{}", msg_type, display);

    let rf = RFModel::from(&qf);
    for error in rf.errors() {
        warn!("error: {}", error);
    }
    assert_eq!(rf.errors().len(), 0);
    let fields_code = rf.fld_defs_to_code();
    let out_dir = env!("CARGO_TARGET_TMPDIR");
    let out_path = Path::new(out_dir).join("fields_v42.rs");
    info!("out_path: {:?}", out_path);
    fs::write(&out_path, fields_code).unwrap();

    let (messages_code, messages_impl) = rf.msg_to_code();
    let out_path = Path::new(out_dir).join("messages_v42.rs");
    info!("out_path: {:?}", out_path);
    fs::write(&out_path, messages_code).unwrap();
    let out_path = Path::new(out_dir).join("message_impls_v42.rs");
    info!("out_path: {:?}", out_path);
    fs::write(&out_path, messages_impl).unwrap();

    let (repgrp_messages_code, repgrp_messages_impl) = rf.repgrp_to_code();
    let out_path = Path::new(out_dir).join("repgrps_v42.rs");
    info!("out_path: {:?}", out_path);
    fs::write(&out_path, repgrp_messages_code).unwrap();
    let out_path = Path::new(out_dir).join("repgrps_impls_v42.rs");
    info!("out_path: {:?}", out_path);
    fs::write(&out_path, repgrp_messages_impl).unwrap();

    let msg_enums = rf.msg_defs_enum_to_code();
    let out_path = Path::new(out_dir).join("msg_enums_v42.rs");
    info!("out_path: {:?}", out_path);
    fs::write(&out_path, msg_enums).unwrap();

    let index_code = rf.schema_to_code();
    let out_path = Path::new(out_dir).join("index_v42.rs");
    info!("out_path: {:?}", out_path);
    fs::write(&out_path, index_code).unwrap();
}

#[test]
fn test_root_44_msgs() {
    setup::log::configure();
    let (path, content) = resource_to_string!("FIX-4.4.xml");
    info!("path: {:?}", path);
    let qf = QFModel::from(content);

    // let (msg_type, msg_cat) = ("A", "admin"); // Logon
    // let (msg_type, msg_cat) = ("D", "app"); // NewOrderSingle
    // let display = qf.details(|m| m.msg_type == msg_type && m.msg_cat == msg_cat).unwrap();
    // info!("\n{}:{}", msg_type, display);

    let rf = RFModel::from(&qf);
    for error in rf.errors() {
        log::error!("{}", error);
    }
    assert_eq!(rf.errors().len(), 0);
    let fields_code = rf.fld_defs_to_code();
    let out_dir = env!("CARGO_TARGET_TMPDIR");
    let out_path = Path::new(out_dir).join("fields_v44.rs");
    info!("out_path: {:?}", out_path);
    fs::write(&out_path, fields_code).unwrap();

    let (messages_code, messages_impl) = rf.msg_to_code();
    let out_path = Path::new(out_dir).join("messages_v44.rs");
    info!("out_path: {:?}", out_path);
    fs::write(&out_path, messages_code).unwrap();

    let out_path = Path::new(out_dir).join("message_impls_v44.rs");
    info!("out_path: {:?}", out_path);
    fs::write(&out_path, messages_impl).unwrap();

    let (repgrp_messages_code, repgrp_messages_impl) = rf.repgrp_to_code();
    let out_path = Path::new(out_dir).join("repgrps_v44.rs");
    info!("out_path: {:?}", out_path);
    fs::write(&out_path, repgrp_messages_code).unwrap();

    let out_path = Path::new(out_dir).join("repgrps_impls_v44.rs");
    info!("out_path: {:?}", out_path);
    fs::write(&out_path, repgrp_messages_impl).unwrap();

    let msg_enums = rf.msg_defs_enum_to_code();
    let out_path = Path::new(out_dir).join("msg_enums_v44.rs");
    info!("out_path: {:?}", out_path);
    fs::write(&out_path, msg_enums).unwrap();

    let index_code = rf.schema_to_code();
    let out_path = Path::new(out_dir).join("index_v44.rs");
    info!("out_path: {:?}", out_path);
    fs::write(&out_path, index_code).unwrap();

    let index_code = rf.helpers_to_code();
    let out_path = Path::new(out_dir).join("helpers_v44.rs");
    info!("out_path: {:?}", out_path);
    fs::write(&out_path, index_code).unwrap();
}
