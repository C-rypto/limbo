use std::env;

mod analyzer;
mod common;
mod computer;
mod tokenizer;

fn main() {
    let mut args = env::args();
    args.next();

    if let Some(path) = args.next() {
        let mut tokens = tokenizer::tokenize(&path);
		
        let root = analyzer::analyze(&mut tokens);

        computer::compute(root);
    }
}
