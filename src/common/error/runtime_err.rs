use crate::common::utils::indent;

use crate::common::utils::Locatable;

pub enum RuntimeErr {
    Undeclared(Box<dyn Locatable>),
    TypeError(Box<dyn Locatable>, Box<dyn Locatable>),
}

impl core::fmt::Display for RuntimeErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Undeclared(sth) => write!(
                f,
                "`{}`\n{}`{}` is undeclared.",
                sth.locate(),
				indent(1),
                sth.to_string(),
            ),
            Self::TypeError(left, right) => write!(
				f,
				"`{}` and `{}`\n{}`{}` and `{}` can not operated.",
				left.locate(),
				right.locate(),
				indent(1),
				left.to_string(),
				right.to_string(),
			),
        }
    }
}
