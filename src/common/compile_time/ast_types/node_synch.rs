use crate::common::{Symbol, Token, TokenStream};

use super::node_types::{
    AtomNode, ExprNode, MathExprNode, MathExprRest, StmtNode, TermNode, TermRest,
};

pub trait Synchronous {
    fn synchronize() -> TokenStream;
}

impl Synchronous for StmtNode {
    fn synchronize() -> TokenStream {
        todo!()
    }
}

impl Synchronous for ExprNode {
    fn synchronize() -> TokenStream {
        todo!()
    }
}

impl Synchronous for MathExprNode {
    fn synchronize() -> TokenStream {
    	todo!()
    }
}

impl Synchronous for MathExprRest {
    fn synchronize() -> TokenStream {
        todo!()
    }
}

impl Synchronous for TermNode {
    fn synchronize() -> TokenStream {
        todo!()
    }
}

impl Synchronous for TermRest {
    fn synchronize() -> TokenStream {
        todo!()
    }
}

impl Synchronous for AtomNode {
    fn synchronize() -> TokenStream {
        todo!()
    }
}
