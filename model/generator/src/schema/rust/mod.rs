pub mod macros;

use std::{collections::HashSet, io::Write, path::Path, sync::Mutex};

use super::quickfix::*;
use lazy_static::lazy_static;
use log::warn;

lazy_static! {
    static ref QUICK_FIX_TYPE_WARNINGS: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

#[derive(Debug)]
enum FieldType {
    STRING,
    INT,
    CHAR,
    COUNTRY,
}

pub trait IntoRust {
    fn to_code(&self) -> String;
}

#[derive(Debug)]
struct RustFixField {
    name: String,
    id: u32,
    field_type: FieldType,
}
impl From<&QuickFixField> for RustFixField {
    fn from(quick_fix_field: &QuickFixField) -> Self {
        let field_type = match quick_fix_field.field_type.as_str() {
            "STRING" => FieldType::STRING,
            "INT" => FieldType::INT,
            "CHAR" => FieldType::CHAR,
            "COUNTRY" => FieldType::COUNTRY,
            quick_fix_field_type => {
                let mut guard = (*QUICK_FIX_TYPE_WARNINGS)
                    .lock()
                    .expect("failed to lock quickfix type warnings set");
                let quick_fix_field_type = quick_fix_field_type.to_string();
                if !guard.contains(&quick_fix_field_type) {
                    warn!(
                        "RustType 'String' will be used due to mapping missing QuickFixType: '{}'",
                        quick_fix_field_type
                    );
                    guard.insert(quick_fix_field_type);
                }
                FieldType::STRING
            }
        };
        Self {
            name: quick_fix_field.name.clone(),
            id: quick_fix_field.number.parse().expect(
                format!(
                    "field 'number' is not valid u32. value: {:?}",
                    quick_fix_field
                )
                .as_str(),
            ),
            field_type,
        }
    }
}
impl IntoRust for RustFixField {
    fn to_code(&self) -> String {
        match self.field_type {
            FieldType::STRING => format!(
                "fix_model_generator::fix_string!({name}, {id});\n",
                name = self.name,
                id = self.id,
            ),
            FieldType::INT => format!(
                "fix_model_generator::fix_int!({name}, {id});\n",
                name = self.name,
                id = self.id,
            ),
            FieldType::CHAR => format!(
                "fix_model_generator::fix_char!({name}, {id});\n",
                name = self.name,
                id = self.id,
            ),
            FieldType::COUNTRY => format!(
                "fix_model_generator::fix_country!({name}, {id});\n",
                name = self.name,
                id = self.id,
            ),
        }
    }
}

#[derive(Debug)]
pub struct RustFixModel {
    fields: Vec<RustFixField>,
}
impl IntoRust for RustFixModel {
    fn to_code(&self) -> String {
        let mut content = String::new();
        for f in &self.fields {
            content.push_str(f.to_code().as_str());
        }
        content
    }
}
impl From<&QuickFixRoot> for RustFixModel {
    fn from(root: &QuickFixRoot) -> Self {
        let fields = root
            .fields()
            .iter()
            // .take(2)
            .map(|f| RustFixField::from(f))
            .collect();
        Self { fields }
    }
}

pub fn save<I: AsRef<str>, O: AsRef<Path>>(contents: I, path: O) -> std::io::Result<()> {
    use std::fs::write;
    write(path, contents.as_ref())
}

pub fn save_vec<I: AsRef<str>, O: AsRef<Path>>(content: &Vec<I>, path: O) -> std::io::Result<()> {
    use std::fs::File;
    let mut file = File::create(path.as_ref())?;

    for f in content {
        file.write(f.as_ref().as_bytes())?;
    }
    file.flush()?;
    Ok(())
}
