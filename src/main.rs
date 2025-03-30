use std::{env, fs, io::Write};

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
    print!("tokens: \n ");
    for token in &tokens {
        print!("{} ", token);
    }
    print!("\n");
    std::io::stdout().flush().unwrap();

    let root = analyzer::analyze(&mut tokens);
    for node in root.nodes {
        println!("{}", node);
    }
}
