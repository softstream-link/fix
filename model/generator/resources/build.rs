// THIS IS A SYM LINK IN SRC

use fix_model_generator::prelude::*;
use std::env;
use std::fs;
use std::path::Path;

macro_rules! cargo_info {
    ($MESSAGE:expr) => {
        println!("cargo::warning=build.rs INFO: {}", $MESSAGE)
    };
}

macro_rules! cargo_error {
    ($MESSAGE:expr) => {
        println!("cargo::warning=build.rs ERROR: {}", $MESSAGE)
    };
}
fn errors_check(errors: Vec<Error>) {
    for error in &errors {
        cargo_error!(format!("fix model error: {}", error));
    }
    let profile = std::env::var("PROFILE").unwrap();
    match (profile.as_str(), !errors.is_empty()) {
        ("debug", true) => cargo_info!(format!("🦀🦀 DEBUG build hence allowing to proceed with code generator errors 🦀🦀")),
        ("release", true) => panic!("RELEASE build hence NOT allowing to proceed with code generator errors"),
        _ => (),
    }
}
fn main() {
    // fix_model_vXX -> FIX-X.X.xml
    let fix_version: usize = std::env::var("CARGO_PKG_NAME").unwrap().replace("fix_model_v", "").parse().unwrap();
    let xml_name = format!("FIX-{}.{}.xml", fix_version / 10, fix_version % 10);

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=resources/{}", xml_name);

    let (inp_path, content) = resource_to_string!(xml_name.clone());
    cargo_info!(format!("fix model input: {:?}", inp_path));

    let out_dir = env::var_os("OUT_DIR").unwrap();
    cargo_info!(format!("fix model dir: {:?}", out_dir));

    let qf_model = QFModel::from(content);
    let r_model = RFModel::from(&qf_model);
    errors_check(r_model.errors());

    let fields_code = r_model.fld_defs_to_code();
    let out_path = Path::new(&out_dir).join("fields.rs");
    cargo_info!(format!("fix fields output: {:?}", out_path));
    fs::write(&out_path, fields_code).unwrap();

    let (messages_def, messages_impls) = r_model.msg_to_code();
    let out_path = Path::new(&out_dir).join("messages_defs.rs");
    cargo_info!(format!("fix messages output: {:?}", out_path));
    fs::write(&out_path, messages_def).unwrap();

    let out_path = Path::new(&out_dir).join("messages_impls.rs");
    cargo_info!(format!("fix messages output: {:?}", out_path));
    fs::write(&out_path, messages_impls).unwrap();

    let index_code = r_model.schema_to_code();
    let out_path = Path::new(&out_dir).join("schema.rs");
    cargo_info!(format!("fix index output: {:?}", out_path));
    fs::write(&out_path, index_code).unwrap();

    let (repgrps_defs, repgrps_impls) = r_model.repgrp_to_code();
    let out_path = Path::new(&out_dir).join("repgrps_defs.rs");
    cargo_info!(format!("fix messages output: {:?}", out_path));
    fs::write(&out_path, repgrps_defs).unwrap();

    let out_path = Path::new(&out_dir).join("repgrps_impls.rs");
    cargo_info!(format!("fix messages output: {:?}", out_path));
    fs::write(&out_path, repgrps_impls).unwrap();

    let msg_enums = r_model.msg_defs_enum_to_code();
    let out_path = Path::new(&out_dir).join("msg_enums.rs");
    cargo_info!(format!("fix messages output: {:?}", out_path));
    fs::write(&out_path, msg_enums).unwrap();

    // let helpers = r_model.helpers_to_code();
    // let out_path = Path::new(&out_dir).join("helpers.rs");
    // cargo_info!(format!("fix messages output: {:?}", out_path));
    // fs::write(&out_path, helpers).unwrap();
}
