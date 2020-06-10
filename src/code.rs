use crate::{utils::token::Token, values::values::Value};
use std::{collections::VecDeque, rc::Rc};

#[derive(Debug)]
pub struct Code {
    value_pointer: usize,
    values: VecDeque<Rc<Value>>,
}

impl Code {
    pub fn new(tokens: VecDeque<Token>) -> Code {
        Code {
            value_pointer: 0,
            values: tokens
                .into_iter()
                .map(|token| Rc::new(token.into()))
                .collect::<VecDeque<_>>(),
        }
    }

    pub fn next(&mut self) -> Option<Rc<Value>> {
        self.value_pointer += 1;
        self.values.get(self.value_pointer - 1).cloned()
    }

    pub fn is_finished(&self) -> bool {
        self.value_pointer == self.values.len()
    }
}
