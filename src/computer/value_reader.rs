use crate::common::{
    compile_time::ast_types::{node_types::{
        expr_node::{AtomNode, CompNode, LogicNode, UnaryNode}, stmt_node::StmtNode, AtomType, BlockNode, ExprNode, MathNode, TermNode
    }, Root},
    error::{ErrorType, RuntimeErr},
    run_time::env::Environment,
    utils::Location,
    values::Value,
};

pub struct ValueReader {
    pub env: Environment,
}

impl ValueReader {
    pub fn new(prev_env: Option<Box<Environment>>) -> ValueReader {
		ValueReader { env: Environment::new(prev_env) }
    }

	pub fn run(&mut self, root: Root) -> Result<Option<(Value, Location)>, ErrorType> {
		for node in root.nodes {
			match node {
				StmtNode::Var { name, value } => {
					let value = self.expr(&value)?;
					self.push(name.to_string(), value.0);
				}
				StmtNode::Out { value } => {
					let value = self.expr(&value)?;
					return Ok(Some(value));
				}
				StmtNode::Block { value } => {
					self.block(&value)?;
				}
			}
		}
		return Ok(None);
	}

    pub fn push(&mut self, idt: String, val: Value) {
        self.env.push(idt, val);
    }

    pub fn atom(&mut self, node: &AtomNode) -> Result<(Value, Location), ErrorType> {
        match &node.node_type {
            AtomType::Val(val) => return Ok((val.clone(), node.pos.clone())),
            AtomType::Idt(idt) => match self.env.find(idt) {
                Some(val) => return Ok((val, node.pos.clone())),
                None => return Err(RuntimeErr::Undeclared(Box::new(node.clone())).into()),
            },
            AtomType::Expr(exp) => return self.expr(&exp),
			AtomType::Block(blc) => match self.block(&blc)? {
				Some(value) => Ok(value),
				None => todo!()
			},
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

    pub fn block(&mut self, node: &BlockNode) -> Result<Option<(Value, Location)>, ErrorType> {
		// 要先把block中的内容执行之后，在block的作用域内完成expr的解析

		// 1.修改上下文
		self.env = Environment::new(Some(Box::new(self.env.clone())));

		// 2.执行block
		let temp_root = Root::new(*node.seq.clone());
		let value = self.run(temp_root)?;

		// 3.还原上下文
		match &self.env.prev {
			Some(env) => self.env = *env.clone(),
			None => unreachable!()
		}
		return Ok(value);
    }
}
