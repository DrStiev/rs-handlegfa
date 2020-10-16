extern crate nom;
extern crate regex;

pub mod gfa;
pub mod gfa2;
pub mod parser_gfa;
pub mod parser_gfa2;

#[path = "error/error.rs"]
pub mod error;
#[path = "test/test.rs"]
pub mod test; 