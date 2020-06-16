//! The Code struct maintains the values and the current position of the values vector.
//! In the future, it should maintain labels, constants, and other information about the code.
//! This Code struct is called internally and should not be called from the outside.

use crate::{errors::{error::Error, error_kind::ErrorKind}, tokens::{token_kind::TokenKind, token::Token}, values::values::Value};
use std::{collections::{HashMap, VecDeque}, rc::Rc};

#[derive(Debug)]
pub struct Code {
    value_pointer: usize,
    values: VecDeque<Rc<Value>>,
    labels: HashMap<String, usize>,
}

impl Code {
    /// This constructs a new Code struct with the specified tokens.
    /// Internally, the tokens are converted to reference counted values.
    ///
    /// # Arguments
    /// `tokens` - The tokens from the lexer.
    pub fn new(tokens: VecDeque<Token>) -> Result<Code, Error> {
        let mut labels = HashMap::new();
        let mut values = VecDeque::new();
        let mut iter = tokens.into_iter().enumerate();
        while let Some((pos, token)) = iter.next() {
            match &token.kind {
                TokenKind::Label(name) => {
                    if let Some(_) = labels.insert(name.to_owned(), pos) {
                        return Err(Error::new(ErrorKind::DuplicateLabel, token.pos));
                    } else {
                        values.push_back(Rc::new(token.into()));
                    }
                },
                _ => values.push_back(Rc::new(token.into())),
            };
        }

        if let Some(&value_pointer) = labels.get(&"main".to_owned()) {
            Ok(Code {
                value_pointer: value_pointer + 1,
                values,
                labels,
            })
        } else {
            Err(Error::message_only(ErrorKind::NoMainLabel))
        }
    }

    /// This function gets the next value from the values vector.
    pub fn next(&mut self) -> Option<Rc<Value>> {
        self.value_pointer += 1;

        // Cloning the object is cheap because it is reference counted.
        self.values.get(self.value_pointer - 1).cloned()
    }

    /// This function updates the value_pointer to have the value of jump_location
    /// if and only if jump_location is a valid index. Note that counting is 0-based.
    ///
    /// # Arguments
    /// `jump_location` - The new value of value_pointer.
    /// `pos` - The position where this was needed.
    pub fn jump(&mut self, jump_location: i64, pos: usize) -> Option<Error> {
        let upper_bound = self.values.len() as i64;
        if jump_location >= 0 && jump_location < upper_bound {
            self.value_pointer = jump_location as usize;
            None
        } else {
            Some(Error::new(ErrorKind::OutOfBounds(0, self.values.len()), pos))
        }
    }

    /// This function updates the value_pointer by the value of jump_location
    /// if and only if jump_location is a valid index. Note that counting is 0-based.
    ///
    /// # Arguments
    /// `jump_location` - The new value of value_pointer.
    /// `pos` - The position where this was needed.
    pub fn relative_jump(&mut self, jump_location: i64, pos: usize) -> Option<Error> {
        let lower_bound = -1 * (self.value_pointer as i64);
        let upper_bound = self.values.len() as i64;
        if jump_location >= lower_bound && jump_location < upper_bound {
            self.value_pointer += jump_location as usize;
            None
        } else {
            Some(Error::new(ErrorKind::OutOfBounds(lower_bound as usize, self.values.len()), pos))
        }
    }

    /// This function returns true if there are no more values in the Code struct.
    pub fn is_finished(&self) -> bool {
        self.value_pointer >= self.values.len()
    }
}
