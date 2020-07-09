//! The VM struct represents the VM state after any given changes.
//! This struct maintains the code that is given and the stack.
//! This struct also should not change that much. The only changes would be additions to value enum and evaluate_value function.
//!
//! The VM can be invoked after the lexer has been run.
//!
//! # Example
//! ```
//! # fn run() -> Result<(), Error> {
//! let contents = "push 1";
//! let tokens = Lexer::new().lex(contents)?;
//! let result = VM::new(tokens).run()?;
//! # Ok(())
//! # }
//! ```

use crate::{
    code::Code,
    errors::{error::Error, error_kind::ErrorKind},
    tokens::token::Token,
    utils::{frames::Frame, stack::Stack},
    values::{value::Value, value_kinds::ValueKind},
};

use std::{collections::VecDeque, rc::Rc};

#[derive(Debug)]
pub struct VM {
    code: Code,
    pub operand_stack: Stack<Rc<Value>>,
    call_stack: Stack<Frame>,
}

impl VM {
    /// Constructs a new VM with the specified tokens.
    /// The tokens are usually generated through the lexer.
    /// Internally, the tokens are converted to different values by the code object.
    ///
    /// # Arguments
    /// `tokens` - The tokens produced by the lexer.
    pub fn new(tokens: VecDeque<Token>) -> Result<VM, Error> {
        let code = Code::new(tokens)?;
        let main_frame = Frame::new(0, "main", None);
        let mut call_stack = Stack::new();
        call_stack.push(main_frame);
        Ok(VM {
            code,
            operand_stack: Stack::new(),
            call_stack,
        })
    }

