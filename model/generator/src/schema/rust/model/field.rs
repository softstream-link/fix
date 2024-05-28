use crate::schema::quickfix::model::{field_def::*, Error};
use convert_case::{Case, Casing};
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote, ToTokens};

use super::{GenericTypeInfo, IsGenericMember};

#[derive(Debug, Clone)]
pub enum RFldDef {
    Plain(RFldDefPlain),
    Data(RFldDefData),
    RepGroup(RFldDefRepGroup),
}
impl IsGenericMember for RFldDef {
    fn is_generic_string(&self) -> bool {
        match self {
            RFldDef::Plain(fld) => fld.is_generic_string(),
            RFldDef::Data(fld) => fld.is_generic_string(),
            RFldDef::RepGroup(fld) => fld.is_generic_string(),
        }
    }
    fn is_generic_char(&self) -> bool {
        match self {
            RFldDef::Plain(fld) => fld.is_generic_char(),
            RFldDef::Data(fld) => fld.is_generic_char(),
            RFldDef::RepGroup(fld) => fld.is_generic_char(),
        }
    }
    fn is_generic_len_data(&self) -> bool {
        match self {
            RFldDef::Plain(fld) => fld.is_generic_len_data(),
            RFldDef::Data(fld) => fld.is_generic_len_data(),
            RFldDef::RepGroup(fld) => fld.is_generic_len_data(),
        }
    }
}
impl RFldDef {
    pub fn tag(&self) -> usize {
        match self {
            RFldDef::Plain(fld) => fld.tag,
            RFldDef::Data(fld) => fld.len_tag,
            RFldDef::RepGroup(fld) => fld.tag,
        }
    }
}
impl ToTokens for RFldDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            RFldDef::Plain(fld) => fld.to_tokens(tokens), // will generate fix_string! or fix_usize! etc
            RFldDef::Data(fld) => fld.to_tokens(tokens),  // will generate fix_data!
            RFldDef::RepGroup(_fld) => (),                // do nothing , as rep groups are generated in a different way
        }
    }
}
impl Ord for RFldDef {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.tag().cmp(&other.tag())
    }
}
impl PartialOrd for RFldDef {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for RFldDef {}
impl PartialEq for RFldDef {
    fn eq(&self, other: &Self) -> bool {
        self.tag() == other.tag()
    }
}
#[derive(Debug, Clone)]
pub struct RFldDefData {
    pub len_name: String,
    pub len_tag: usize,
    pub data_name: String,
    pub data_tag: usize,
}
impl IsGenericMember for RFldDefData {
    fn is_generic_string(&self) -> bool {
        false
    }
    fn is_generic_char(&self) -> bool {
        false
    }
    fn is_generic_len_data(&self) -> bool {
        true
    }
}
impl ToTokens for RFldDefData {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let len_name = format_ident!("{}", self.len_name);
        let len_id = Literal::usize_unsuffixed(self.len_tag);
        let data_name = format_ident!("{}", self.data_name);
        let data_id = Literal::usize_unsuffixed(self.data_tag);
        tokens.extend(quote!(
            fix_model_generator::prelude::fix_data!(#len_name, #len_id, #data_name, #data_id);
        ));
    }
}

#[derive(Debug, Clone)]
pub enum RFldPlainType {
    String,  // has generic
    CharAny, // has generic
    USize,
    USizeFixedLength,
    U8FixedLength,
    ISize,
    Float64,
    Float32,
    AsciiCharEnum(Vec<RAsciiCharVariant>),
    Bool,
}
#[derive(Debug, Clone)]
pub struct RFldDefPlain {
    pub name: String,
    pub tag: usize,
    pub fix_type: String,
    pub new_type: RFldPlainType,
}
impl IsGenericMember for RFldDefPlain {
    fn is_generic_string(&self) -> bool {
        matches!(self.new_type, RFldPlainType::String)
    }
    fn is_generic_char(&self) -> bool {
        matches!(self.new_type, RFldPlainType::CharAny)
    }
    fn is_generic_len_data(&self) -> bool {
        false
    }
}

impl TryFrom<&QFFieldDef> for RFldDefPlain {
    type Error = Error;
    fn try_from(qf_field_def: &QFFieldDef) -> Result<Self, Self::Error> {
        // log::warn!("RFldDefPlain name: {}, type: {}", qf_field_def.name, qf_field_def.r#type);

        let field_type = if ["BodyLength"].contains(&qf_field_def.name.as_str()) {
            RFldPlainType::USizeFixedLength
        } else if ["CheckSum"].contains(&qf_field_def.name.as_str()) {
            RFldPlainType::U8FixedLength
        } else if qf_field_def.is_generic_string() {
            RFldPlainType::String
        } else if qf_field_def.is_generic_char() {
            RFldPlainType::CharAny
        } else if qf_field_def.is_type_isize() {
            RFldPlainType::ISize
        } else if qf_field_def.is_type_usize() {
            RFldPlainType::USize
        } else if qf_field_def.is_float64() {
            RFldPlainType::Float64
        } else if qf_field_def.is_float32() {
            RFldPlainType::Float32
        } else if qf_field_def.is_bool() {
            RFldPlainType::Bool
        } else if qf_field_def.is_ascii_char_enum() {
            RFldPlainType::AsciiCharEnum(
                qf_field_def
                    .variants
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|var| RAsciiCharVariant::from(var))
                    .collect(),
            )
        } else {
            return Err(Error::QuickFixFieldTypeNotMapped(format!(
                "name: {}, type: {}",
                qf_field_def.name, qf_field_def.r#type
            )));
        };
        Ok(Self {
            name: qf_field_def.name.clone(),
            tag: qf_field_def.number.parse().expect(
                format!(
                    "quickfix definion of field 'number' is not valid, expected usize. value: {:?}",
                    qf_field_def
                )
                .as_str(),
            ),
            new_type: field_type,
            fix_type: qf_field_def.r#type.clone(),
        })
    }
}
impl From<&RFldDefPlain> for TokenStream {
    fn from(value: &RFldDefPlain) -> Self {
        let name = format_ident!("{}", value.name);
        let tag = Literal::usize_unsuffixed(value.tag);
        match &value.new_type {
            RFldPlainType::String => quote!(
                fix_model_generator::prelude::fix_string!(#name, #tag);
            ),
            RFldPlainType::USize => quote!(
                fix_model_generator::prelude::fix_usize!(#name, #tag);
            ),
            RFldPlainType::USizeFixedLength => quote!(
                fix_model_generator::prelude::fix_usize_fixed_length!(#name, #tag);
            ),
            RFldPlainType::U8FixedLength => quote!(
                fix_model_generator::prelude::fix_u8_fixed_length!(#name, #tag);
            ),
            RFldPlainType::ISize => quote!(
                fix_model_generator::prelude::fix_isize!(#name, #tag);
            ),
            RFldPlainType::Float64 => quote!(
                fix_model_generator::prelude::fix_float64!(#name, #tag);
            ),
            RFldPlainType::Float32 => quote!(
                fix_model_generator::prelude::fix_float32!(#name, #tag);
            ),
            RFldPlainType::Bool => quote!(
                fix_model_generator::prelude::fix_bool!(#name, #tag);
            ),
            RFldPlainType::CharAny => quote!(
                fix_model_generator::prelude::fix_char_any!(#name, #tag);
            ),
            RFldPlainType::AsciiCharEnum(vars) => quote!(
                fix_model_generator::prelude::fix_ascii_char_enum!(#name, #tag, #(#vars, )* );
            ),
        }
    }
}
impl ToTokens for RFldDefPlain {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(Into::<TokenStream>::into(self));
    }
}

#[derive(Debug, Clone)]
pub struct RAsciiCharVariant {
    pub val: char,
    pub name: String,
}
impl From<&RAsciiCharVariant> for TokenStream {
    fn from(value: &RAsciiCharVariant) -> Self {
        let name = format_ident!("{}", value.name);
        let val = Literal::string(value.val.to_string().as_str());
        quote!(
            #name: #val
        )
    }
}
impl ToTokens for RAsciiCharVariant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(Into::<TokenStream>::into(self));
    }
}
impl From<&QFVariant> for RAsciiCharVariant {
    fn from(qf_enum: &QFVariant) -> Self {
        Self {
            val: {
                assert!(
                    qf_enum.enum_value.chars().count() == 1,
                    "Quick Fix Enum value is '{}' but only expected a single character enum: {}",
                    qf_enum.enum_value,
                    qf_enum
                );
                assert!(
                    qf_enum.enum_value.is_ascii(),
                    "Quick Fix Enum value is not an ascii character: {}",
                    qf_enum.enum_value
                );
                qf_enum.enum_value.chars().next().expect("enum value is empty")
            },
            name: {
                let name = if qf_enum.description.chars().next().expect("enum description is empty").is_alphabetic() {
                    qf_enum.description.clone()
                } else {
                    format!("E{}", qf_enum.description)
                };
                name.to_case(Case::Pascal)
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct RFldDefRepGroup {
    pub name: String,
    pub tag: usize,
    pub generic_info: GenericTypeInfo,
}
impl IsGenericMember for RFldDefRepGroup {
    fn is_generic_string(&self) -> bool {
        self.generic_info.is_generic_string()
    }
    fn is_generic_char(&self) -> bool {
        self.generic_info.is_generic_char()
    }
    fn is_generic_len_data(&self) -> bool {
        self.generic_info.is_generic_len_data()
    }
}
