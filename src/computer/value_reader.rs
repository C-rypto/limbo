use crate::{
    common::{
        compile_time::ast_types::node_types::{AtomNode, ExprNode, MathExprNode, TermNode},
        error::undeclared,
        run_time::env::Environment,
        values::Value,
    },
    runtime_err,
};

pub struct ValueReader {
    pub environment: Box<Environment>,
}

impl ValueReader {
    pub fn new(environment: Box<Environment>) -> ValueReader {
        return ValueReader { environment };
    }

    pub fn push(&mut self, idt: String, val: Value) {
        self.environment.push(idt, val);
    }

    pub fn atom(&mut self, node: &AtomNode) -> Value {
        match &node {
            AtomNode::Val(val) => val.clone(),
            AtomNode::Idt(idt) => match self.environment.find(idt) {
                Some(val) => val,
                None => runtime_err!(undeclared(idt)),
            },
            AtomNode::Expr(exp) => self.expr(&exp),
        }
    }

    pub fn term(&mut self, node: &TermNode) -> Value {
        let left = self.atom(&node.left_hand);
        match &node.right_hand {
            Some((op, right)) => op.binary_operate(left, self.term(&right)),
            None => left,
        }
    }

    pub fn math_expr(&mut self, node: &MathExprNode) -> Value {
        let left = self.term(&node.left_hand);
        match &node.right_hand {
            Some((op, right)) => op.binary_operate(left, self.expr(&right)),
            None => left,
        }
    }

    pub fn expr(&mut self, node: &ExprNode) -> Value {
        match &node {
            ExprNode::Math(math_exp) => self.math_expr(math_exp),
        }
    }
}
