//! The Frame strut maintains information about the current frame.
//! This includes caller position, parameters, and local variables.

#[derive(Debug, Eq, PartialEq)]
pub struct Frame {
    pub caller_position: usize,
    name: String,
}

impl Frame {
    pub fn new(caller_position: usize, name: &str) -> Frame {
        Frame {
            caller_position,
            name: name.to_owned(),
        }
    }
}