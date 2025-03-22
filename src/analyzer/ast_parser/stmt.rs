use crate::{analyzer::ast_parser::expr, common::{ast_types::node_types::StmtNode, Keyword, Stream, TokenStream}};

pub fn parse(tokens: &mut TokenStream, keyword: Keyword) -> StmtNode {
    match keyword {
        Keyword::Var => {
            tokens.match_next("idt");tokens.match_next("=");expr::parse(tokens);
        }
        Keyword::Out => {
			expr::parse(tokens);
		}
    }
    todo!()
}
