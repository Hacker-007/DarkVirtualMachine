//! The Code struct maintains the values and the current position of the values vector.
//! In the future, it should maintain labels, constants, and other information about the code.
//! This Code struct is called internally and should not be called from the outside.

use crate::utils::label::Label;
use crate::{
    errors::{error::Error, error_kind::ErrorKind},
    tokens::{token::Token, token_kind::TokenKind},
    values::value::Value,
};
use std::{
    collections::{HashMap, VecDeque},
    rc::Rc,
};

#[derive(Debug)]
pub struct Code {
    value_pointer: usize,
    values: VecDeque<Rc<Value>>,
    labels: HashMap<String, Label>,
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
        let iter = tokens.into_iter().enumerate();
        let mut label_stack = vec![];
        for (pos, token) in iter {
            if let Token {
                kind: TokenKind::Label(name, parameters),
                pos: token_position,
            } = &token
            {
                label_stack.push((pos, *token_position, name.to_owned(), parameters.to_vec()));
            } else if let Token {
                kind: TokenKind::End,
                pos: token_position,
            } = &token
            {
                match label_stack.pop() {
                    Some((last_start, last_pos, last_name, last_parameters)) => {
                        if labels
                            .insert(last_name, Label::new(last_start, pos, last_parameters))
                            .is_some()
                        {
                            return Err(Error::new(ErrorKind::DuplicateLabel, last_pos));
                        }
                    }
                    None => return Err(Error::new(ErrorKind::EndWithoutLabel, *token_position)),
                }
            }

            values.push_back(Rc::new(token.into()));
        }

        if let Some((_, last_pos, _, _)) = label_stack.pop() {
            Err(Error::new(ErrorKind::NoEndOfLabel, last_pos))
        } else if let Some(Label {
            start_pos: value_pointer,
            ..
        }) = labels.get(&"main".to_owned())
        {
            Ok(Code {
                value_pointer: value_pointer + 1,
                values,
                labels,
            })
        } else {
            Err(Error::message_only(ErrorKind::NoMainLabel))
        }
    }

    /// This constructs a new Code struct with the specified tokens.
    /// Internally, the tokens are converted to reference counted values.
    /// Additionally, the Code struct does not check for a main label and instead starts at the first token.
    ///
    /// # Arguments
    /// `tokens` - The tokens from the lexer.
    pub fn repl(tokens: VecDeque<Token>) -> Result<Code, Error> {
        let mut labels = HashMap::new();
        let mut values = VecDeque::new();
        let iter = tokens.into_iter().enumerate();
        let mut label_stack = vec![];
        for (pos, token) in iter {
            if let Token {
                kind: TokenKind::Label(name, parameters),
                pos: token_position,
            } = &token
            {
                label_stack.push((pos, *token_position, name.to_owned(), parameters.to_vec()));
            } else if let Token {
                kind: TokenKind::End,
                pos: token_position,
            } = &token
            {
                match label_stack.pop() {
                    Some((last_start, last_pos, last_name, last_parameters)) => {
                        if labels
                            .insert(last_name, Label::new(last_start, pos, last_parameters))
                            .is_some()
                        {
                            return Err(Error::new(ErrorKind::DuplicateLabel, last_pos));
                        }
                    }
                    None => return Err(Error::new(ErrorKind::EndWithoutLabel, *token_position)),
                }
            }

            values.push_back(Rc::new(token.into()));
        }

        Ok(Code {
            value_pointer: 0,
            values,
            labels,
        })
    }

    /// This function updates the value_pointer to have the value of jump_location
    /// if and only if jump_location is a valid index. Note that counting is 0-based.
    ///
    /// # Arguments
    /// `jump_location` - The new value of value_pointer.
    /// `pos` - The position where this was needed.
    pub fn jump(&mut self, jump_location: i64, pos: usize) -> Option<Error> {
        let upper_bound = self.values.len() as i64;
        if jump_location >= 0 && jump_location <= upper_bound {
            self.value_pointer = jump_location as usize;
            None
        } else {
            Some(Error::new(
                ErrorKind::OutOfBounds(0, self.values.len() + 1),
                pos,
            ))
        }
    }

    /// This function updates the value_pointer by the value of jump_location
    /// if and only if jump_location is a valid index. Note that counting is 0-based.
    ///
    /// # Arguments
    /// `jump_location` - The new value of value_pointer.
    /// `pos` - The position where this was needed.
    pub fn relative_jump(&mut self, jump_location: i64, pos: usize) -> Option<Error> {
        let lower_bound = -(self.value_pointer as i64);
        let upper_bound = self.values.len() as i64;
        if jump_location >= lower_bound && jump_location <= upper_bound {
            if jump_location < 0 {
                self.value_pointer -= (-jump_location) as usize;
                None
            } else {
                self.value_pointer += jump_location as usize;
                None
            }
        } else {
            Some(Error::new(
                ErrorKind::OutOfBounds(lower_bound as usize, self.values.len()),
                pos,
            ))
        }
    }

    /// Gets the value pointer to the location of the label passed in. If the label name does not exist, an error is reported.
    /// Additionally, it returns the position of the label.
    ///
    /// # Arguments
    /// `label_name` - The name of the label.
    /// `pos` - The position where this was needed.
    pub fn get_label_location(
        &self,
        label_name: &str,
        pos: usize,
    ) -> Result<(usize, usize, Vec<String>), Error> {
        if let Some(Label {
            start_pos: label_pos_start,
            end_pos: label_pos_end,
            parameters,
        }) = self.labels.get(label_name)
        {
            Ok((*label_pos_start, *label_pos_end, parameters.iter().map(|param| param.name.to_string()).collect::<Vec<_>>()))
        } else {
            Err(Error::new(ErrorKind::UndefinedLabel, pos))
        }
    }

    /// Sets the value pointer to the location of the label passed in. If the label name does not exist, an error is reported.
    /// Additionally, it returns the position of the label.
    ///
    /// # Arguments
    /// `label_name` - The name of the label.
    /// `pos` - The position where this was needed.
    pub fn set_label_location(
        &mut self,
        label_name: &str,
        pos: usize,
    ) -> Result<(usize, usize), Error> {
        if let Some(Label {
            start_pos: label_pos_start,
            end_pos: label_pos_end,
            ..
        }) = self.labels.get_mut(label_name)
        {
            self.value_pointer = *label_pos_start + 1;
            Ok((*label_pos_start, *label_pos_end))
        } else {
            Err(Error::new(ErrorKind::UndefinedLabel, pos))
        }
    }

    /// This function gets the start and end locations of the given label.
    /// This function returns None if the label does not exist.
    pub fn get_label_start_end(&self, label_name: &str) -> Option<(usize, usize)> {
        self.labels
            .get(label_name)
            .map(|label| (label.start_pos, label.end_pos))
    }

    /// This function gets the current value of value pointer.
    pub fn get_current_pos(&self) -> usize {
        self.value_pointer
    }

    /// This function returns true if there are no more values in the Code struct.
    pub fn is_finished(&self) -> bool {
        self.value_pointer >= self.values.len()
    }
}

impl Iterator for Code {
    type Item = Rc<Value>;

    fn next(&mut self) -> Option<Self::Item> {
        self.value_pointer += 1;

        // Cloning the object is cheap because it is reference counted.
        self.values.get(self.value_pointer - 1).cloned()
    }
}
