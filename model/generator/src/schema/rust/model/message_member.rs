use crate::prelude::Error;

use super::{
    field::{RFldDef, RFldDefData, RFldDefPlain, RFldDefRepGroup, RFldPlainType},
    root::RFModel,
    GenericTypeInfo, IsGenericMember,
};
use convert_case::{Case, Casing};
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote, ToTokens};

/// Wrapper around a vector of `RMessageMember` to be able to impl From
pub struct RMessageMembers {
    pub members: Vec<RMessageMember>,
    pub errors: Vec<Error>,
}
#[derive(Debug)]
pub struct RMessageMember {
    pub member: RFldDef,
    pub required: bool,
}
impl IsGenericMember for RMessageMember {
    fn is_generic_string(&self) -> bool {
        self.member.is_generic_string()
    }
    fn is_generic_char(&self) -> bool {
        self.member.is_generic_char()
    }
    fn is_generic_len_data(&self) -> bool {
        self.member.is_generic_len_data()
    }
}
impl RMessageMember {
    /// Returns somethinkg like this for string types
    /// ```
    /// /*
    /// struct MyStruct<S>
    /// where
    ///  Blah<S>: Default, // <-- this line
    ///  Blah1<S>: Default, // <-- this line
    /// {
    ///     blah: Blah<S>,
    ///     blah1: Option<Blah1<S>>,
    /// }
    /// */
    /// ```
    pub fn default_trait_bounds(&self, rf_model: &RFModel) -> TokenStream {
        match &self.member {
            RFldDef::Plain(plain) => match plain.new_type {
                RFldPlainType::String => {
                    let member_type_name = format_ident!("{}", plain.name);
                    quote!(#member_type_name<S>: Default,)
                }
                RFldPlainType::CharAny => {
                    let member_type_name = format_ident!("{}", plain.name);
                    quote!(#member_type_name<C>: Default,)
                }
                _ => quote!(),
            },
            RFldDef::RepGroup(rep_grp) => {
                // log::error!("default_bounds: rep group member name: {}", rep_grp.name);
                rf_model.repgrp_default_bounds(rep_grp)
            }
            _ => quote!(),
        }
    }
    pub fn name(&self) -> &str {
        match &self.member {
            RFldDef::Plain(plain) => &plain.name,
            RFldDef::Data(data) => &data.data_name,
            RFldDef::RepGroup(rep_grp) => &rep_grp.name,
        }
    }
    pub fn serialize_field(&self, is_human_redable: bool) -> TokenStream {
        match &self.member {
            RFldDef::Plain(RFldDefPlain { name, tag, .. })
            | RFldDef::Data(RFldDefData {
                data_name: name,
                len_tag: tag,
                ..
            })
            | RFldDef::RepGroup(RFldDefRepGroup { name, tag, .. }) => {
                let member_name = format_ident!("r#{}", name.to_case(Case::Snake));

                let rename_tag = match is_human_redable {
                    // false => Literal::usize_unsuffixed(*tag),
                    false => Literal::string(format!("{}", tag).as_str()),
                    true => Literal::string(name),
                };
                if self.required {
                    quote!(__serde_state.serialize_field(#rename_tag, &self.#member_name)?;)
                } else {
                    quote!(
                        if !Option::is_none(&self.#member_name) {
                            __serde_state.serialize_field(#rename_tag, &self.#member_name)?;
                        }
                    )
                }
            }
        }
    }
    pub fn member_len(&self) -> TokenStream {
        if self.required {
            quote!(1)
        } else {
            // #(+ if Option::is_none(&self.#member) { 0 } else { 1 })*
            match &self.member {
                RFldDef::Plain(RFldDefPlain { name, .. })
                | RFldDef::Data(RFldDefData { data_name: name, .. })
                | RFldDef::RepGroup(RFldDefRepGroup { name, .. }) => {
                    let member_name = format_ident!("r#{}", name.to_case(Case::Snake));
                    quote!( if Option::is_none(&self.#member_name) { 0 } else { 1 } )
                }
            }
        }
    }
}
impl From<&RMessageMember> for TokenStream {
    fn from(value: &RMessageMember) -> Self {
        let required = value.required;
        let is_optional = match required {
            true => quote!(),
            false => quote!(#[serde(skip_serializing_if = "Option::is_none")]),
        };

        let field_meta = &value.member;
        match field_meta {
            RFldDef::Data(fld_meta) => {
                let member_rename = fld_meta.len_tag.to_string(); // note data name vs len id
                let member_alias = &fld_meta.data_name; // note data name vs len id
                let member_name = format_ident!("{}", fld_meta.data_name.to_case(Case::Snake)); // note data name vs len id
                let member_type_name = format_ident!("{}", fld_meta.data_name);
                let doc = format!(" {:?} New Type wrapper", fld_meta);
                let member_type = match required {
                    true => quote!(#member_type_name),
                    false => quote!(Option<#member_type_name<D>>),
                };
                quote!(
                    #[doc = #doc]
                    #[serde(rename = #member_rename)]
                    #[serde(alias = #member_alias)]
                    #is_optional
                    pub #member_name: #member_type,
                )
            }
            RFldDef::Plain(fld_meta) => {
                let member_rename = fld_meta.tag.to_string();
                let member_alias = &fld_meta.name;
                let member_name = format_ident!("r#{}", fld_meta.name.to_case(Case::Snake));
                let member_type_name = format_ident!("{}", fld_meta.name);
                let doc = format!(" {:?} New Type wrapper", fld_meta);
                let member_type = match (&fld_meta.new_type, required) {
                    (RFldPlainType::String, true) => quote!(#member_type_name<S>),
                    (RFldPlainType::String, false) => quote!(Option<#member_type_name<S>>),
                    (RFldPlainType::CharAny, true) => quote!(#member_type_name<C>),
                    (RFldPlainType::CharAny, false) => quote!(Option<#member_type_name<C>>),
                    (_, true) => quote!(#member_type_name),
                    (_, false) => quote!(Option<#member_type_name>),
                };

                quote!(
                    #[doc = #doc]
                    #[serde(rename = #member_rename)]
                    #[serde(alias = #member_alias)]
                    #is_optional
                    pub #member_name: #member_type,
                )
            }
            RFldDef::RepGroup(fld_meta) => {
                #[rustfmt::skip]
                let generic = match fld_meta.generic_info {
                    GenericTypeInfo { string: true, chr: true, data: true, } => quote!(<S,C,D>),
                    GenericTypeInfo { string: true, chr: true, data: false, } => quote!(<S,C>),
                    GenericTypeInfo { string: true, chr: false, data: true, } => quote!(<S,D>),
                    GenericTypeInfo { string: true, chr: false, data: false, } => quote!(<S>),
                    GenericTypeInfo { string: false, chr: true, data: true, } => quote!(<C,D>),
                    GenericTypeInfo { string: false, chr: true, data: false, } => quote!(<C>),
                    GenericTypeInfo { string: false, chr: false, data: true, } => quote!(<D>),
                    GenericTypeInfo { string: false, chr: false, data: false, } => quote!(),
                    // (true, true, true) => quote!(<S,C,D>),
                    // (true, true, false) => quote!(<S,C>),
                    // (true, false, true) => quote!(<S,D>),
                    // (true, false, false) => quote!(<S>),
                    // (false, true, true) => quote!(<C,D>),
                    // (false, true, false) => quote!(<C>),
                    // (false, false, true) => quote!(<D>),
                    // (false, false, false) => quote!(),
                };

                let member_rename = fld_meta.tag.to_string();
                let member_alias = &fld_meta.name;
                let member_name = format_ident!("r#{}", fld_meta.name.to_case(Case::Snake));
                let member_type_name = format_ident!("{}", fld_meta.name);

                let doc = format!(" {:?} '{}' New Type wrapper", fld_meta, generic);
                let member_type = match required {
                    true => quote!(Vec< #member_type_name #generic >),
                    false => quote!(Option<Vec< #member_type_name #generic >>),
                };
                quote!(
                    #[doc = #doc]
                    #[serde(rename = #member_rename)]
                    #[serde(alias = #member_alias)]
                    #is_optional
                    pub #member_name: #member_type,
                )
            }
        }
    }
}
impl ToTokens for RMessageMember {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(Into::<TokenStream>::into(self));
    }
}
