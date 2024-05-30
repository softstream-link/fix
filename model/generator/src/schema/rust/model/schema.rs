use super::root::RFModel;
use crate::schema::rust::model::message::MessageCategory;
use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use quote::ToTokens;

pub enum IndexEntry {
    Data {
        len_tag: usize,
        data_tag: usize,
        len_name: String,
        data_name: String,
    },
}
impl ToTokens for IndexEntry {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            IndexEntry::Data {
                len_tag,
                data_tag,
                len_name,
                data_name,
            } => {
                let len_tag = Literal::byte_string(len_tag.to_string().as_bytes());
                let data_tag = Literal::byte_string(data_tag.to_string().as_bytes());

                tokens.extend(quote! {
                    fix_model_core::prelude::BinaryDataLenPair {
                        #[doc = #len_name]
                        tag_len: #len_tag,
                        #[doc = #data_name]
                        tag_data: #data_tag,
                    }
                });
            }
        }
    }
}
impl IndexEntry {
    fn get_len_tag_as_string(&self) -> String {
        match self {
            IndexEntry::Data { len_tag, .. } => len_tag.to_string(),
        }
    }
}
impl Ord for IndexEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_len_tag_as_string().cmp(&other.get_len_tag_as_string())
    }
}
impl PartialOrd for IndexEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.get_len_tag_as_string().cmp(&other.get_len_tag_as_string()))
    }
}
impl PartialEq for IndexEntry {
    fn eq(&self, other: &Self) -> bool {
        self.get_len_tag_as_string() == other.get_len_tag_as_string()
    }
}
impl Eq for IndexEntry {}

pub struct SchemaDef<'a> {
    pub name: String,
    pub entries: Vec<IndexEntry>,
    pub rf_model: &'a RFModel,
}
impl<'a> From<&SchemaDef<'a>> for TokenStream {
    fn from(index: &SchemaDef) -> TokenStream {
        let name = format_ident!("{}Schema", index.name);
        let entries = &index.entries;

        let app_variants = index
            .rf_model
            .msg_defs
            .iter()
            .filter_map(|msg_def| {
                if matches!(msg_def.msg_category, MessageCategory::App) {
                    let name = format_ident!("{}", msg_def.name.as_str());
                    let generic_names = msg_def.generics(index.rf_model).generic_names;
                    if !generic_names.is_empty() {
                        Some(quote! (
                            #name::#generic_names::MSG_TYPE_CODE => Ok((None, Some(MsgApp::<S, C, D>::#name(#name::deserialize(deserializer)?)))
                        )))
                    } else {
                        Some(quote! (
                            #name::MSG_TYPE_CODE => Ok((None, Some(MsgApp::<S, C, D>::#name(#name::deserialize(deserializer)?)))
                        )))
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let adm_variants = index
            .rf_model
            .msg_defs
            .iter()
            .filter_map(|msg_def| {
                if matches!(msg_def.msg_category, MessageCategory::Admin) {
                    let name = format_ident!("{}", msg_def.name.as_str());
                    let generic_names = msg_def.generics(index.rf_model).generic_names;
                    if !generic_names.is_empty() {
                        // TODO dynamic generic resolution for enum which is a sum of all generics in messages
                        Some(quote! (
                            // Logon::<S, D>::MSG_TYPE_CODE      => Ok((Some(MsgAdm::<S, D>        ::Logon(Logon::deserialize(deserializer)?)), None)),
                            #name::#generic_names::MSG_TYPE_CODE => Ok((Some(MsgAdm::<S, D>::#name(#name::deserialize(deserializer)?)), None))
                        ))
                    } else {
                        Some(quote! (
                            // Logon::MSG_TYPE_CODE      => Ok((Some(MsgAdm::<S, D>        ::Logon(Logon::deserialize(deserializer)?)), None)),
                            #name::MSG_TYPE_CODE => Ok((Some(MsgAdm::<S, D>::#name(#name::deserialize(deserializer)?)), None))
                        ))
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        quote! {
            pub struct #name;
            #[allow(unused_doc_comments)]
            impl fix_model_core::prelude::Schema for #name {
                type Header<'de, S, C, D> = Header3OperationalSequence<S, D>;
                type AdmType<S, C, D> = MsgAdm<S, D>;
                type AppType<S, C, D> = MsgApp<S, C, D>;
                fn binary_data_len_pair_index() -> fix_model_core::prelude::TagTypesSorted {
                    static INDEX_PRE_SORTED_BY_TAG_LEN: fix_model_core::prelude::TagTypesSorted = &[
                        #(#entries),*
                    ];
                    INDEX_PRE_SORTED_BY_TAG_LEN
                }

                fn deserializer_msg<'de, __D, S, C, D>(
                    msg_type: &str,
                    deserializer: __D,
                ) -> std::result::Result<(Option<Self::AdmType<S, C, D>>, Option<Self::AppType<S, C, D>>), __D::Error>
                where
                    __D: serde::Deserializer<'de>,
                    S: serde::Deserialize<'de>,
                    C: serde::Deserialize<'de>,
                    D: serde::Deserialize<'de>,
                {
                    use serde::Deserialize;
                    use fix_model_core::prelude::MsgTypeCode;
                    match msg_type {
                        #(#adm_variants,)*
                        #(#app_variants,)*
                        _ => Err(serde::de::Error::custom(format!("unknown msg_type: {}", msg_type))),
                    }
                }
            }



        }
    }
}
impl<'a> ToTokens for SchemaDef<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(Into::<TokenStream>::into(self));
    }
}
