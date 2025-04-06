use crate::common::values::Value;

pub type Location = (String, u32, u32);

pub trait Locatable: ToString {
    fn locate(&self) -> String;
}

pub struct LocatableValue(pub Value, pub Location);

impl Locatable for LocatableValue {
    fn locate(&self) -> String {
        return format!("{}:{}:{}", self.1 .0, self.1 .1, self.1 .2);
    }
}

impl core::fmt::Display for LocatableValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return self.0.fmt(f);
    }
}
