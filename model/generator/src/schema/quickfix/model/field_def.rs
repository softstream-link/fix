use crate::schema::rust::model::{GenericTypeInfo, IsGenericMember};
use serde::Deserialize;
use std::fmt::Display;

/// # Represents the field definitions in the QuickFIX schema
/// ```xml
/// <fix type='FIX' major='4' minor='4' servicepack='0'>
///     <fields>
///         <field number='1' name='Account' type='STRING' />
///         <field number='2' name='AdvId' type='STRING' />
///         <field number='5' name='AdvTransType' type='STRING'>
///             <value enum='N' description='NEW' />
///             <value enum='C' description='CANCEL' />
///             <value enum='R' description='REPLACE' />
///         </field>
/// ```
#[derive(Debug, Deserialize)]
pub struct QFFieldDefs {
    #[serde(rename = "$value")]
    defs: Vec<QFFieldDef>,
}
impl QFFieldDefs {
    pub fn sort(&mut self) {
        self.defs.sort_by(|a, b| a.name.cmp(&b.name));
    }
    pub(super) fn get(&self) -> &Vec<QFFieldDef> {
        &self.defs
    }
    pub(super) fn get_mut(&mut self) -> &mut Vec<QFFieldDef> {
        &mut self.defs
    }
}
/// # Represents a single field definition in the QuickFIX schema
/// ```xml
///  <field number='5' name='AdvTransType' type='STRING'>
///      <value enum='N' description='NEW' />
///      <value enum='C' description='CANCEL' />
///      <value enum='R' description='REPLACE' />
///  </field>
/// ```
#[derive(Debug, Deserialize, Clone)]
pub struct QFFieldDef {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@number")]
    pub number: String,
    #[serde(rename = "@type")]
    pub r#type: String,

    #[serde(rename = "$value")]
    pub variants: Option<Vec<QFVariant>>,
}
impl IsGenericMember for QFFieldDef {
    fn is_generic_string(&self) -> bool {
        static TYPES: &[&str] = &[
            "STRING",
            "COUNTRY",
            "CURRENCY",
            "EXCHANGE",
            // TODO from here onward can add a trait & try_from to go to/from date-2-string
            "MONTHYEAR",           // "YYYYMM"
            "MULTIPLEVALUESTRING", // "a b c"  like a list of enums with bits on & off for flags tuple of strings
            "LOCALMKTDATE",        // "YYYYMMDD"
            "UTCDATE",
            "UTCDATEONLY",  // "YYYYMMDD" 4.2 vs 4.4
            "UTCTIMEONLY",  // "HH:MM:SS" or "HH:MM:SS.sss" or
            "UTCTIMESTAMP", // "YYYYMMDD-HH:MM:SS.sss"
        ];
        TYPES.contains(&self.r#type.as_str())
    }
    fn is_generic_char(&self) -> bool {
        self.r#type == "CHAR" && self.variants.is_none()
    }
    fn is_generic_len_data(&self) -> bool {
        self.is_type_len_data()
    }
}
static LEN_FIELDS_WITHOUT_DATA_SHOULD_BE_TREATED_AS_USIZE: &[&str] = &["BodyLength", "MaxMessageSize", "SecurityXMLLen"];
impl QFFieldDef {
    pub fn generic_memeber_type_info(&self) -> GenericTypeInfo {
        GenericTypeInfo {
            string: self.is_generic_string(),
            chr: self.is_generic_char(),
            data: self.is_generic_len_data(),
        }
    }
    pub fn is_type_data(&self) -> bool {
        self.r#type == "DATA"
    }
    pub fn is_type_length(&self) -> bool {
        self.r#type == "LENGTH" && !LEN_FIELDS_WITHOUT_DATA_SHOULD_BE_TREATED_AS_USIZE.contains(&self.name.as_str())
    }
    pub fn is_type_plain(&self) -> bool {
        !(self.is_type_len_data()) && 
        // skip NUINGROUP because this type is not a field but a value that gets automatically injected by the FIX parser
        !&["NUMINGROUP"].contains(&self.r#type.as_str())
    }
    pub fn is_type_len_data(&self) -> bool {
        self.is_type_data() || self.is_type_length()
    }
    pub fn is_type_isize(&self) -> bool {
        static TYPES: &[&str] = &["INT"];
        TYPES.contains(&self.r#type.as_str())
    }
    pub fn is_type_usize(&self) -> bool {
        static TYPES: &[&str] = &["SEQNUM", "DAYOFMONTH"]; // TODO maybe day of month can be u8
        TYPES.contains(&self.r#type.as_str())
            || (self.r#type == "LENGTH" && LEN_FIELDS_WITHOUT_DATA_SHOULD_BE_TREATED_AS_USIZE.contains(&self.name.as_str()))
    }
    pub fn is_float32(&self) -> bool {
        self.r#type == "PERCENTAGE"
    }
    pub fn is_float64(&self) -> bool {
        static TYPES: &[&str] = &["FLOAT", "AMT", "PRICE", "PRICEOFFSET", "QTY"];
        TYPES.contains(&self.r#type.as_str())
    }
    pub fn is_bool(&self) -> bool {
        self.r#type == "BOOLEAN"
    }
    pub fn is_ascii_char_enum(&self) -> bool {
        self.r#type == "CHAR" && self.variants.is_some()
    }
}

impl Display for QFFieldDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}-{}-{})", self.name, self.number, self.r#type)?;
        let len = 1;
        if let Some(enums) = &self.variants {
            let str = enums.iter().take(len).map(|e| e.to_string()).collect::<Vec<_>>().join(", ");
            write!(f, " enums: {}", str)?;
            write!(
                f,
                "{}",
                if enums.len() > len {
                    format!(" ...+{}", enums.len() - len)
                } else {
                    "".to_owned()
                }
            )?;
        }
        Ok(())
    }
}

/// # Represents a single field variant in the QuickFIX schema
/// ```xml
/// <value enum='N' description='NEW' />
/// ```
#[derive(Debug, Deserialize, Clone)]
pub struct QFVariant {
    #[serde(rename = "@enum")]
    pub enum_value: String,
    #[serde(rename = "@description")]
    pub description: String,
}
impl Display for QFVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.enum_value, self.description)
    }
}
