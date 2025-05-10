use crate::common::utils::{Locatable, Location};

use super::ExprNode;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq)]
pub struct AssignNode {
    pub name: String,
    pub value: Box<ExprNode>,
    pub pos: Location,
}

impl Locatable for AssignNode {
    fn locate(&self) -> String {
        return format!("{}", self.pos);
    }
}

impl core::fmt::Display for AssignNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.name, self.value)
    }
}
