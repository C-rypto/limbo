use crate::common::values::Value;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(Clone, PartialEq)]
pub struct Location(pub String, pub u32, pub u32);

impl From<(String, u32, u32)> for Location {
    fn from(value: (String, u32, u32)) -> Self {
        return Self(value.0, value.1, value.2);
    }
}

pub trait Locatable: ToString {
    fn locate(&self) -> String;
}

impl Locatable for Location {
    fn locate(&self) -> String {
        return self.to_string();
    }
}

impl core::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.0, self.1, self.2)
    }
}

pub struct LocatableValue(pub Value, pub Location);

impl Locatable for LocatableValue {
    fn locate(&self) -> String {
        return format!("{}", self.1);
    }
}

impl core::fmt::Display for LocatableValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return self.0.fmt(f);
    }
}
