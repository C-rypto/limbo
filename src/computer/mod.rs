use std::io::Write;

use value_reader::ValueReader;

use crate::common::{
    compile_time::ast_types::{node_types::stmt_node::StmtNode, Root},
    error::{ErrorType, RuntimeErr},
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
            // StmtNode::IfElse {
            //     cond,
            //     if_seq,
            //     else_seq,
            // } => {
            //     let cond_val = reader.expr(&cond)?;
            //     if cond_val.0.is_boolean() {
            //         if cond_val.0.boolean() {
            //             return compute(Root::new(if_seq), Some(Box::new(environment.clone())));
            //         } else {
            //             match else_seq {
            //                 Some(else_seq) => {
            //                     return compute(
            //                         Root::new(else_seq),
            //                         Some(Box::new(environment.clone())),
            //                     )
            //                 }
            //                 None => continue,
            //             }
            //         }
            //     } else {
            //         return Err(RuntimeErr::TypeError(cond, None).into());
            //     }
            // }
        }
    }

    return Ok(());
}
