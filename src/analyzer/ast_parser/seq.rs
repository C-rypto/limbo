use crate::{
    analyzer::ast_parser::{expr, stmt},
    common::{
        ast_types::{
            ast_node::{ASTNode, ASTStream},
            node_types::SeqNode,
        },
        error::syntax_err,
        Token, TokenStream,
    },
};

pub fn parse(tokens: &mut TokenStream) -> SeqNode {
    let mut stream = ASTStream::new();

    while let Some(token) = tokens.pop_front() {
        match token {
            Token::Unknown(..) => syntax_err::report(syntax_err::unknown_token(token)),
            Token::Keyword(kwd) => stream.push_back(ASTNode::Stmt(stmt::parse(tokens, kwd))),
            _ => stream.push_back(ASTNode::Expr(expr::parse(tokens, Some(token)))),
        }
    }

    SeqNode { elements: stream }
}
