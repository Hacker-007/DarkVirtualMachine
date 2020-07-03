//! The Stack struct is the backbone for the VM. It maintains all of the values that are operated on.
//! The methods on the stack allow the stack to be changed and modified. The owner of the stack is the VM.
//! These methods should not be accessed outside of the VM struct as it could cause unexpected behavior.

use crate::errors::{error::Error, error_kind::ErrorKind};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Stack<T: Debug + PartialEq>(pub Vec<T>);

impl<T: Debug + PartialEq> Stack<T> {
    /// Constructs an empty stack.
    pub fn new() -> Stack<T> {
        Stack(vec![])
    }

    /// This function pushes the given value on to the stack.
    ///
    /// # Arguments
    /// `value` - The value to push on to the stack.
    pub fn push(&mut self, value: T) {
        self.0.push(value)
    }

    /// This function pop the top value on to the stack. This may result in an error if the stack is empty.
    ///
    /// # Arguments
    /// `pos` - The position where the pop was called. This is used if there was error.
    pub fn pop(&mut self, pos: usize) -> Result<T, Error> {
        self.0
            .pop()
            .ok_or_else(|| Error::new(ErrorKind::EmptyStack, pos))
    }

    /// This function returns a reference to the top value on the stack, without consuming it.
    /// If the stack is empty, None is returned.
    pub fn peek(&self) -> Option<&T> {
        if self.0.is_empty() {
            None
        } else {
            self.0.first()
        }
    }

    /// This function returns a reference to the top value on the stack, without consuming it.
    /// If the stack is empty, None is returned.
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.0.is_empty() {
            None
        } else {
            self.0.first_mut()
        }
    }

    /// This function returns true if there are no elements in the stack.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
