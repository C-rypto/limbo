use std::{env, fs};

mod analyzer;
mod common;
mod computer;
mod tokenizer;

fn main() {
    let mut args = env::args();
    args.next();

    let mut src = "".to_string();
    if let Some(path) = args.next() {
        src = fs::read_to_string(path).expect("无法正常读取文件！");
    }

    let mut tokens = tokenizer::tokenize(&src);

    let root = analyzer::analyze(&mut tokens);

    computer::compute(root);
}
