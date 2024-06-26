use crate::schema::{
    quickfix::model::Error,
    rust::model::{message_member::RMessageMember, IsGenericMember},
};
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::{root::RFModel, GenericTypeInfo};

#[derive(Debug, PartialEq)]
pub enum MessageCategory {
    Admin,
    App,
    RepGrp,
    Header,
    Trailer,
    TagValue,
}

pub struct MessageTokenParts {
    pub msg_def: TokenStream,
    pub msg_impls: TokenStream,
}
pub struct MessageGenericParts {
    /// <A,B,C>
    pub generic_names: TokenStream,
    /// A: Serialize, B: Serialize, C: Serialize
    pub serialize_trait_bounds: TokenStream,
    /// A: Default, B: Default, C: Default
    pub default_trait_bounds: TokenStream,

    pub borrowed_asc: TokenStream,
    pub owned_asc: TokenStream,
    pub aschar: TokenStream,

    pub borrowed_str: TokenStream,
    pub owned_str: TokenStream,
    pub char_str: TokenStream,

    pub borrowed_dat: TokenStream,
    pub owned_dat: TokenStream,
}

#[derive(Debug)]
pub struct RFMessageDef {
    pub name: String,
    pub msg_type: String,
    pub msg_category: MessageCategory,
    pub members: Vec<RMessageMember>,
    pub errors: Vec<Error>,
    pub xml: String,
}
impl IsGenericMember for RFMessageDef {
    fn is_generic_string(&self) -> bool {
        self.members.iter().any(|m| m.is_generic_string())
    }
    fn is_generic_char(&self) -> bool {
        self.members.iter().any(|m| m.is_generic_char())
    }
    fn is_generic_len_data(&self) -> bool {
        self.members.iter().any(|m| m.is_generic_len_data())
    }
}
impl RFMessageDef {
    pub fn default_trait_bounds(&self, rf_model: &RFModel) -> TokenStream {
        let mut token_stream = TokenStream::new();
        for bnd in self
            .members
            .iter()
            .map(|member| member.default_trait_bounds(rf_model))
            .collect::<Vec<_>>()
        {
            token_stream.extend(bnd);
        }
        token_stream
    }
    pub fn generics(&self, rf_model: &RFModel) -> MessageGenericParts {
        let default_trait_bounds = self.default_trait_bounds(rf_model);
        // TODO remove default stiring trait bounds
        #[rustfmt::skip]
        let (generic_names, serialize_trait_bounds, default_trait_bounds) = match self.generic_info() {
            GenericTypeInfo { string: true,  chr: true,  data: true  } => ( quote!(<S,C,D>), quote!(S: serde::Serialize, C: serde::Serialize, D: serde::Serialize) , quote!(#default_trait_bounds) ),
            GenericTypeInfo { string: true,  chr: true,  data: false } => ( quote!(<S,C>),   quote!(S: serde::Serialize, C: serde::Serialize) ,                      quote!(#default_trait_bounds) ),
            GenericTypeInfo { string: true,  chr: false, data: true  } => ( quote!(<S,D>),   quote!(S: serde::Serialize, D: serde::Serialize) ,                      quote!(#default_trait_bounds) ),
            GenericTypeInfo { string: true,  chr: false, data: false } => ( quote!(<S>),     quote!(S: serde::Serialize) ,                                           quote!(#default_trait_bounds) ),
            GenericTypeInfo { string: false, chr: true,  data: true  } => ( quote!(<C,D>),   quote!(C: serde::Serialize, D: serde::Serialize) ,                      quote!(#default_trait_bounds) ),
            GenericTypeInfo { string: false, chr: true,  data: false } => ( quote!(<C>),     quote!(C: serde::Serialize) ,                                           quote!(#default_trait_bounds) ),
            GenericTypeInfo { string: false, chr: false, data: true  } => ( quote!(<D>),     quote!(D: serde::Serialize) ,                                           quote!(#default_trait_bounds) ),
            GenericTypeInfo { string: false, chr: false, data: false } => ( quote!(),        quote!() ,                                                              quote!()                      ),
        };
        #[rustfmt::skip]
        let (borrowed_asc, owned_asc, aschar, borrowed_str, owned_str, char_str , borrowed_dat, owned_dat) = match self.generic_info() {
            GenericTypeInfo { string: true,  chr: true,  data: true  } => ( quote!( &fix_model_core::prelude::asc, ), quote!( fix_model_core::prelude::Ascii, ), quote!( fix_model_core::prelude::aschar, ), quote!( &str, ), quote!( String, ), quote!( char, ), quote!( &fix_model_core::prelude::dat, ), quote!( fix_model_core::prelude::Data, ) ),
            GenericTypeInfo { string: true,  chr: true,  data: false } => ( quote!( &fix_model_core::prelude::asc, ), quote!( fix_model_core::prelude::Ascii, ), quote!( fix_model_core::prelude::aschar, ), quote!( &str, ), quote!( String, ), quote!( char, ), quote!(                                ), quote!(                                ) ),
            GenericTypeInfo { string: true,  chr: false, data: true  } => ( quote!( &fix_model_core::prelude::asc, ), quote!( fix_model_core::prelude::Ascii, ), quote!(                                  ), quote!( &str, ), quote!( String, ), quote!(       ), quote!( &fix_model_core::prelude::dat, ), quote!( fix_model_core::prelude::Data, ) ),
            GenericTypeInfo { string: true,  chr: false, data: false } => ( quote!( &fix_model_core::prelude::asc, ), quote!( fix_model_core::prelude::Ascii, ), quote!(                                  ), quote!( &str, ), quote!( String, ), quote!(       ), quote!(                                ), quote!(                                ) ),
            GenericTypeInfo { string: false, chr: true,  data: true  } => ( quote!(                                ), quote!(                                 ), quote!( fix_model_core::prelude::aschar, ), quote!(       ), quote!(         ), quote!( char, ), quote!( &fix_model_core::prelude::dat, ), quote!( fix_model_core::prelude::Data, ) ),
            GenericTypeInfo { string: false, chr: true,  data: false } => ( quote!(                                ), quote!(                                 ), quote!( fix_model_core::prelude::aschar, ), quote!(       ), quote!(         ), quote!( char, ), quote!(                                ), quote!(                                ) ),
            GenericTypeInfo { string: false, chr: false, data: true  } => ( quote!(                                ), quote!(                                 ), quote!(                                  ), quote!(       ), quote!(         ), quote!(       ), quote!( &fix_model_core::prelude::dat, ), quote!( fix_model_core::prelude::Data, ) ),
            GenericTypeInfo { string: false, chr: false, data: false } => ( quote!(                                ), quote!(                                 ), quote!(                                  ), quote!(       ), quote!(         ), quote!(       ), quote!(                                ), quote!(                                ) ),
        };
        MessageGenericParts {
            generic_names,
            serialize_trait_bounds,
            default_trait_bounds,

            borrowed_asc,
            owned_asc,
            aschar,

            borrowed_str,
            owned_str,
            char_str,

            borrowed_dat,
            owned_dat,
        }
    }
}
impl From<(&RFMessageDef, &RFModel)> for MessageTokenParts {
    fn from(value: (&RFMessageDef, &RFModel)) -> Self {
        let (r_msg_def, rf_model) = value;
        let name = format_ident!("{}", &r_msg_def.name);
        let msg_type = &r_msg_def.msg_type;
        let members = &r_msg_def.members;

        let generics = r_msg_def.generics(rf_model);

        let (generic_names, serialize_trait_bounds, default_trait_bounds) =
            (&generics.generic_names, &generics.serialize_trait_bounds, &generics.default_trait_bounds);

        let owned_asc = &generics.owned_asc;
        let borrowed_asc = &generics.borrowed_asc;
        let aschar = &generics.aschar;

        let owned_str = &generics.owned_str;
        let borrowed_str = &generics.borrowed_str;
        let char_str = &generics.char_str;

        let owned_dat = &generics.owned_dat;
        let borrowed_dat = &generics.borrowed_dat;

        let serialize_field_fix = members.iter().map(|m| m.serialize_field(false)).collect::<Vec<_>>();
        let serialize_field_json = members.iter().map(|m| m.serialize_field(true)).collect::<Vec<_>>();
        let member_len = members.iter().map(|m| m.member_len()).collect::<Vec<_>>();
        let member_to_owned_inner_if_ref = members.iter().map(|m| m.to_owned_inner_if_ref()).collect::<Vec<_>>();
        let member_names = members
            .iter()
            .map(|m| {
                let name = format_ident!("r#{}", m.name().to_case(Case::Snake));
                quote!(#name)
            })
            .collect::<Vec<_>>();

        let mut xml = r_msg_def
            .xml
            .replace("><group", ">\n    <group")
            .replace("><field", ">\n    <field")
            .replace("><component", ">\n    <component")
            .replace("</component>", "\n</component>")
            .replace("</message>", "\n</message>");
        if xml.starts_with("<group name=") {
            xml = xml.replace("</group>", "\n</group>");
        } else if xml.starts_with("<component name=") {
            xml = xml.replace("</group>", "\n    </group>");
        }
        let doc = format!("# Defition:\n```xml\n{}\n```", xml);

        let msg_def = quote!(
            #[doc = #doc]
            #[derive(serde::Deserialize, Debug, PartialEq, Clone)]
            pub struct #name #generic_names{
                #(#members)*
            }
        );

        let mut msg_impls = quote!();
        msg_impls.extend(quote!(
            #[automatically_derived]
            impl #name < #borrowed_asc #aschar #borrowed_dat > {
               pub fn to_owned_inner_if_ref(&self) -> #name < #owned_asc #aschar #owned_dat > {
                    #name {
                        #(#member_to_owned_inner_if_ref )*
                    }
                }
            }
        ));
        if !borrowed_asc.is_empty() || !aschar.is_empty() {
            msg_impls.extend(quote!(
                #[automatically_derived]
                impl #name < #borrowed_str #char_str #borrowed_dat > {
                    pub fn to_owned_inner_if_ref(&self) -> #name < #owned_str #char_str #owned_dat > {
                         #name {
                             #(#member_to_owned_inner_if_ref )*
                         }
                     }
                 }
            ));
        }
        msg_impls.extend(quote!(
            #[automatically_derived]
            impl #generic_names std::default::Default for #name #generic_names where #default_trait_bounds {
               fn default() -> Self {
                    Self {
                        #(#member_names: Default::default() ,)*
                    }
                }
            }
        ));

        msg_impls.extend(quote!(
            #[allow(unused_imports)]
            #[automatically_derived]
            impl #generic_names serde::Serialize for #name #generic_names where #serialize_trait_bounds  {
                fn serialize<__S: serde::Serializer>(&self, __serializer: __S,) -> serde::__private::Result<__S::Ok, __S::Error>{
                    if __serializer.is_human_readable() {
                        use serde::ser::SerializeStruct;
                        let mut __serde_state = serde::Serializer::serialize_struct(
                            __serializer,
                            stringify!(#name),
                            false as usize
                                #(+ #member_len )*
                        )?;
                        #(#serialize_field_json)* // for json it expects to know number of members to be serialized to close the bracker correctly
                        serde::ser::SerializeStruct::end(__serde_state)
                    } else {
                        use serde::ser::SerializeStruct;
                        let mut __serde_state = serde::Serializer::serialize_struct(
                            __serializer,
                            stringify!(#name),
                            false as usize
                        )?;
                        #(#serialize_field_fix)*
                        serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            }
        ));

        match r_msg_def.msg_category {
            MessageCategory::Admin | MessageCategory::App => {
                let is_app = matches!(r_msg_def.msg_category, MessageCategory::App);
                msg_impls.extend(quote!(
                    impl #generic_names fix_model_core::prelude::MsgTypeCode for #name #generic_names {
                        const MSG_TYPE_CODE: &'static str = #msg_type;
                        #[inline]
                        fn is_app(&self) -> bool {
                            #is_app
                        }
                    }
                ));
            }
            MessageCategory::Header => {
                msg_impls.extend(quote!(
                    impl #generic_names fix_model_core::prelude::Header for #name #generic_names where #serialize_trait_bounds {}
                ));
            }
            _ => {}
        }

        MessageTokenParts { msg_def, msg_impls }
    }
}
