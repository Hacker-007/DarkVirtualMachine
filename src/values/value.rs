use super::value_kinds::ValueKind;
use crate::{
    errors::{error::Error, error_kind::ErrorKind},
    tokens::{token::Token, token_kind::TokenKind},
};
use std::fmt;

/// The Value struct maintains both the position where this value is used and its kind.
/// Maintaining the position is useful because it can be used to produce good error messages.

#[derive(PartialEq, Clone)]
pub struct Value {
    pub pos: usize,
    pub kind: ValueKind,
}

impl Value {
    /// Constructs a new Value struct with the specified position and kind.
    ///
    /// # Arguments
    /// `pos` - The position where this value is created or called.
    /// `kind` - The value of this value.
    pub fn new(pos: usize, kind: ValueKind) -> Value {
        Value { pos, kind }
    }
}

impl Value {
    /// This function takes the current value and a reference to another value and adds them together.
    /// Note that this function does not take ownership of either value. Instead, it creates a new value.
    ///
    /// # Arguments
    /// `other` - The other value to add.
    /// `pos` - The position where this operation was called.
    pub fn add(&self, other: &Value, pos: usize) -> Result<Value, Error> {
        match (&self.kind, &other.kind) {
            (ValueKind::String(val1), ValueKind::String(val2)) => Ok(Value::new(
                pos,
                ValueKind::String(format!("{}{}", val1, val2)),
            )),
            (_, ValueKind::String(val2)) if self.kind != ValueKind::Void => Ok(Value::new(
                pos,
                ValueKind::String(format!("{:#?}{}", self, val2)),
            )),
            (ValueKind::String(val1), _) if other.kind != ValueKind::Void => Ok(Value::new(
                pos,
                ValueKind::String(format!("{}{:#?}", val1, other)),
            )),

            (ValueKind::Int(val1), ValueKind::Int(val2)) => {
                Ok(Value::new(pos, ValueKind::Int(val1 + val2)))
            }
            (ValueKind::Int(val1), ValueKind::Float(val2)) => {
                Ok(Value::new(pos, ValueKind::Float(*val1 as f64 + val2)))
            }
            (ValueKind::Float(val1), ValueKind::Int(val2)) => {
                Ok(Value::new(pos, ValueKind::Float(val1 + *val2 as f64)))
            }
            (ValueKind::Float(val1), ValueKind::Float(val2)) => {
                Ok(Value::new(pos, ValueKind::Float(val1 + val2)))
            }

            _ => Err(Error::new(
                ErrorKind::UnsupportedOperation(
                    "Add".to_owned(),
                    format!(
                        "The Value '{}' And The Value '{}'.",
                        self.kind.get_value_name(),
                        other.kind.get_value_name()
                    ),
                ),
                pos,
            )),
        }
    }

    /// This function takes the current value and a reference to another value and subtracts them.
    /// Note that this function does not take ownership of either value. Instead, it creates a new value.
    ///
    /// # Arguments
    /// `other` - The other value to subtract.
    /// `pos` - The position where this operation was called.
    pub fn sub(&self, other: &Value, pos: usize) -> Result<Value, Error> {
        match (&self.kind, &other.kind) {
            (ValueKind::Int(val1), ValueKind::Int(val2)) => {
                Ok(Value::new(pos, ValueKind::Int(val1 - val2)))
            }
            (ValueKind::Int(val1), ValueKind::Float(val2)) => {
                Ok(Value::new(pos, ValueKind::Float(*val1 as f64 - val2)))
            }
            (ValueKind::Float(val1), ValueKind::Int(val2)) => {
                Ok(Value::new(pos, ValueKind::Float(val1 - *val2 as f64)))
            }
            (ValueKind::Float(val1), ValueKind::Float(val2)) => {
                Ok(Value::new(pos, ValueKind::Float(val1 - val2)))
            }

            _ => Err(Error::new(
                ErrorKind::UnsupportedOperation(
                    "Sub".to_owned(),
                    format!(
                        "The Value '{}' And The Value '{}'.",
                        self.kind.get_value_name(),
                        other.kind.get_value_name()
                    ),
                ),
                pos,
            )),
        }
    }

