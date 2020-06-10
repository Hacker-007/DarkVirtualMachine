//! The Value struct maintains both the position where this value is used and its kind.
//! Maintaining the position is useful because it can be used to produce good error messages.

use crate::utils::token::{Token, TokenKind};
use std::{fmt, rc::Rc};

pub struct Value {
    pub pos: usize,
    pub kind: ValueKind,
}

impl Value {
    /// Constructs a new Value struct with the specified position and kind.
    ///
    /// # Arguments
    /// `pos` - The position where this value is created or called.
    /// `kind` - The type of this value.
    pub fn new(pos: usize, kind: ValueKind) -> Value {
        Value { pos, kind }
    }
}

/// The ValueKind enum maintains the various types in the language.
/// All of the supported types are in this enum. This makes it easy to expand in the future.

pub enum ValueKind {
    Void,
    Any,
    Int(i64),
    Float(f64),
    String(String),
    Variable(String, Rc<Value>),
    Push,
    Pop,
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
                TokenKind::StringLiteral(value) => ValueKind::String(value),
                TokenKind::Identifier(name) => ValueKind::Variable(name, Rc::new(Value::new(token.pos, ValueKind::Void))),
                TokenKind::Push => ValueKind::Push,
                TokenKind::Pop => ValueKind::Pop,
            },
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl PartialEq for ValueKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ValueKind::Void, ValueKind::Void) |
            (ValueKind::Any, ValueKind::Any) => true,
            (ValueKind::Int(val1), ValueKind::Int(val2)) => val1 == val2,
            (ValueKind::Float(val1), ValueKind::Float(val2)) => val1 == val2,
            (ValueKind::String(val1), ValueKind::String(val2)) => val1 == val2,
            (ValueKind::Variable(_, val1), ValueKind::Variable(_, val2)) => val1 == val2,
            (ValueKind::Push, ValueKind::Push) |
            (ValueKind::Pop, ValueKind::Pop) => true,
            _ => false,
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.kind)
    }
}

impl fmt::Debug for ValueKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueKind::Void => write!(f, "Void"),
            ValueKind::Any => write!(f, "Any"),
            ValueKind::Int(value) => write!(f, "{}", value),
            ValueKind::Float(value) => write!(f, "{}", value),
            ValueKind::String(value) => write!(f, "{}", value),
            ValueKind::Variable(name, value) => write!(f, "{} => {:#?}", name, value),
            ValueKind::Push => write!(f, "<instruction push>"),
            ValueKind::Pop => write!(f, "<instruction pop>"),
        }
    }
}
