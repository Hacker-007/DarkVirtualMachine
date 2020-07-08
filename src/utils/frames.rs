//! The Frame strut maintains information about the current frame.
//! This includes caller position, parameters, and local variables.

use super::store::Store;
use crate::{errors::error::Error, values::value::Value};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq)]
pub struct Frame {
    caller_position: usize,
    name: String,
    pub current_store: Rc<RefCell<Store>>,
}

impl Frame {
    /// Constructs a new frame.
    /// A Frame maintains the caller's position, along with its name.
    /// In the future, it will maintain local variables and any parameters passed in.
    ///
    /// # Arguments
    /// `caller_position` - The position where this frame was called or entered.
    /// `name` - The name of this frame.
    pub fn new(
        caller_position: usize,
        name: &str,
        parent_store: Option<&Rc<RefCell<Store>>>,
    ) -> Frame {
        Frame {
            caller_position,
            name: name.to_owned(),
            current_store: Rc::new(RefCell::new(Store::new(parent_store.cloned()))),
        }
    }

    pub fn find(&self, name: &str, pos: usize) -> Result<Rc<Value>, Error> {
        self.current_store.borrow().get(name, pos)
    }

    pub fn define(&mut self, name: &str, value: Rc<Value>) {
        self.current_store.borrow_mut().define(name, value);
    }

    /// This function gets the position of the caller of this frame.
    pub fn get_caller_position(&self) -> usize {
        self.caller_position
    }
}
