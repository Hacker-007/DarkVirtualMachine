use std::fmt::Debug;

#[derive(Debug)]
pub struct Stack<T: Debug>(Vec<T>);

impl<T: Debug> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack(vec![])
    }

    pub fn push(&mut self, value: T) {
        self.0.push(value)
    }
}
