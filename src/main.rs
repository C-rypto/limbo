#![cfg_attr(debug_assertions, allow(dead_code))]

use std::env;

use common::error;

mod analyzer;
mod common;
mod computer;
mod tokenizer;

fn main() {
    let mut args = env::args();
    args.next();

    if let Some(path) = args.next() {
        let mut tokens: common::TokenStream = error::unwrap(tokenizer::tokenize(&path));

        let root = error::unwrap(analyzer::analyze(&mut tokens));

        error::unwrap(computer::compute(root, None));
    }
}