    /// This function takes the current value and a reference to another value and mutliplies them.
    /// Note that this function does not take ownership of either value. Instead, it creates a new value.
    ///
    /// # Arguments
    /// `other` - The other value to multiply.
    /// `pos` - The position where this operation was called.
    pub fn mul(&self, other: &Value, pos: usize) -> Result<Value, Error> {
        match (&self.kind, &other.kind) {
            (ValueKind::String(val1), ValueKind::Int(val2)) => Ok(Value::new(
                pos,
                ValueKind::String(val1.repeat(val2.abs() as usize)),
            )),
            (ValueKind::Int(val1), ValueKind::String(val2)) if self.kind != ValueKind::Void => Ok(
                Value::new(pos, ValueKind::String(val2.repeat(val1.abs() as usize))),
            ),

            (ValueKind::Int(val1), ValueKind::Int(val2)) => {
                Ok(Value::new(pos, ValueKind::Int(val1 * val2)))
            }
            (ValueKind::Int(val1), ValueKind::Float(val2)) => {
                Ok(Value::new(pos, ValueKind::Float(*val1 as f64 * val2)))
            }
            (ValueKind::Float(val1), ValueKind::Int(val2)) => {
                Ok(Value::new(pos, ValueKind::Float(val1 * *val2 as f64)))
            }
            (ValueKind::Float(val1), ValueKind::Float(val2)) => {
                Ok(Value::new(pos, ValueKind::Float(val1 * val2)))
            }

            _ => Err(Error::new(
                ErrorKind::UnsupportedOperation(
                    "Mul".to_owned(),
                    format!(
                        "The Value '{}' And The Value '{}'.",
                        self.kind.get_value_name(),
                        other.kind.get_value_name()
                    ),
                ),
                pos,
            )),
        }
    }

    /// This function takes the current value and a reference to another value and divides them.
    /// Note that this function does not take ownership of either value. Instead, it creates a new value.
    ///
    /// # Arguments
    /// `other` - The other value to divide.
    /// `pos` - The position where this operation was called.
    pub fn div(&self, other: &Value, pos: usize) -> Result<Value, Error> {
        match (&self.kind, &other.kind) {
            (ValueKind::Int(val1), ValueKind::Int(val2)) => {
                if val2 == &0 {
                    Err(Error::new(ErrorKind::DivisionByZero, pos))
                } else {
                    Ok(Value::new(pos, ValueKind::Int(val1 / val2)))
                }
            }
            (ValueKind::Int(val1), ValueKind::Float(val2)) => {
                if val2 - 0.0 < std::f64::EPSILON {
                    Err(Error::new(ErrorKind::DivisionByZero, pos))
                } else {
                    Ok(Value::new(pos, ValueKind::Float(*val1 as f64 / val2)))
                }
            }
            (ValueKind::Float(val1), ValueKind::Int(val2)) => {
                if val1 - 0.0 < std::f64::EPSILON {
                    Err(Error::new(ErrorKind::DivisionByZero, pos))
                } else {
                    Ok(Value::new(pos, ValueKind::Float(val1 / *val2 as f64)))
                }
            }
            (ValueKind::Float(val1), ValueKind::Float(val2)) => {
                if val2 - 0.0 < std::f64::EPSILON {
                    Err(Error::new(ErrorKind::DivisionByZero, pos))
                } else {
                    Ok(Value::new(pos, ValueKind::Float(val1 / val2)))
                }
            }

            _ => Err(Error::new(
                ErrorKind::UnsupportedOperation(
                    "Div".to_owned(),
                    format!(
                        "The Value '{}' And The Value '{}'.",
                        self.kind.get_value_name(),
                        other.kind.get_value_name()
                    ),
                ),
                pos,
            )),
        }
    }

    /// This function takes the current value and a reference to another value and returns if the current value
    /// is less than the second one. Note that this function does not consume either value.
    ///
    /// # Arguments
    /// `other` - The other value to compare.
    /// `pos` - The position where this operation was called.
    pub fn lt(&self, other: &Value, pos: usize) -> Result<Value, Error> {
        match (&self.kind, &other.kind) {
            (ValueKind::Int(val1), ValueKind::Int(val2)) => {
                Ok(Value::new(pos, ValueKind::Boolean(val1 < val2)))
            }
            (ValueKind::Float(val1), ValueKind::Float(val2)) => {
                Ok(Value::new(pos, ValueKind::Boolean(val1 < val2)))
            }
            (ValueKind::String(val1), ValueKind::String(val2)) => {
                Ok(Value::new(pos, ValueKind::Boolean(val1 < val2)))
            }

            _ => Err(Error::new(
                ErrorKind::UnsupportedOperation(
                    "Lt".to_owned(),
                    format!(
                        "The Value '{}' And The Value '{}'.",
                        self.kind.get_value_name(),
                        other.kind.get_value_name()
                    ),
                ),
                pos,
            )),
        }
    }

