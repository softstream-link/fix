use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Value {
    #[serde(rename = "@enum")]
    pub enum_value: String,
    #[serde(rename = "@description")]
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct QuickFixField {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@number")]
    pub number: String,
    #[serde(rename = "@type")]
    pub field_type: String,

    #[serde(rename = "$value")]
    pub values: Option<Vec<Value>>,
}

#[derive(Debug, Deserialize)]
pub struct Fields {
    #[serde(rename = "$value")]
    pub fields: Vec<QuickFixField>,
}

#[derive(Debug, Deserialize)]
pub struct QuickFixRoot {
    #[serde(rename = "@type")]
    pub protocol_type: String,
    #[serde(rename = "@major")]
    pub major: String,
    #[serde(rename = "@minor")]
    pub minor: String,
    #[serde(rename = "@servicepack")]
    pub servicepack: String,

    fields: Fields,
}
impl QuickFixRoot {
    pub fn fields(&self) -> &Vec<QuickFixField> {
        &self.fields.fields
    }
}
impl<S: AsRef<str>> From<S> for QuickFixRoot {
    fn from(xml: S) -> Self {
        quick_xml::de::from_str(xml.as_ref()).unwrap()
    }
}

#[cfg(feature = "unittest")]
#[cfg(test)]
mod tests {
    use std::io::Result;

    use super::*;
    use crate::prelude::*;
    #[test]
    fn test_deserialize_fix() -> Result<()> {
        let xml = resource_to_string!("quickfix/FIX-5.0.xml")?;
        let fix = QuickFixRoot::from(xml);
        println!("fix: {:#?}", fix);
        Ok(())
    }
}
