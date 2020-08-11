use crate::utils::parameter::Parameter;

#[derive(Debug)]
pub struct Label {
    pub start_pos: usize,
    pub end_pos: usize,
    parameters: Vec<Parameter>,
}

impl Label {
    pub fn new(start_pos: usize, end_pos: usize, parameters: Vec<Parameter>) -> Label {
        Label {
            start_pos,
            end_pos,
            parameters,
        }
    }
}