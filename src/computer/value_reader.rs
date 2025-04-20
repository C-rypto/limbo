use crate::common::{
    compile_time::ast_types::node_types::{
        expr_node::{AtomNode, CompNode, LogicNode, UnaryNode},
        AtomNodeType, ExprNode, MathNode, TermNode,
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
    pub fn new(environment: &Environment) -> ValueReader {
        let environment = Box::new(environment.clone());
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

    pub fn unary(&mut self, node: &UnaryNode) -> Result<(Value, Location), ErrorType> {
        let (value, pos) = self.atom(&node.atom)?;
        match &node.op {
            Some(oper) => {
                return Ok((oper.unary_operate(value, &pos)?, pos));
            }
            None => return Ok((value, pos)),
        }
    }

    pub fn term(&mut self, node: &TermNode) -> Result<(Value, Location), ErrorType> {
        let (left, left_pos) = self.unary(&node.left_hand)?;
        match &node.right_hand {
            Some((op, right)) => {
                let (right, right_pos) = self.term(&right)?;
                return Ok((
                    op.binary_operate(left, right, &left_pos, &right_pos)?,
                    right_pos,
                ));
            }
            None => return Ok((left, left_pos)),
        }
    }

    pub fn math(&mut self, node: &MathNode) -> Result<(Value, Location), ErrorType> {
        let (left, left_pos) = self.term(&node.left_hand)?;
        match &node.right_hand {
            Some((op, right)) => {
                let (right, right_pos) = self.math(&right)?;
                return Ok((
                    op.binary_operate(left, right, &left_pos, &right_pos)?,
                    right_pos,
                ));
            }
            None => return Ok((left, left_pos)),
        }
    }

    pub fn comp(&mut self, node: &CompNode) -> Result<(Value, Location), ErrorType> {
        let (left, left_pos) = self.math(&node.left_hand)?;
        match &node.right_hand {
            Some((op, right)) => {
                let (right, right_pos) = self.comp(&right)?;
                return Ok((
                    op.binary_operate(left, right, &left_pos, &right_pos)?,
                    right_pos,
                ));
            }
            None => return Ok((left, left_pos)),
        }
    }

    pub fn logic(&mut self, node: &LogicNode) -> Result<(Value, Location), ErrorType> {
        let (left, left_pos) = self.comp(&node.left_hand)?;
        match &node.right_hand {
            Some((op, right)) => {
                let (right, right_pos) = self.logic(&right)?;
                return Ok((
                    op.binary_operate(left, right, &left_pos, &right_pos)?,
                    right_pos,
                ));
            }
            None => return Ok((left, left_pos)),
        }
    }

    pub fn expr(&mut self, node: &ExprNode) -> Result<(Value, Location), ErrorType> {
        return self.logic(&node.inner);
    }
}
