//! The Store struct is a basic wrapper around a HashMap.
//! It is useful for maintaining variables and available functions.

use crate::{
    errors::{error::Error, error_kind::ErrorKind},
    values::value::Value,
};
use std::{collections::HashMap, rc::Rc};

#[derive(Debug, PartialEq)]
pub struct Store<'a> {
    parent_store: Option<&'a Store<'a>>,
    store: HashMap<String, Rc<Value>>,
}

impl<'a> Store<'a> {
    /// Creates a new Store.
    ///
    /// # Arguments
    /// `parent_store` - The parent of this store. This maintains all of the variables defined in a higher scope.
    pub fn new(parent_store: Option<&'a Store<'a>>) -> Store<'a> {
        Store {
            parent_store,
            store: HashMap::new(),
        }
    }

    /// This function defines a new variable. The variable will be bound to the scope containing this store.
    /// This function will override any existing value of a previously defined value.
    ///
    /// # Arguments
    /// `name` - The name of the variable.
    /// `value` - The value of the variable.
    pub fn define(&mut self, name: &str, value: Rc<Value>) {
        self.store.insert(name.to_owned(), value);
    }

    /// This function gets the value of a variable. If the variable does not exist, then an error is reported.
    ///
    /// # Arguments
    /// `name` - The name of the variable.
    /// `pos` - The position where this operation was called.
    pub fn get(&self, name: &str, pos: usize) -> Result<Rc<Value>, Error> {
        let var = self.store.get(name);
        if let Some(variable) = var {
            Ok(variable.clone())
        } else if let Some(parent) = self.parent_store {
            parent.get(name, pos)
        } else {
            Err(Error::new(ErrorKind::UndefinedVariable, pos))
        }
    }
}
