use crate::common::{
    compile_time::ast_types::node_types::{
        expr_node::AtomNode, AtomNodeType, ExprNode, MathExprNode, TermNode,
    },
    error::{ErrorType, RuntimeErr},
    run_time::env::Environment,
    values::Value,
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

    pub fn atom(&mut self, node: &AtomNode) -> Result<Value, ErrorType> {
        match &node.node_type {
            AtomNodeType::Val(val) => Ok(val.clone()),
            AtomNodeType::Idt(idt) => match self.environment.find(idt) {
                Some(val) => Ok(val),
                None => return Err(RuntimeErr::Undeclared(Box::new(node.clone())).into()),
            },
            AtomNodeType::Expr(exp) => self.expr(&exp),
        }
    }

    pub fn term(&mut self, node: &TermNode) -> Result<Value, ErrorType> {
        let left = self.atom(&node.left_hand)?;
        match &node.right_hand {
            Some((op, right)) => Ok(op.binary_operate(left, self.term(&right)?)),
            None => Ok(left),
        }
    }

    pub fn math_expr(&mut self, node: &MathExprNode) -> Result<Value, ErrorType> {
        let left = self.term(&node.left_hand)?;
        match &node.right_hand {
            Some((op, right)) => Ok(op.binary_operate(left, self.expr(&right)?)),
            None => Ok(left),
        }
    }

    pub fn expr(&mut self, node: &ExprNode) -> Result<Value, ErrorType> {
        match &node {
            ExprNode::Math(math_exp) => self.math_expr(math_exp),
        }
    }
}
