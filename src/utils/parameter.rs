#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    pub pos: usize,
    pub name: String,
}

impl Parameter {
    pub fn new(pos: usize, name: String) -> Parameter {
        Parameter { pos, name }
    }
}
