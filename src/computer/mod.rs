use crate::common::{
    compile_time::ast_types::{ast_node::ASTNode, node_types::StmtNode, Root},
    run_time::env::Environment,
};

pub fn compute(root: Root) {
    let mut env = Environment::new(None);

    for node in root.nodes {
        match node {
            ASTNode::Stmt(stmt) => match stmt {
                StmtNode::Var(idt, val) => todo!(),
                StmtNode::Out(val) => todo!(),
            },
            _ => unreachable!(),
        }
    }
}
