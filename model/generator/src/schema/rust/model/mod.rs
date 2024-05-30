use std::iter::Sum;

pub mod field;
pub mod schema;
pub mod message;
pub mod message_member;
pub mod repeating_group;
pub mod root;

fn format_token_stream(ts: &proc_macro2::TokenStream) -> String {
    let file = syn::parse_file(&ts.to_string()).unwrap();
    prettyplease::unparse(&file)
    
    // match syn::parse_file(&ts.to_string()) {
    //     Ok(file) => prettyplease::unparse(&file),
    //     Err(e) => {
    //         // println!("Error: {:?}", e);
    //         ts.to_string()
    //     }
    // }
}

pub trait IsGenericMember {
    fn is_generic_string(&self) -> bool;
    fn is_generic_char(&self) -> bool;
    fn is_generic_len_data(&self) -> bool;
    fn generic_info(&self) -> GenericTypeInfo {
        GenericTypeInfo {
            string: self.is_generic_string(),
            chr: self.is_generic_char(),
            data: self.is_generic_len_data(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GenericTypeInfo {
    pub string: bool,
    pub chr: bool,
    pub data: bool,
}
impl IsGenericMember for GenericTypeInfo {
    fn is_generic_string(&self) -> bool {
        self.string
    }
    fn is_generic_char(&self) -> bool {
        self.chr
    }
    fn is_generic_len_data(&self) -> bool {
        self.data
    }
}
impl Sum for GenericTypeInfo {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(
            GenericTypeInfo {
                string: false,
                chr: false,
                data: false,
            },
            |acc, x| GenericTypeInfo {
                string: acc.string || x.string,
                chr: acc.chr || x.chr,
                data: acc.data || x.data,
            },
        )
    }
}