    /// Runs the VM until the end of the code.
    /// This function may return an optionally value, representing the value of the last expression.
    /// It may also prematurely return an error. This may be updated to return a vector of errors.
    pub fn run(&mut self) -> Result<Option<Rc<Value>>, Error> {
        loop {
            // A seperate function must be called here.
            // Otherwise, Rust's borrow checker will complain with the error that self.code is mutabley borrowed more than once.
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

    /// Evaluates the next value.
    /// This means every value is an expression in some sense.
    fn evaluate_value(&mut self, value: Rc<Value>) -> Result<Option<Rc<Value>>, Error> {
        match &value.kind {
            ValueKind::Void => Ok(None),
            ValueKind::Any => Ok(None),

            ValueKind::Int(_)
            | ValueKind::Float(_)
            | ValueKind::Boolean(_)
            | ValueKind::String(_) => Ok(Some(value)),

            // Cloning here is cheap because val is reference counted, so only a counter is incremented.
            ValueKind::Identifier(name) => self
                .call_stack
                .peek()
                .unwrap()
                .find(name, value.pos)
                .map(Some),
            ValueKind::Label(_) => {
                let mut found_end = false;
                while let Some(value) = self.next() {
                    if let ValueKind::End = value.kind {
                        found_end = true;
                        break;
                    }
                }

                if !found_end {
                    Err(Error::new(ErrorKind::NoEndOfLabel, value.pos))
                } else {
                    Ok(None)
                }
            }
            ValueKind::End => {
                let frame = self.call_stack.pop(value.pos)?;
                if let Some(error) = self
                    .code
                    .jump(frame.get_caller_position() as i64, value.pos)
                {
                    Err(error)
                } else {
                    Ok(None)
                }
            }

            ValueKind::Push => self.push(value.pos),
            ValueKind::Pop => self.pop(value.pos).map(|(_, value)| value),
            ValueKind::Peek => self.operand_stack.peek().map_or(
                Ok(Some(Rc::new(Value::new(value.pos, ValueKind::Void)))),
                |peeked_value| Ok(Some(peeked_value.clone())),
            ),
            ValueKind::Add => self.add(value.pos),
            ValueKind::Sub => self.sub(value.pos),
            ValueKind::Mul => self.mul(value.pos),
            ValueKind::Div => self.div(value.pos),
            ValueKind::LessThan => self.lt(value.pos),
            ValueKind::LessThanEqual => self.lte(value.pos),
            ValueKind::GreaterThan => self.gt(value.pos),
            ValueKind::GreaterThanEqual => self.gte(value.pos),
            ValueKind::Equal => self.eq(value.pos),
            ValueKind::NotEqual => self.neq(value.pos),
            ValueKind::Jump => self.jmp(value.pos),
            ValueKind::RelativeJump => self.rjmp(value.pos),
            ValueKind::JumpIfTrue => self.jmpt(value.pos),
            ValueKind::JumpIfFalse => self.jmpf(value.pos),
            ValueKind::Print => self.print(value.pos),
            ValueKind::PrintNewLine => self.printn(value.pos),
            ValueKind::Set => self.set(value.pos),
            ValueKind::Call => self.call(value.pos),
        }
    }

    /// Pushes the next value on to the stack.
    /// This will call the get_arg method, which calls the evaluate_value function again.
    /// This ensures that instructions can be followed by more instructions as arguments.
    ///
    /// # Arguments
    /// * `pos` - The position where the instruction was called.
    fn push(&mut self, pos: usize) -> Result<Option<Rc<Value>>, Error> {
        // Get the next argument. The two parameters passed are useful in the case of errors.
        let (pos, arg) = self.get_arg(1, pos)?;

        // If the argument does not exist, return an error, otherwise push it on to the stack.
        match arg {
            Some(value) => self.operand_stack.push(value),
            None => {
                return Err(Error::new(
                    ErrorKind::ValueMismatch(
                        ValueKind::Any.get_value_name(),
                        ValueKind::Void.get_value_name(),
                    ),
                    pos,
                ))
            }
        }

        Ok(None)
    }

    /// Pops the top value from the stack.
    ///
    /// # Arguments
    /// * `pos` - The position where the instruction was called.
    fn pop(&mut self, pos: usize) -> Result<(usize, Option<Rc<Value>>), Error> {
        // Pop the value and if there are no errors, map it to an option with the value.
        // stack.pop takes the position where the instruction was used in the case that the stack was empty.
        self.operand_stack.pop(pos).map(|val| (val.pos, Some(val)))
    }

    /// Pops the top two values from the stack and adds them together.
    /// This internally calls both the pop instruction and the add method on the Value struct.
    ///
    /// # Arguments
    /// * `pos` - The position where the instruction was called.
    fn add(&mut self, pos: usize) -> Result<Option<Rc<Value>>, Error> {
        let (arg_pos_1, arg1) = self.pop(pos)?;
        let (arg_pos_2, arg2) = self.pop(pos)?;

        match (arg1, arg2) {
            (Some(operand1), Some(operand2)) => operand1
                .add(operand2.as_ref(), pos)
                .map(|val| Some(Rc::new(val))),
            (None, _) => Err(Error::new(
                ErrorKind::ValueMismatch(
                    ValueKind::Any.get_value_name(),
                    ValueKind::Void.get_value_name(),
                ),
                arg_pos_1,
            )),
            (_, None) => Err(Error::new(
                ErrorKind::ValueMismatch(
                    ValueKind::Any.get_value_name(),
                    ValueKind::Void.get_value_name(),
                ),
                arg_pos_2,
            )),
        }
    }

    /// Pops the top two values from the stack and subtracts them.
    /// This internally calls both the pop instruction and the sub method on the Value struct.
    ///
    /// # Arguments
    /// * `pos` - The position where the instruction was called.
    fn sub(&mut self, pos: usize) -> Result<Option<Rc<Value>>, Error> {
        let (arg_pos_1, arg1) = self.pop(pos)?;
        let (arg_pos_2, arg2) = self.pop(pos)?;

        match (arg1, arg2) {
            (Some(operand1), Some(operand2)) => operand1
                .sub(operand2.as_ref(), pos)
                .map(|val| Some(Rc::new(val))),
            (None, _) => Err(Error::new(
                ErrorKind::ValueMismatch(
                    ValueKind::Any.get_value_name(),
                    ValueKind::Void.get_value_name(),
                ),
                arg_pos_1,
            )),
            (_, None) => Err(Error::new(
                ErrorKind::ValueMismatch(
                    ValueKind::Any.get_value_name(),
                    ValueKind::Void.get_value_name(),
                ),
                arg_pos_2,
            )),
        }
    }

    /// Pops the top two values from the stack and multiplies them.
    /// This internally calls both the pop instruction and the mul method on the Value struct.
    ///
    /// # Arguments
    /// * `pos` - The position where the instruction was called.
    fn mul(&mut self, pos: usize) -> Result<Option<Rc<Value>>, Error> {
        let (arg_pos_1, arg1) = self.pop(pos)?;
        let (arg_pos_2, arg2) = self.pop(pos)?;

        match (arg1, arg2) {
            (Some(operand1), Some(operand2)) => operand1
                .mul(operand2.as_ref(), pos)
                .map(|val| Some(Rc::new(val))),
            (None, _) => Err(Error::new(
                ErrorKind::ValueMismatch(
                    ValueKind::Any.get_value_name(),
                    ValueKind::Void.get_value_name(),
                ),
                arg_pos_1,
            )),
            (_, None) => Err(Error::new(
                ErrorKind::ValueMismatch(
                    ValueKind::Any.get_value_name(),
                    ValueKind::Void.get_value_name(),
                ),
                arg_pos_2,
            )),
        }
    }

    /// Pops the top two values from the stack and divides them.
    /// This internally calls both the pop instruction and the div method on the Value struct.
    ///
    /// # Arguments
    /// * `pos` - The position where the instruction was called.
    fn div(&mut self, pos: usize) -> Result<Option<Rc<Value>>, Error> {
        let (arg_pos_1, arg1) = self.pop(pos)?;
        let (arg_pos_2, arg2) = self.pop(pos)?;

        match (arg1, arg2) {
            (Some(operand1), Some(operand2)) => operand1
                .div(operand2.as_ref(), pos)
                .map(|val| Some(Rc::new(val))),
            (None, _) => Err(Error::new(
                ErrorKind::ValueMismatch(
                    ValueKind::Any.get_value_name(),
                    ValueKind::Void.get_value_name(),
                ),
                arg_pos_1,
            )),
            (_, None) => Err(Error::new(
                ErrorKind::ValueMismatch(
                    ValueKind::Any.get_value_name(),
                    ValueKind::Void.get_value_name(),
                ),
                arg_pos_2,
            )),
        }
    }

    /// Compares the two arguments and returns if the first argument is less than the second argument.
    ///
    /// # Arguments
    /// `pos` - The position where this instruction was called.
    fn lt(&mut self, pos: usize) -> Result<Option<Rc<Value>>, Error> {
        let (arg_pos_1, arg1) = self.get_arg(2, pos)?;
        let (arg_pos_2, arg2) = self.get_arg(1, pos)?;

        match (arg1, arg2) {
            (Some(operand1), Some(operand2)) => operand1
                .lt(operand2.as_ref(), pos)
                .map(|val| Some(Rc::new(val))),
            (None, _) => Err(Error::new(
                ErrorKind::ValueMismatch(
                    ValueKind::Any.get_value_name(),
                    ValueKind::Void.get_value_name(),
                ),
                arg_pos_1,
            )),
            (_, None) => Err(Error::new(
                ErrorKind::ValueMismatch(
                    ValueKind::Any.get_value_name(),
                    ValueKind::Void.get_value_name(),
                ),
                arg_pos_2,
            )),
        }
    }

    /// Compares the two arguments and returns if the first argument is less than the second argument.
    ///
    /// # Arguments
    /// `pos` - The position where this instruction was called.
    fn lte(&mut self, pos: usize) -> Result<Option<Rc<Value>>, Error> {
        let (arg_pos_1, arg1) = self.get_arg(2, pos)?;
        let (arg_pos_2, arg2) = self.get_arg(1, pos)?;

        match (arg1, arg2) {
            (Some(operand1), Some(operand2)) => operand1
                .lte(operand2.as_ref(), pos)
                .map(|val| Some(Rc::new(val))),
            (None, _) => Err(Error::new(
                ErrorKind::ValueMismatch(
                    ValueKind::Any.get_value_name(),
                    ValueKind::Void.get_value_name(),
                ),
                arg_pos_1,
            )),
            (_, None) => Err(Error::new(
                ErrorKind::ValueMismatch(
                    ValueKind::Any.get_value_name(),
                    ValueKind::Void.get_value_name(),
                ),
                arg_pos_2,
            )),
        }
    }

    /// Compares the two arguments and returns if the first argument is less than the second argument.
    ///
    /// # Arguments
    /// `pos` - The position where this instruction was called.
    fn gt(&mut self, pos: usize) -> Result<Option<Rc<Value>>, Error> {
        let (arg_pos_1, arg1) = self.get_arg(2, pos)?;
        let (arg_pos_2, arg2) = self.get_arg(1, pos)?;

        match (arg1, arg2) {
            (Some(operand1), Some(operand2)) => operand1
                .gt(operand2.as_ref(), pos)
                .map(|val| Some(Rc::new(val))),
            (None, _) => Err(Error::new(
                ErrorKind::ValueMismatch(
                    ValueKind::Any.get_value_name(),
                    ValueKind::Void.get_value_name(),
                ),
                arg_pos_1,
            )),
            (_, None) => Err(Error::new(
                ErrorKind::ValueMismatch(
                    ValueKind::Any.get_value_name(),
                    ValueKind::Void.get_value_name(),
                ),
                arg_pos_2,
            )),
        }
    }

    /// Compares the two arguments and returns if the first argument is less than the second argument.
    ///
    /// # Arguments
    /// `pos` - The position where this instruction was called.
    fn gte(&mut self, pos: usize) -> Result<Option<Rc<Value>>, Error> {
        let (arg_pos_1, arg1) = self.get_arg(2, pos)?;
        let (arg_pos_2, arg2) = self.get_arg(1, pos)?;

        match (arg1, arg2) {
            (Some(operand1), Some(operand2)) => operand1
                .gte(operand2.as_ref(), pos)
                .map(|val| Some(Rc::new(val))),
            (None, _) => Err(Error::new(
                ErrorKind::ValueMismatch(
                    ValueKind::Any.get_value_name(),
                    ValueKind::Void.get_value_name(),
                ),
                arg_pos_1,
            )),
            (_, None) => Err(Error::new(
                ErrorKind::ValueMismatch(
                    ValueKind::Any.get_value_name(),
                    ValueKind::Void.get_value_name(),
                ),
                arg_pos_2,
            )),
        }
    }

    /// Compares the two arguments and returns if the first argument is less than the second argument.
    ///
    /// # Arguments
    /// `pos` - The position where this instruction was called.
    fn eq(&mut self, pos: usize) -> Result<Option<Rc<Value>>, Error> {
        let (arg_pos_1, arg1) = self.get_arg(2, pos)?;
        let (arg_pos_2, arg2) = self.get_arg(1, pos)?;

        match (arg1, arg2) {
            (Some(operand1), Some(operand2)) => operand1
                .equal(operand2.as_ref(), pos)
                .map(|val| Some(Rc::new(val))),
            (None, _) => Err(Error::new(
                ErrorKind::ValueMismatch(
                    ValueKind::Any.get_value_name(),
                    ValueKind::Void.get_value_name(),
                ),
                arg_pos_1,
            )),
            (_, None) => Err(Error::new(
                ErrorKind::ValueMismatch(
                    ValueKind::Any.get_value_name(),
                    ValueKind::Void.get_value_name(),
                ),
                arg_pos_2,
            )),
        }
    }

    /// Compares the two arguments and returns if the first argument is less than the second argument.
    ///
    /// # Arguments
    /// `pos` - The position where this instruction was called.
    fn neq(&mut self, pos: usize) -> Result<Option<Rc<Value>>, Error> {
        let (arg_pos_1, arg1) = self.get_arg(2, pos)?;
        let (arg_pos_2, arg2) = self.get_arg(1, pos)?;

        match (arg1, arg2) {
            (Some(operand1), Some(operand2)) => operand1
                .not_equal(operand2.as_ref(), pos)
                .map(|val| Some(Rc::new(val))),
            (None, _) => Err(Error::new(
                ErrorKind::ValueMismatch(
                    ValueKind::Any.get_value_name(),
                    ValueKind::Void.get_value_name(),
                ),
                arg_pos_1,
            )),
            (_, None) => Err(Error::new(
                ErrorKind::ValueMismatch(
                    ValueKind::Any.get_value_name(),
                    ValueKind::Void.get_value_name(),
                ),
                arg_pos_2,
            )),
        }
    }

    /// Changes the instruction pointer in the Code struct to the argument passed in.
    /// However, there are restrictions on the argument:
    /// - First, the argument must be an int.
    /// - Second, the argument must fit in the range 0 and values.len() exclusive.
    /// If either of these constraints are broken, an error is returned.
    ///
    /// # Arguments
    /// `pos` - The position where this instruction was called.
    fn jmp(&mut self, pos: usize) -> Result<Option<Rc<Value>>, Error> {
        let (arg_pos_1, arg1) = self.get_arg(1, pos)?;
        match arg1 {
            Some(value) => {
                if let ValueKind::Int(jump_location) = value.kind {
                    if let Some(error) = self.code.jump(jump_location, pos) {
                        Err(error)
                    } else {
                        Ok(None)
                    }
                } else {
                    Err(Error::new(
                        ErrorKind::ValueMismatch(
                            ValueKind::Int(0).get_value_name(),
                            value.kind.get_value_name(),
                        ),
                        arg_pos_1,
                    ))
                }
            }
            None => Err(Error::new(
                ErrorKind::ValueMismatch(
                    ValueKind::Int(0).get_value_name(),
                    ValueKind::Void.get_value_name(),
                ),
                arg_pos_1,
            )),
        }
    }

    /// Changes the instruction pointer in the Code struct by the argument passed in.
    /// This argument can be positive or negative. However, it must meet the same bound requirements
    /// as the jmp instruction.
    ///
    /// # Arguments
    /// `pos` - The position where this instruction was called.
    fn rjmp(&mut self, pos: usize) -> Result<Option<Rc<Value>>, Error> {
        let (arg_pos_1, arg1) = self.get_arg(1, pos)?;
        match arg1 {
            Some(value) => {
                if let ValueKind::Int(jump_location) = value.kind {
                    if let Some(error) = self.code.relative_jump(jump_location, pos) {
                        Err(error)
                    } else {
                        Ok(None)
                    }
                } else {
                    Err(Error::new(
                        ErrorKind::ValueMismatch(
                            ValueKind::Int(0).get_value_name(),
                            value.kind.get_value_name(),
                        ),
                        arg_pos_1,
                    ))
                }
            }
            None => Err(Error::new(
                ErrorKind::ValueMismatch(
                    ValueKind::Int(0).get_value_name(),
                    ValueKind::Void.get_value_name(),
                ),
                arg_pos_1,
            )),
        }
    }

    /// Changes the instruction pointer in the Code struct to the argument passed in
    /// if the top value on the stack is true.
    /// However, there are restrictions on the argument:
    /// - First, the argument must be an int.
    /// - Second, the argument must fit in the range 0 and values.len() exclusive.
    /// If either of these constraints are broken, an error is returned.
    ///
    /// # Arguments
    /// `pos` - The position where this instruction was called.
    fn jmpt(&mut self, pos: usize) -> Result<Option<Rc<Value>>, Error> {
        match self.operand_stack.peek() {
            Some(value) if value.is_truthy() => self.jmp(pos),
            None => Err(Error::new(ErrorKind::EmptyStack, pos)),
            _ => Ok(None),
        }
    }

    /// Changes the instruction pointer in the Code struct to the argument passed in
    /// if the top value on the stack is false.
    /// However, there are restrictions on the argument:
    /// - First, the argument must be an int.
    /// - Second, the argument must fit in the range 0 and values.len() exclusive.
    /// If either of these constraints are broken, an error is returned.
    ///
    /// # Arguments
    /// `pos` - The position where this instruction was called.
    fn jmpf(&mut self, pos: usize) -> Result<Option<Rc<Value>>, Error> {
        match self.operand_stack.peek() {
            Some(value) if !value.is_truthy() => self.jmp(pos),
            None => Err(Error::new(ErrorKind::EmptyStack, pos)),
            _ => Ok(None),
        }
    }

    /// Prints the argument passed in.
    ///
    /// # Arguments
    /// `pos` - The position where this instruction was called.
    fn print(&mut self, pos: usize) -> Result<Option<Rc<Value>>, Error> {
        let (arg_pos_1, arg1) = self.get_arg(1, pos)?;
        match arg1 {
            Some(value) => {
                print!("{:#?}", value);
                Ok(None)
            }
            None => Err(Error::new(
                ErrorKind::ValueMismatch(
                    ValueKind::Any.get_value_name(),
                    ValueKind::Void.get_value_name(),
                ),
                arg_pos_1,
            )),
        }
    }

    /// Prints the argument passed in with a new line after it.
    ///
    /// # Arguments
    /// `pos` - The position where this instruction was called.
    fn printn(&mut self, pos: usize) -> Result<Option<Rc<Value>>, Error> {
        let (arg_pos_1, arg1) = self.get_arg(1, pos)?;
        match arg1 {
            Some(value) => {
                println!("{:#?}", value);
                Ok(None)
            }
            None => Err(Error::new(
                ErrorKind::ValueMismatch(
                    ValueKind::Any.get_value_name(),
                    ValueKind::Void.get_value_name(),
                ),
                arg_pos_1,
            )),
        }
    }

    /// Sets the identifier passed in to the value passed in.
    ///
    /// # Arguments
    /// `pos` - The position where this instruction was called.
    fn set(&mut self, pos: usize) -> Result<Option<Rc<Value>>, Error> {
        let (arg_pos_1, arg1) = self.get_arg_unevaluated(2, pos)?;
        let (arg_pos_2, arg2) = self.get_arg(1, pos)?;

        match &arg1.kind {
            ValueKind::Identifier(name) => {
                if let Some(value) = arg2 {
                    self.call_stack.peek_mut().unwrap().define(name, value);
                    Ok(None)
                } else {
                    Err(Error::new(
                        ErrorKind::ValueMismatch(
                            ValueKind::Any.get_value_name(),
                            ValueKind::Void.get_value_name(),
                        ),
                        arg_pos_2,
                    ))
                }
            }
            kind => Err(Error::new(
                ErrorKind::ValueMismatch(
                    ValueKind::Identifier("".to_owned()).get_value_name(),
                    kind.get_value_name(),
                ),
                arg_pos_1,
            )),
        }
    }

    /// Calls the label passed in. In other words, it changes the instruction pointer.
    /// In the future, this would be changed to include the number of parameters on the stack.
    ///
    /// # Arguments
    /// `pos` - The position where this instruction was called.
    fn call(&mut self, pos: usize) -> Result<Option<Rc<Value>>, Error> {
        let (arg_pos_1, arg1) = self.get_arg_unevaluated(1, pos)?;
        match &arg1.kind {
            ValueKind::Identifier(label_name) => {
                let caller_pos = self.code.get_current_pos();
                let (start, end) = self.code.set_label_location(label_name, arg_pos_1)?;
                let store = self.call_stack.peek().filter(|frame| {
                    if let Some((cur_start, cur_end)) = self.code.get_label_start_end(&frame.name) {
                        cur_start < start && end < cur_end
                    } else {
                        false
                    }
                }).map(|frame| &frame.current_store);

                let new_frame = Frame::new(caller_pos, label_name, store);
                self.call_stack.push(new_frame);

                Ok(None)
            },
            kind => Err(Error::new(
                ErrorKind::ValueMismatch(
                    ValueKind::Label("".to_owned()).get_value_name(),
                    kind.get_value_name(),
                ),
                arg_pos_1,
            )),
        }
    }

    /// Gets the next argument.
    /// This funtion is usually called by instructions.
    ///
    /// # Arguments
    /// * `expected_args` - The number of arguments remaining for the instruction.
    /// * `pos` - The position where the instrution was called.
    fn get_arg(
        &mut self,
        expected_args: usize,
        pos: usize,
    ) -> Result<(usize, Option<Rc<Value>>), Error> {
        let arg = self
            .next()
            .ok_or_else(|| Error::new(ErrorKind::ExpectedArgs(expected_args), pos))?;
        Ok((arg.pos, self.evaluate_value(arg)?))
    }

    /// Gets the next argument.
    /// This funtion is usually called by instructions.
    ///
    /// # Arguments
    /// * `expected_args` - The number of arguments remaining for the instruction.
    /// * `pos` - The position where the instrution was called.
    fn get_arg_unevaluated(
        &mut self,
        expected_args: usize,
        pos: usize,
    ) -> Result<(usize, Rc<Value>), Error> {
        let arg = self
            .next()
            .ok_or_else(|| Error::new(ErrorKind::ExpectedArgs(expected_args), pos))?;
        Ok((arg.pos, arg))
    }

    /// Gets the next value.
    /// This method needs to be abstracted away because Rust will complain with the message that self.code was mutabley borrowed more than once.
    fn next(&mut self) -> Option<Rc<Value>> {
        self.code.next()
    }

    /// Checks if there are any more values left.
    /// This method needs to be abstracted away because Rust will complain with the message that self.code was mutabley borrowed more than once.
    fn is_finished(&self) -> bool {
        self.code.is_finished() || self.call_stack.is_empty()
    }
}
