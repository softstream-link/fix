use super::{
    field::{RFldDef, RFldDefRepGroup},
    format_token_stream,
    index::{IndexDef, IndexEntry},
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
    pub fn ready(&mut self) {
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
    pub fn msg_defs_to_code(&self) -> String {
        let msgs = self
            .msg_defs
            .iter()
            .map(|msg_def| MessageTokenParts::from((msg_def, self)).msg_def)
            .collect::<Vec<_>>();
        format_token_stream(&quote! {
            #(#msgs)*
        })
    }

    pub fn msg_defs_enum_to_code(&self) -> String {
        let apps_enum = self
            .msg_defs
            .iter()
            .filter(|msg_def| msg_def.msg_category == MessageCategory::App)
            .filter_map(|msg_def| {
                let name = format_ident!("{}", msg_def.name.as_str());
                let generic_names = msg_def.generics(self).generic_names;
                Some(quote! (#name(#name #generic_names),))
            })
            .collect::<Vec<_>>();

        let admin_enum = self
            .msg_defs
            .iter()
            .filter(|msg_def| msg_def.msg_category == MessageCategory::Admin)
            .filter_map(|msg_def| {
                let name = format_ident!("{}", msg_def.name.as_str());
                let generic_names = msg_def.generics(self).generic_names;
                Some(quote! (#name(#name #generic_names),))
            })
            .collect::<Vec<_>>();
        // TODO add dynamic resolution of names
        format_token_stream(&quote! {
            #[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone)]
            pub enum MsgApp<S, C, D>{
                #(#apps_enum)*
            }
            #[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone)]
            pub enum MsgAdm<S, D>{
                #(#admin_enum)*
            }
            #[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Clone)]
            pub enum Msg<S, C, D>{
                App(MsgApp<S, C, D>),
                Adm(MsgAdm<S, D>),
            }
        })
    }

    pub fn msg_impls_to_code(&self) -> String {
        let impls = self
            .msg_defs
            .iter()
            .map(|msg_def| MessageTokenParts::from((msg_def, self)).msg_impls)
            .collect::<Vec<_>>();
        format_token_stream(&quote! {
            #(#impls)*
        })
    }
    pub fn repgrp_default_bounds(&self, rep_grp: &RFldDefRepGroup) -> TokenStream {
        let rep_grp_msg = &self.rep_grp_defs[self
            .rep_grp_defs
            .binary_search_by(|g| g.0.name.cmp(&rep_grp.name))
            .expect(format!("Missing repeading group definition for {:?}", rep_grp).as_str())];
        rep_grp_msg.0.default_trait_bounds(self)
    }
    pub fn repgrp_defs_to_code(&self) -> String {
        let repgrp_msgs = self
            .rep_grp_defs
            .iter()
            .map(|rep_grp_def| MessageTokenParts::from((rep_grp_def, self)).msg_def)
            .collect::<Vec<_>>();
        format_token_stream(&quote! {
            #(#repgrp_msgs)*
        })
    }
    pub fn repgrp_impls_to_code(&self) -> String {
        let repgrp_msgs = self
            .rep_grp_defs
            .iter()
            .map(|rep_grp_def| MessageTokenParts::from((rep_grp_def, self)).msg_impls)
            .collect::<Vec<_>>();
        format_token_stream(&quote! {
            #(#repgrp_msgs)*
        })
    }
    pub fn index_to_code(&self) -> String {
        let index = &self.index();
        format_token_stream(&quote! {
            #index
        })
    }
    pub fn errors(&self) -> Vec<Error> {
        let mut errors = vec![];
        errors.extend(self.errors.clone());
        errors.extend(self.msg_defs.iter().map(|m| &m.errors).flatten().cloned());
        errors.extend(self.rep_grp_defs.iter().map(|g| &g.0.errors).flatten().cloned());
        errors
    }

    fn index(&self) -> IndexDef {
        let tags = self
            .fld_defs
            .iter()
            .map(|f| match f {
                RFldDef::Data(d) => Some(d),
                _ => None,
            })
            .filter(|f| f.is_some())
            .map(|f| f.unwrap())
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
        let idx = IndexDef {
            name: self.name.clone(),
            entries,
        };
        idx
    }
}
