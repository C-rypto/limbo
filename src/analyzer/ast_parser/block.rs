use crate::common::{
    compile_time::ast_types::{
        node_types::{stmt_node::StmtNode, BlockNode},
        Sequence,
    },
    error::{CompileErr, ErrorType},
    Symbol, TokenStream, TokenTag,
};

use super::stmt;

pub fn parse(tokens: &mut TokenStream) -> Result<BlockNode, ErrorType> {
    let mut seq = Sequence::new();
    let mut has_output = false;

    // tokens.skip_white_space();
    while let Some(next) = tokens.next() {
        if next.get_tag() == TokenTag::Symbols(Symbol::RBrace) {
            break;
        }

        let stmt = stmt::parse(tokens, next)?;
        if let StmtNode::Out { .. } = &stmt {
            if has_output {
                return Err(CompileErr::ExcessOut(Box::new(stmt)).into());
            }
            has_output = true;
        }

        seq.push_back(stmt);
    }

    return Ok(BlockNode { seq: Box::new(seq) });
}
