use std::{env, fs, io::Write};

mod analyzer;
mod common;
mod tokenizer;

fn main() {
    let mut args = env::args();
    args.next();

    let mut src = "".to_string();
    if let Some(path) = args.next() {
        src = fs::read_to_string(path).expect("无法正常读取文件！");
    }

    let mut tokens = tokenizer::tokenize(&src);
    print!("tokens: \n\t");
    for token in &tokens {
        print!("{} ", token);
    }
    print!("\n");
    std::io::stdout().flush().unwrap();

    let ast_tree = analyzer::analyze(&mut tokens);
    print!(" nodes: \n\t");
    for node in ast_tree.sequence.elements {
        print!("{} ", node);
    }
    print!("\n");
    std::io::stdout().flush().unwrap();
}
