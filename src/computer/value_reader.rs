use crate::common::{
    compile_time::ast_types::node_types::{
        expr_node::AtomNode, AtomNodeType, ExprNode, MathExprNode, TermNode,
    },
    error::{ErrorType, RuntimeErr},
    run_time::env::Environment,
    utils::Location,
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

    pub fn atom(&mut self, node: &AtomNode) -> Result<(Value, Location), ErrorType> {
        match &node.node_type {
            AtomNodeType::Val(val) => return Ok((val.clone(), node.pos.clone())),
            AtomNodeType::Idt(idt) => match self.environment.find(idt) {
                Some(val) => return Ok((val, node.pos.clone())),
                None => return Err(RuntimeErr::Undeclared(Box::new(node.clone())).into()),
            },
            AtomNodeType::Expr(exp) => return self.expr(&exp),
        }
    }

    pub fn term(&mut self, node: &TermNode) -> Result<(Value, Location), ErrorType> {
        let (left, left_pos) = self.atom(&node.left_hand)?;
        match &node.right_hand {
            Some((op, right)) => {
                let (right, right_pos) = self.term(&right)?;
                return Ok((op.binary_operate(left, right, &left_pos ,&right_pos)?, right_pos));
            }
            None => return Ok((left, left_pos)),
        }
    }

    pub fn math_expr(&mut self, node: &MathExprNode) -> Result<(Value, Location), ErrorType> {
        let (left, left_pos) = self.term(&node.left_hand)?;
        match &node.right_hand {
            Some((op, right)) => {
                let (right, right_pos) = self.expr(&right)?;
                return Ok((op.binary_operate(left, right, &left_pos, &right_pos)?, right_pos));
            }
            None => return Ok((left, left_pos)),
        }
    }

    pub fn expr(&mut self, node: &ExprNode) -> Result<(Value, Location), ErrorType> {
        match &node {
            ExprNode::Math(math_exp) => self.math_expr(math_exp),
        }
    }
}
