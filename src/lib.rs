#![doc = include_str!("../README.md")]

pub mod proto;

#[cfg(feature = "svr")]
pub mod svr;

#[cfg(feature = "clt")]
pub mod clt;


// pub const KRPC_APP_NAME : &'static str = option_env!("APP_NAME");


// #[cfg(test)]
// mod tests {
//     use super::*;

//     // krpc_app_name!("123");
//     pub fn add(left: u64, right: u64) -> u64 {
//         left + right
//     }

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
