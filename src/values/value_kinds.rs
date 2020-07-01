//! The ValueKind enum maintains the various values in the language.
//! All of the supported values are in this enum. This makes it easy to expand in the future.

use super::value::Value;
use std::{fmt, rc::Rc};

#[derive(PartialEq)]
pub enum ValueKind {
    Void,
    Any,
    Int(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Variable(String, Rc<Value>),
    Label(String),
    End,

    Push,
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Equal,
    NotEqual,
    Jump,
    RelativeJump,
    JumpIfTrue,
    JumpIfFalse,
    Print,
    PrintNewLine,
}

impl ValueKind {
    /// This function gets the name of the value.
    /// For example, an int with the value of 15, will have the value name 'Int'.
    /// This method is used to provide the right error messages.
    pub fn get_value_name(&self) -> String {
        match self {
            ValueKind::Void => "Void",
            ValueKind::Any => "Any",
            ValueKind::Int(_) => "Int",
            ValueKind::Float(_) => "Float",
            ValueKind::Boolean(_) => "Boolean",
            ValueKind::String(_) => "String",
            ValueKind::Variable(_, value) => return value.kind.get_value_name(),
            ValueKind::Label(_) => "Label",
            ValueKind::End => "End",
            ValueKind::Push => "Instruction Push",
            ValueKind::Pop => "Instruction Pop",
            ValueKind::Add => "Instruction Add",
            ValueKind::Sub => "Instruction Sub",
            ValueKind::Mul => "Instruction Mul",
            ValueKind::Div => "Instruction Div",
            ValueKind::LessThan => "Instruction LessThan",
            ValueKind::LessThanEqual => "Instruction LessThanEqual",
            ValueKind::GreaterThan => "Instruction GreaterThan",
            ValueKind::GreaterThanEqual => "Instruction GreaterThanEqual",
            ValueKind::Equal => "Instruction Equal",
            ValueKind::NotEqual => "Instruction NotEqual",
            ValueKind::Jump => "Instruction Jump",
            ValueKind::RelativeJump => "Instruction JumpRelative",
            ValueKind::JumpIfTrue => "Instruction JumpIfTrue",
            ValueKind::JumpIfFalse => "Instruction JumpIfFalse",
            ValueKind::Print => "Instruction Print",
            ValueKind::PrintNewLine => "Instruction PrintNewLine",
        }
        .to_owned()
    }
}

impl fmt::Debug for ValueKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueKind::Void => write!(f, "Void"),
            ValueKind::Any => write!(f, "Any"),
            ValueKind::Int(value) => write!(f, "{}", value),
            ValueKind::Float(value) => write!(f, "{}", value),
            ValueKind::Boolean(value) => write!(f, "{}", value),
            ValueKind::String(value) => write!(f, "{}", value),
            ValueKind::Variable(name, _) => write!(f, "Variable '{}'", name),
            ValueKind::Label(name) => write!(f, "Label '{}'", name),
            ValueKind::End => write!(f, "End"),
            ValueKind::Push => write!(f, "<instruction push>"),
            ValueKind::Pop => write!(f, "<instruction pop>"),
            ValueKind::Add => write!(f, "<instruction add>"),
            ValueKind::Sub => write!(f, "<instruction sub>"),
            ValueKind::Mul => write!(f, "<instruction mul>"),
            ValueKind::Div => write!(f, "<instruction div>"),
            ValueKind::LessThan => write!(f, "<instruction lt>"),
            ValueKind::LessThanEqual => write!(f, "<instruction lte>"),
            ValueKind::GreaterThan => write!(f, "<instruction gt>"),
            ValueKind::GreaterThanEqual => write!(f, "<instruction gte>"),
            ValueKind::Equal => write!(f, "<instruction eq>"),
            ValueKind::NotEqual => write!(f, "<instruction neq>"),
            ValueKind::Jump => write!(f, "<instruction jmp>"),
            ValueKind::RelativeJump => write!(f, "<instruction rjmp>"),
            ValueKind::JumpIfTrue => write!(f, "<instruction jmpt>"),
            ValueKind::JumpIfFalse => write!(f, "<instruction jmpf>"),
            ValueKind::Print => write!(f, "<instruction print>"),
            ValueKind::PrintNewLine => write!(f, "<instruction printn>"),
        }
    }
}