    /// This function takes the current value and a reference to another value and returns if the current value
    /// is less than or equal to the second one. Note that this function does not consume either value.
    ///
    /// # Arguments
    /// `other` - The other value to compare.
    /// `pos` - The position where this operation was called.
    pub fn lte(&self, other: &Value, pos: usize) -> Result<Value, Error> {
        match (&self.kind, &other.kind) {
            (ValueKind::Int(val1), ValueKind::Int(val2)) => {
                Ok(Value::new(pos, ValueKind::Boolean(val1 <= val2)))
            }
            (ValueKind::Float(val1), ValueKind::Float(val2)) => {
                Ok(Value::new(pos, ValueKind::Boolean(val1 <= val2)))
            }
            (ValueKind::String(val1), ValueKind::String(val2)) => {
                Ok(Value::new(pos, ValueKind::Boolean(val1 <= val2)))
            }

            _ => Err(Error::new(
                ErrorKind::UnsupportedOperation(
                    "Lte".to_owned(),
                    format!(
                        "The Value '{}' And The Value '{}'.",
                        self.kind.get_value_name(),
                        other.kind.get_value_name()
                    ),
                ),
                pos,
            )),
        }
    }

    /// This function takes the current value and a reference to another value and returns if the current value
    /// is greater than the second one. Note that this function does not consume either value.
    ///
    /// # Arguments
    /// `other` - The other value to compare.
    /// `pos` - The position where this operation was called.
    pub fn gt(&self, other: &Value, pos: usize) -> Result<Value, Error> {
        match (&self.kind, &other.kind) {
            (ValueKind::Int(val1), ValueKind::Int(val2)) => {
                Ok(Value::new(pos, ValueKind::Boolean(val1 > val2)))
            }
            (ValueKind::Float(val1), ValueKind::Float(val2)) => {
                Ok(Value::new(pos, ValueKind::Boolean(val1 > val2)))
            }
            (ValueKind::String(val1), ValueKind::String(val2)) => {
                Ok(Value::new(pos, ValueKind::Boolean(val1 > val2)))
            }

            _ => Err(Error::new(
                ErrorKind::UnsupportedOperation(
                    "Gt".to_owned(),
                    format!(
                        "The Value '{}' And The Value '{}'.",
                        self.kind.get_value_name(),
                        other.kind.get_value_name()
                    ),
                ),
                pos,
            )),
        }
    }

    /// This function takes the current value and a reference to another value and returns if the current value
    /// is greater than or equal to the second one. Note that this function does not consume either value.
    ///
    /// # Arguments
    /// `other` - The other value to compare.
    /// `pos` - The position where this operation was called.
    pub fn gte(&self, other: &Value, pos: usize) -> Result<Value, Error> {
        match (&self.kind, &other.kind) {
            (ValueKind::Int(val1), ValueKind::Int(val2)) => {
                Ok(Value::new(pos, ValueKind::Boolean(val1 >= val2)))
            }
            (ValueKind::Float(val1), ValueKind::Float(val2)) => {
                Ok(Value::new(pos, ValueKind::Boolean(val1 >= val2)))
            }
            (ValueKind::String(val1), ValueKind::String(val2)) => {
                Ok(Value::new(pos, ValueKind::Boolean(val1 >= val2)))
            }

            _ => Err(Error::new(
                ErrorKind::UnsupportedOperation(
                    "Gte".to_owned(),
                    format!(
                        "The Value '{}' And The Value '{}'.",
                        self.kind.get_value_name(),
                        other.kind.get_value_name()
                    ),
                ),
                pos,
            )),
        }
    }

    /// This function takes the current value and a reference to another value and returns if the current value
    /// is equal to the second one. Note that this function does not consume either value.
    ///
    /// # Arguments
    /// `other` - The other value to compare.
    /// `pos` - The position where this operation was called.
    pub fn equal(&self, other: &Value, pos: usize) -> Value {
        match (&self.kind, &other.kind) {
            (ValueKind::Int(val1), ValueKind::Int(val2)) => {
                Value::new(pos, ValueKind::Boolean(val1 == val2))
            }
            (ValueKind::Float(val1), ValueKind::Float(val2)) => Value::new(
                pos,
                ValueKind::Boolean((val1 - val2).abs() < std::f64::EPSILON),
            ),
            (ValueKind::Boolean(val1), ValueKind::Boolean(val2)) => {
                Value::new(pos, ValueKind::Boolean(val1 == val2))
            }
            (ValueKind::String(val1), ValueKind::String(val2)) => {
                Value::new(pos, ValueKind::Boolean(val1 == val2))
            }

            _ => Value::new(pos, ValueKind::Boolean(false)),
        }
    }

