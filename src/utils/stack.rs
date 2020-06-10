use std::fmt::Debug;
use super::error::{ErrorKind, Error};

#[derive(Debug)]
pub struct Stack<T: Debug>(Vec<T>);

impl<T: Debug> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack(vec![])
    }

    pub fn push(&mut self, value: T) {
        self.0.push(value)
    }

    pub fn pop(&mut self, pos: usize) -> Result<T, Error> {
        self.0.pop().ok_or(Error::new(ErrorKind::EmptyStack, pos))
    }
}
