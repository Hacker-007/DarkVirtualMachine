use crate::values::value::Value;

#[derive(Debug, PartialEq)]
pub struct Parameter {
    pos: usize,
    name: String,
    value: Value,
}

impl Parameter {
    pub fn new(pos: usize, name: String, value: Value) -> Parameter {
        Parameter {
            pos,
            name,
            value,
        }
    }
}