    /// This function takes the current value and a reference to another value and returns if the current value
    /// is not equal to the second one. Note that this function does not consume either value.
    ///
    /// # Arguments
    /// `other` - The other value to compare.
    /// `pos` - The position where this operation was called.
    pub fn not_equal(&self, other: &Value, pos: usize) -> Value {
        match (&self.kind, &other.kind) {
            (ValueKind::Int(val1), ValueKind::Int(val2)) => {
                Value::new(pos, ValueKind::Boolean(val1 != val2))
            }
            (ValueKind::Float(val1), ValueKind::Float(val2)) => Value::new(
                pos,
                ValueKind::Boolean((val1 - val2).abs() > std::f64::EPSILON),
            ),
            (ValueKind::Boolean(val1), ValueKind::Boolean(val2)) => {
                Value::new(pos, ValueKind::Boolean(val1 != val2))
            }
            (ValueKind::String(val1), ValueKind::String(val2)) => {
                Value::new(pos, ValueKind::Boolean(val1 != val2))
            }

            _ => Value::new(pos, ValueKind::Boolean(true)),
        }
    }

    /// This function takes the current value and returns if it is "truthy".
    /// This can mean different things for differet values. For ints, it is whether it is not 0.
    /// For floats, it is whether it is not NAN, infinite, and not 0. For strings, it is whether
    /// it is not empty. Every other value is considered to be false.
    pub fn is_truthy(&self) -> bool {
        match &self.kind {
            ValueKind::Int(value) => value != &0,
            ValueKind::Float(value) => value.is_normal(),
            ValueKind::Boolean(value) => *value,
            ValueKind::String(value) => !value.is_empty(),
            _ => false,
        }
    }
}

/// Converts a token into a value. This is used by the Code struct when generating the vector of values.
impl From<Token> for Value {
    fn from(token: Token) -> Self {
        Value {
            pos: token.pos,
            kind: match token.kind {
                TokenKind::Void => ValueKind::Void,
                TokenKind::Any => ValueKind::Any,
                TokenKind::IntegerLiteral(value) => ValueKind::Int(value),
                TokenKind::FloatLiteral(value) => ValueKind::Float(value),
                TokenKind::BooleanLiteral(value) => ValueKind::Boolean(value),
                TokenKind::StringLiteral(value) => ValueKind::String(value),
                TokenKind::Identifier(name) => ValueKind::Identifier(name),
                TokenKind::Label(name, parameters) => ValueKind::Label(name, parameters),
                TokenKind::End => ValueKind::End,

                TokenKind::Push => ValueKind::Push,
                TokenKind::Pop => ValueKind::Pop,
                TokenKind::Peek => ValueKind::Peek,
                TokenKind::Add => ValueKind::Add,
                TokenKind::Sub => ValueKind::Sub,
                TokenKind::Mul => ValueKind::Mul,
                TokenKind::Div => ValueKind::Div,
                TokenKind::LessThan => ValueKind::LessThan,
                TokenKind::LessThanEqual => ValueKind::LessThanEqual,
                TokenKind::GreaterThan => ValueKind::GreaterThan,
                TokenKind::GreaterThanEqual => ValueKind::GreaterThanEqual,
                TokenKind::Equal => ValueKind::Equal,
                TokenKind::NotEqual => ValueKind::NotEqual,
                TokenKind::Jump => ValueKind::Jump,
                TokenKind::RelativeJump => ValueKind::RelativeJump,
                TokenKind::JumpIfTrue => ValueKind::JumpIfTrue,
                TokenKind::JumpIfFalse => ValueKind::JumpIfFalse,
                TokenKind::Print => ValueKind::Print,
                TokenKind::PrintNewLine => ValueKind::PrintNewLine,
                TokenKind::Set => ValueKind::Set,
                TokenKind::Call => ValueKind::Call,
            },
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.kind)
    }
}
