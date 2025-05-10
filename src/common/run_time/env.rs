use std::collections::HashMap;

use crate::common::values::Value;

pub type IdentifTable = HashMap<String, Value>;

#[derive(Clone, PartialEq)]
pub struct Environment {
    pub prev: Option<Box<Environment>>,
    pub table: IdentifTable,
}

impl Environment {
    pub fn new(prev: Option<Box<Environment>>) -> Environment {
        Environment {
            prev,
            table: IdentifTable::new(),
        }
    }

	pub fn contains(&self, target: &String) -> bool {
		self.table.get(target) != None
	}

    pub fn insert(&mut self, idt: String, val: Value) {
        self.table.insert(idt, val);
    }

    pub fn overwrite(&mut self, idt: String, val: Value) -> bool {
        if self.contains(&idt) {
			self.insert(idt, val);
			return true;
		}
		else {
			match self.prev.as_mut() {
				Some(prev) => {
					// let mut prev = *prev;
					prev.overwrite(idt, val)
				}
				None => return false,
			}
		}
    }

    pub fn find(&self, target: &String) -> Option<Value> {
        match self.table.get(target) {
            Some(val) => Some(val.clone()),
            None => match &self.prev {
                Some(prev) => prev.find(target),
                None => None,
            },
        }
    }
}
