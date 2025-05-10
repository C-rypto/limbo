use crate::common::{
    compile_time::ast_types::{
        node_types::{
            expr_node::{AtomNode, CompNode, LogicNode, UnaryNode},
            stmt_node::StmtNode,
            AtomType, BlockNode, ExprNode, MathNode, TermNode,
        },
        Root, ToRoot,
    },
    error::{ErrorType, RuntimeErr},
    run_time::env::Environment,
    utils::Location,
    values::Value,
};

pub struct ValueReader {
    pub env: Environment,
    pub saved_env: Option<Environment>,
}

impl ValueReader {
    pub fn new(prev_env: Option<Box<Environment>>) -> ValueReader {
        ValueReader {
            env: Environment::new(prev_env),
            saved_env: None,
        }
    }

    // 功能函数
    pub fn run(&mut self, root: Root) -> Result<Option<(Value, Location)>, ErrorType> {
        // 运行
        for node in root.nodes {
            match node {
                StmtNode::Assign { inner } => {
					let (expr_value, _) = self.expr(&inner.value)?;
                    if !self.env.overwrite(inner.name.to_string(), expr_value) {
						return Err(RuntimeErr::Undeclared(inner).into());
					}
                }
                StmtNode::Var { inner } => {
                    let (expr_value, _) = self.expr(&inner.value)?;
                    self.assign(inner.name.to_string(), expr_value);
                }
                StmtNode::Out { inner } => {
                    let value = self.expr(&inner)?;
                    return Ok(Some(value));
                }
                StmtNode::Block { inner } => {
                    self.block(&inner)?;
                }
                StmtNode::IfElse {
                    cond,
                    if_stmt,
                    else_stmt,
                } => {
                    let (cond, _) = self.expr(&cond)?;
                    if cond.boolean() {
                        self.run(if_stmt.to_root())?;
                    } else {
                        if let Some(else_stmt) = else_stmt {
                            self.run(else_stmt.to_root())?;
                        }
                    }
                }
            }
        }
        return Ok(None);
    }

    pub fn assign(&mut self, idt: String, val: Value) {
        // 赋值
        self.env.insert(idt, val);
    }

    fn begin_scope(&mut self) -> Result<(), ErrorType> {
        // 进入作用域并切换上下文
        self.env = Environment::new(Some(Box::new(self.env.clone())));
        Ok(())
    }

    fn end_scope(&mut self) -> Result<(), ErrorType> {
        // 结束作用域并还原上下文
        match &self.env.prev {
            Some(env) => self.env = *env.clone(),
            None => unreachable!(),
        }
        Ok(())
    }

    // 解析函数
    fn block(&mut self, node: &BlockNode) -> Result<Option<(Value, Location)>, ErrorType> {
        // 要先把block中的内容执行之后，在block的作用域内完成expr的解析

        // 1.修改上下文
        self.begin_scope()?;

        // 2.执行block
        let value = self.run(node.seq.clone().to_root())?;

        // 3.还原上下文
        self.end_scope()?;

        // 4. 返回可能存在的返回值
        return Ok(value);
    }

    fn atom(&mut self, node: &AtomNode) -> Result<(Value, Location), ErrorType> {
        // 原子值
        match &node.node_type {
            AtomType::Val(val) => return Ok((val.clone(), node.pos.clone())),
            AtomType::Idt(idt) => match self.env.find(idt) {
                Some(val) => return Ok((val, node.pos.clone())),
                None => return Err(RuntimeErr::Undeclared(Box::new(node.clone())).into()),
            },
            AtomType::Expr(exp) => return self.expr(&exp),
            AtomType::Block(blc) => match self.block(&blc)? {
                Some(value) => Ok(value),
                None => todo!(),
            },
        }
    }

    fn unary(&mut self, node: &UnaryNode) -> Result<(Value, Location), ErrorType> {
        // 运算符：负号, !
        let (value, pos) = self.atom(&node.atom)?;
        match &node.op {
            Some(oper) => {
                return Ok((oper.unary_operate(value, &pos)?, pos));
            }
            None => return Ok((value, pos)),
        }
    }

    fn term(&mut self, node: &TermNode) -> Result<(Value, Location), ErrorType> {
        // 运算符：*, /
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

    fn math(&mut self, node: &MathNode) -> Result<(Value, Location), ErrorType> {
        // 运算符：+, -
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

    fn comp(&mut self, node: &CompNode) -> Result<(Value, Location), ErrorType> {
        // 运算符：<, >, <=, >=, ==, !=
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

    fn logic(&mut self, node: &LogicNode) -> Result<(Value, Location), ErrorType> {
        // 运算符：&&, ||
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

    fn expr(&mut self, node: &ExprNode) -> Result<(Value, Location), ErrorType> {
        // 为了方便拓展，所以包装一下
        return self.logic(&node.inner);
    }
}
