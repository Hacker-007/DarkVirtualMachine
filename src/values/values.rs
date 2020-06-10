use crate::utils::token::{Token, TokenKind};
use std::{fmt, rc::Rc};

pub struct Value {
    pub pos: usize,
    pub kind: ValueKind,
}

impl Value {
    pub fn new(pos: usize, kind: ValueKind) -> Value {
        Value { pos, kind }
    }
}

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
                TokenKind::Identifier(name) => {
                    ValueKind::Variable(name, Rc::new(Value::new(token.pos, ValueKind::Void)))
                }
                TokenKind::Push => ValueKind::Push,
                TokenKind::Pop => ValueKind::Pop,
            },
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
