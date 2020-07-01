//! The Store struct is a basic wrapper around a HashMap.
//! It is useful for maintaining variables and available functions.

use crate::{
    errors::{error::Error, error_kind::ErrorKind},
    values::value::Value,
};
use std::{collections::HashMap, rc::Rc};

pub struct Store(HashMap<String, Rc<Value>>);

impl Store {
    /// Creates a new Store. In the future, this Store may be populated with some default functions from the standard library.
    pub fn new() -> Store {
        Store(HashMap::new())
    }

    /// This function defines a new variable. The variable will be bound to the scope containing this store.
    /// This function may return an error if the variable has already been defined.
    ///
    /// # Arguments
    /// `name` - The name of the variable.
    /// `value` - The value of the variable.
    /// `pos` - The position where this operation was called.
    pub fn define(&mut self, name: &str, value: Rc<Value>, pos: usize) -> Result<(), Error> {
        if self.0.contains_key(name) {
            Err(Error::new(ErrorKind::DuplicateVariable, pos))
        } else {
            self.0.insert(name.to_owned(), value);
            Ok(())
        }
    }

    /// This function gets the value of a variable. If the variable does not exist, then an error is reported.
    ///
    /// # Arguments
    /// `name` - The name of the variable.
    /// `pos` - The position where this operation was called.
    pub fn get(&self, name: &str, pos: usize) -> Result<Rc<Value>, Error> {
        self.0
            .get(name)
            .ok_or_else(|| Error::new(ErrorKind::UndefinedVariable, pos))
            .map(|var| var.clone())
    }
}
