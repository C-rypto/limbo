pub mod ast_types;
pub mod token_types;
pub mod values;

pub mod error;

pub use token_types::*;

pub trait Stream {
    fn match_next(&mut self, mark: &'static str);
}
