use crate::common::{
    ast_types::{
        ast_node::{ASTNode, ASTStream},
        node_types::{AtomNode, ExprNode},
    }, error::syntax_err, Symbol, Token, TokenStream
};

use super::term;

pub fn parse(tokens: &mut TokenStream, first: Option<Token>) -> ExprNode {
    let mut elements = ASTStream::new();

    if let Some(first) = first {
        match first {
            Token::Identif(..) | Token::Literal(..) => elements.push_back(ASTNode::from(first)),
            Token::Symbols(sym) => {
                match sym {
					Symbol::Add | Symbol::Sub => term::parse(tokens, sym),
					
				}
            }
            _ => unreachable!(),
        }
    }

	while let Some(token) = tokens.pop_front() {
		/*
		Follow:
			),
			<kwd>,
			=,
		 */
		match token {
			Token::Unknown(..) => syntax_err::report(syntax_err::unknown_token(token)),
			Token::Identif(idt) => elements.push_back(ASTNode::Expr(ExprNode::Atom(Box::new(AtomNode::Idt(idt))))),
		}
	}

    ExprNode::Expr(elements)
}
