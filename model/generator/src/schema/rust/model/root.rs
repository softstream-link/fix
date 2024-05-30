use super::{
    field::{RFldDef, RFldDefRepGroup},
    format_token_stream,
    schema::{SchemaDef, IndexEntry},
    message::{MessageTokenParts, RFMessageDef},
    repeating_group::RRepGrpMessageDef,
};
use crate::{prelude::Error, schema::rust::model::message::MessageCategory};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Debug)]
pub struct RFModel {
    pub name: String,
    pub fld_defs: Vec<RFldDef>,
    pub msg_defs: Vec<RFMessageDef>,
    pub rep_grp_defs: Vec<RRepGrpMessageDef>,
    pub errors: Vec<Error>,
}
impl RFModel {
    pub fn complete_preparation(&mut self) {
        self.fld_defs.sort(); // by tag, only estetics
        self.msg_defs.sort_by(|a, b| a.name.cmp(&b.name)); // by name for binary search lookup
        self.rep_grp_defs.sort_by(|a, b| a.0.name.cmp(&b.0.name)); // by name for binary search lookup
    }
    pub fn fld_defs_to_code(&self) -> String {
        let flds = &self.fld_defs;
        format_token_stream(&quote! {
            #(#flds)*
        })
    }
    pub fn msg_to_code(&self) -> (String, String) {
        let token_parts = self
            .msg_defs
            .iter()
            .map(|msg_def| MessageTokenParts::from((msg_def, self)))
            .collect::<Vec<_>>();

        let msg_defs = token_parts.iter().map(|token_part| &token_part.msg_def).collect::<Vec<_>>();
        let msg_defs = format_token_stream(&quote! {
            #(#msg_defs)*
        });

        let msg_impls = token_parts.iter().map(|token_part| &token_part.msg_impls).collect::<Vec<_>>();
        let msg_impls = format_token_stream(&quote! {
            #(#msg_impls)*
        });
        (msg_defs, msg_impls)
    }

    pub fn msg_defs_enum_to_code(&self) -> String {
        let apps_enum = self
            .msg_defs
            .iter()
            .filter_map(|msg_def| {
                if matches!(msg_def.msg_category, MessageCategory::App) {
                    let name = format_ident!("{}", msg_def.name.as_str());
                    let generic_names = msg_def.generics(self).generic_names;
                    Some(quote! (#name(#name #generic_names),))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let admin_enum = self
            .msg_defs
            .iter()
            .filter_map(|msg_def| {
                if matches!(msg_def.msg_category, MessageCategory::Admin) {
                    let name = format_ident!("{}", msg_def.name.as_str());
                    let generic_names = msg_def.generics(self).generic_names;
                    Some(quote! (#name(#name #generic_names),))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // TODO add dynamic generic resolution of names
        // TODO Box large enum variants
        format_token_stream(&quote! {
            #[allow(clippy::large_enum_variant)]
            #[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone)]
            pub enum MsgApp<S, C, D>{
                #(#apps_enum)*
            }
            #[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone)]
            pub enum MsgAdm<S, D>{
                #(#admin_enum)*
            }
        })
    }

    pub fn repgrp_default_bounds(&self, rep_grp: &RFldDefRepGroup) -> TokenStream {
        let rep_grp_msg = &self.rep_grp_defs[self
            .rep_grp_defs
            .binary_search_by(|g| g.0.name.cmp(&rep_grp.name))
            .unwrap_or_else(|_| panic!("Missing repeading group definition for {:?}", rep_grp))];
        rep_grp_msg.0.default_trait_bounds(self)
    }
    pub fn repgrp_to_code(&self) -> (String, String) {
        let token_parts = self
            .rep_grp_defs
            .iter()
            .map(|rep_grp_def| MessageTokenParts::from((rep_grp_def, self)))
            .collect::<Vec<_>>();

        let repgrp_msgs = token_parts.iter().map(|token_part| &token_part.msg_def).collect::<Vec<_>>();
        let repgrp_msgs = format_token_stream(&quote! {
            #(#repgrp_msgs)*
        });

        let repgrp_impls = token_parts.iter().map(|token_part| &token_part.msg_impls).collect::<Vec<_>>();
        let repgrp_impls = format_token_stream(&quote! {
            #(#repgrp_impls)*
        });
        (repgrp_msgs, repgrp_impls)
    }

    pub fn schema_to_code(&self) -> String {
        let index = &self.schema_def();
        format_token_stream(&quote! {
            #index
        })
    }
    pub fn helpers_to_code(&self) -> String {
        let schema_name = format_ident!("{}Schema", self.name);
        let methods = quote!(
            pub fn from_fix<'de, T: serde::Deserialize<'de>>(slice: &'de [u8]) -> fix_serde::prelude::Result<T> {
                fix_serde::prelude::from_slice_with_schema::<_, #schema_name>(slice)
            }
            pub fn to_fix<T: serde::Serialize>(value: &T, capacity: Option<usize>) -> fix_serde::prelude::Result<fix_serde::prelude::Serializer<fix_serde::prelude::BytesWrite, #schema_name>> {
                fix_serde::prelude::to_bytes_with_schema::<_,#schema_name>(value, capacity)
            }
           
            pub type FrameEnchoder = fix_serde::prelude::FrameEnchoder<#schema_name >;
            pub type FrameDecoder<'de> = fix_serde::prelude::FrameDecoder<'de, #schema_name >;

        );
        format_token_stream(&quote! {
            #methods
        })
    }
    pub fn errors(&self) -> Vec<Error> {
        let mut errors = vec![];
        errors.extend(self.errors.clone());
        errors.extend(self.msg_defs.iter().flat_map(|m| &m.errors).cloned());
        errors.extend(self.rep_grp_defs.iter().flat_map(|g| &g.0.errors).cloned());
        errors
    }

    fn schema_def(&self) -> SchemaDef {
        let tags = self
            .fld_defs
            .iter()
            .filter_map(|f| match f {
                RFldDef::Data(d) => Some(d),
                _ => None,
            })
            .collect::<Vec<_>>();
        let mut entries = tags
            .iter()
            .map(|f| IndexEntry::Data {
                len_tag: f.len_tag,
                data_tag: f.data_tag,
                len_name: f.len_name.clone(),
                data_name: f.data_name.clone(),
            })
            .collect::<Vec<_>>();
        entries.sort();
        SchemaDef {
            name: self.name.clone(),
            entries,
        }
    }
}
