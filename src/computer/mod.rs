use std::io::Write;

use value_reader::ValueReader;

use crate::common::{
    compile_time::ast_types::{ast_node::ASTNode, node_types::stmt_node::StmtNode, Root},
    error::ErrorType,
    run_time::env::Environment,
};

mod value_reader;

pub fn compute(root: Root) -> Result<(), ErrorType> {
    let environment = Environment::new(None);
    let mut reader = ValueReader::new(Box::new(environment));

    for node in root.nodes {
        match node {
            ASTNode::Stmt(stmt) => match stmt {
                StmtNode::Var(idt, exp) => {
                    let val = reader.expr(&exp)?;
                    reader.push(idt.to_string(), val.0);
                }
                StmtNode::Out(exp) => {
                    let val = reader.expr(&exp)?;
                    print!("{}", val.0.output());
                    std::io::stdout().flush().unwrap();
                }
            },
            _ => unreachable!(),
        }
    }

    return Ok(());
}
