use std::io::Write;

use value_reader::ValueReader;

use crate::common::{
    compile_time::ast_types::{node_types::stmt_node::StmtNode, Root},
    error::ErrorType,
    run_time::env::Environment,
};

mod value_reader;

pub fn compute(root: Root, prev_env: Option<Box<Environment>>) -> Result<(), ErrorType> {
    let environment = Environment::new(prev_env);
    let mut reader = ValueReader::new(&environment);

    for node in root.nodes {
        match node {
            StmtNode::Var { name, value } => {
                let val = reader.expr(&value)?;
                reader.push(name.to_string(), val.0);
            }
            StmtNode::Out { value } => {
                let val = reader.expr(&value)?;
                print!("{}", val.0.output());
                std::io::stdout().flush().unwrap();
            }
			StmtNode::Block { value } => {
				let temp_root = Root::new(*value);
				compute(temp_root, Some(Box::new(environment.clone())))?;
			}
        }
    }

    return Ok(());
}
