// use super::macros::FixString;

// #[derive(Debug, Clone)]
// pub struct RawFrame<const MAX_MSG_SIZE: usize> {
//     buf: [u8; MAX_MSG_SIZE],
//     end: usize,
// }
// impl<const MAX_MSG_SIZE: usize> Default for RawFrame<MAX_MSG_SIZE> {
//     fn default() -> Self {
//         Self {
//             buf: [0; MAX_MSG_SIZE],
//             end: 0,
//         }
//     }
// }


// #[cfg(test)]
// mod tests {
//     use crate::schema::rust::macros;


//     #[test]
//     fn test_account1() {
//         use crate::fix_string;
//         fix_string!(Account, 1);
//         let a = Account::new("value".to_string());
//         println!("a: {:?}", a);
//         println!("a: {}", a);
//         println!("a: {:+}", a);
//         println!("a: {:-}", a);
        
//     }

// }
