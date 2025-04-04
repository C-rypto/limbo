use crate::common::{Symbol, Token, TokenStream};

use super::node_types::{
    AtomNode, ExprNode, MathExprNode, MathExprRest, StmtNode, TermNode, TermRest,
};

pub trait Synchronous {
    fn synchronize(&self) -> TokenStream;
}

impl Synchronous for StmtNode {
    fn synchronize(&self) -> TokenStream {
        let synch = vec![Token::EOL];
        return TokenStream::from(synch);
    }
}

impl Synchronous for ExprNode {
    fn synchronize(&self) -> TokenStream {
        match self {
            Self::Math(math_expr) => math_expr.synchronize(),
        }
    }
}

impl Synchronous for MathExprNode {
    fn synchronize(&self) -> TokenStream {
        match &self.right_hand {
            Some((.., right)) => right.synchronize(),
            None => self.left_hand.synchronize(),
        }
    }
}

impl Synchronous for MathExprRest {
    fn synchronize(&self) -> TokenStream {
        let synch = vec![Token::Symbols(Symbol::RParen), Token::EOL];

        return TokenStream::from(synch);
    }
}

impl Synchronous for TermNode {
    fn synchronize(&self) -> TokenStream {
        match &self.right_hand {
            Some((.., right)) => right.synchronize(),
            None => self.left_hand.synchronize(),
        }
    }
}

impl Synchronous for TermRest {
    fn synchronize(&self) -> TokenStream {
        let synch = vec![Token::Symbols(Symbol::RParen), Token::EOL];

        return TokenStream::from(synch);
    }
}

impl Synchronous for AtomNode {
    fn synchronize(&self) -> TokenStream {
        let synch = vec![
            Token::Symbols(Symbol::Add),
            Token::Symbols(Symbol::Sub),
            Token::Symbols(Symbol::Mul),
            Token::Symbols(Symbol::Div),
            Token::EOL,
        ];

        return TokenStream::from(synch);
    }
}
