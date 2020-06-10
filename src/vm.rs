use crate::{
    code::Code,
    utils::{
        error::{Error, ErrorKind},
        stack::Stack,
        token::Token,
    },
    values::values::{Value, ValueKind},
};

use std::{collections::VecDeque, rc::Rc};

#[derive(Debug)]
pub struct VM {
    code: Code,
    stack: Stack<Rc<Value>>,
}

impl VM {
    pub fn new(tokens: VecDeque<Token>) -> VM {
        VM {
            code: Code::new(tokens),
            stack: Stack::new(),
        }
    }

    pub fn run(&mut self) -> Result<Option<Rc<Value>>, Error> {
        loop {
            if self.is_finished() {
                return Ok(None);
            }

            let next = self.next().unwrap();
            let result = self.evaluate_value(next)?;
            if self.is_finished() && result.is_some() {
                return Ok(result);
            }
        }
    }

    fn evaluate_value(&mut self, value: Rc<Value>) -> Result<Option<Rc<Value>>, Error> {
        match &value.kind {
            ValueKind::Void => Ok(None),
            ValueKind::Any => Ok(None),

            ValueKind::Int(_) | ValueKind::Float(_) | ValueKind::String(_) => Ok(Some(value)),

            ValueKind::Variable(_, val) => Ok(Some(val.clone())),

            ValueKind::Push => self.push(value.pos),
        }
    }

    fn push(&mut self, pos: usize) -> Result<Option<Rc<Value>>, Error> {
        let (pos, arg) = self.get_arg(1, pos)?;
        match arg {
            Some(value) => self.stack.push(value),
            None => {
                return Err(Error::new(
                    ErrorKind::TypeMismatch(ValueKind::Any, ValueKind::Void),
                    pos,
                ))
            }
        }

        Ok(None)
    }

    fn get_arg(
        &mut self,
        expected_args: usize,
        pos: usize,
    ) -> Result<(usize, Option<Rc<Value>>), Error> {
        let arg = self
            .next()
            .ok_or(Error::new(ErrorKind::ExpectedArgs(expected_args), pos))?;
        Ok((arg.pos, self.evaluate_value(arg)?))
    }

    fn next(&mut self) -> Option<Rc<Value>> {
        self.code.next()
    }

    fn is_finished(&self) -> bool {
        self.code.is_finished()
    }
}
