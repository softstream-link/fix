/// Will return a [PathBuf] path to a file located in the `resources` directory of the currently built crate.
#[macro_export]
macro_rules! resource {
    ($fname:expr) => {{
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let path_buf = std::path::PathBuf::from(manifest_dir);
        let path_buf = path_buf.join("resources").join($fname);
        assert!(path_buf.is_file(), "file not found: {:?}", path_buf);
        path_buf
    }};
}
pub use resource;
/// Will return a [String] of the file content located in the `resources` directory of the currently built crate.
///
/// Arguments:
/// * `$fname` - a literal string with a path relative to the `resources` directory of the currently built crate.
#[macro_export]
macro_rules! resource_to_string {
    ($fname:expr) => {{
        let path_buf = $crate::resource!($fname);
        (
            path_buf.clone(),
            std::fs::read_to_string(path_buf.clone()).expect(format!("failed to read file: {:?}", path_buf).as_str()),
        )
    }};
}

pub use resource_to_string;

#[macro_export]
macro_rules! create_target_subdir {
    // () => {{
    //     let read_me_location = env!("CARGO_TARGET_DIR");
    //     let path_buf = std::path::PathBuf::from(read_me_location);
    //     let path_buf = path_buf.parent().unwrap().join("target");
    //     std::fs::create_dir_all(&path_buf).unwrap();
    //     path_buf
    // }};
    ($SUB_DIR_NAME:literal) => {{
        let cargo_target_dir = std::env::var("CARGO_TARGET_DIR").expect("CARGO_TARGET_DIR not set");
        let path_buf = std::path::PathBuf::from(cargo_target_dir);
        // let path_buf = path_buf.parent().unwrap().join("target").join($SUB_DIR_NAME);
        let path_buf = path_buf.join($SUB_DIR_NAME);
        std::fs::create_dir_all(&path_buf).unwrap();
        path_buf
    }};
}
pub use create_target_subdir;
