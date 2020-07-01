//! The Frame strut maintains information about the current frame.
//! This includes caller position, parameters, and local variables.

#[derive(Debug, Eq, PartialEq)]
pub struct Frame {
    caller_position: usize,
    name: String,
}

impl Frame {
    /// Constructs a new frame.
    /// A Frame maintains the caller's position, along with its name.
    /// In the future, it will maintain local variables and any parameters passed in.
    ///
    /// # Arguments
    /// `caller_position` - The position where this frame was called or entered.
    /// `name` - The name of this frame.
    pub fn new(caller_position: usize, name: &str) -> Frame {
        Frame {
            caller_position,
            name: name.to_owned(),
        }
    }

    /// This function gets the position of the caller of this frame.
    pub fn get_caller_position(&self) -> usize {
        self.caller_position
    }
}
