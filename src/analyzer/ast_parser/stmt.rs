use crate::{
    analyzer::ast_parser::expr,
    common::{ast_types::node_types::StmtNode, Keyword, Stream, TokenStream},
};

pub fn parse(tokens: &mut TokenStream, keyword: Keyword) -> StmtNode {
    match keyword {
        Keyword::Var => {
            let idt = tokens.match_next("idt");
            tokens.match_next("=");
            expr::parse(tokens, None);
            StmtNode::Var(idt.to_string())
        }
        Keyword::Out => StmtNode::Out(Box::new(expr::parse(tokens, None))),
    }
}
