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

pub struct SchemaDef {
    pub name: String,
    pub entries: Vec<IndexEntry>,
}
impl From<&SchemaDef> for TokenStream {
    fn from(index: &SchemaDef) -> TokenStream {
        let name = format_ident!("{}Schema", index.name);
        let entries = &index.entries;
        quote! {
            pub struct #name;
            #[allow(unused_doc_comments)]
            impl fix_model_core::prelude::Schema for #name {
                type Header<'de, S: serde::Deserialize<'de> + serde::Serialize, C: serde::Deserialize<'de> + serde::Serialize, D: serde::Deserialize<'de> + serde::Serialize> = Header3OperationalSequence<S, D>;
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
                    match msg_type {
                        "A" => Ok((Some(MsgAdm::<S, D>::Logon(Logon::deserialize(deserializer)?)), None)),
                        _ => Err(serde::de::Error::custom(format!("unknown msg_type: {}", msg_type))),
                    }
                }
            }



        }
    }
}
impl ToTokens for SchemaDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(Into::<TokenStream>::into(self));
    }
}
