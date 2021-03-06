//! The TokenKind enum maintains all of the different Tokens that could occur within the program.
//! Using an enum allows for easy extensibility.

use crate::utils::parameter::Parameter;

#[derive(Debug)]
pub enum TokenKind {
    Void,
    Any,
    IntegerLiteral(i64),
    FloatLiteral(f64),
    BooleanLiteral(bool),
    StringLiteral(String),
    Identifier(String),
    Label(String, Vec<Parameter>),
    End,

    Push,
    Pop,
    Peek,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
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
    RelativeJumpIfTrue,
    RelativeJumpIfFalse,
    Print,
    PrintNewLine,
    Set,
    Call,
}

impl TokenKind {
    /// This function checks if the given name is an instrution. If it is, it returns the correct instruction token.
    /// This function is called by the lexer, specifically by the make_word function.
    ///
    /// # Arguments
    /// `name` - The name of the current word.
    pub fn is_instruction(name: &str) -> Option<TokenKind> {
        match name.to_ascii_lowercase().as_str() {
            "push" => Some(TokenKind::Push),
            "pop" => Some(TokenKind::Pop),
            "peek" => Some(TokenKind::Peek),
            "add" => Some(TokenKind::Add),
            "sub" => Some(TokenKind::Sub),
            "mul" => Some(TokenKind::Mul),
            "div" => Some(TokenKind::Div),
            "mod" => Some(TokenKind::Mod),
            "lt" => Some(TokenKind::LessThan),
            "lte" => Some(TokenKind::LessThanEqual),
            "gt" => Some(TokenKind::GreaterThan),
            "gte" => Some(TokenKind::GreaterThanEqual),
            "eq" => Some(TokenKind::Equal),
            "neq" => Some(TokenKind::NotEqual),
            "jmp" => Some(TokenKind::Jump),
            "rjmp" => Some(TokenKind::RelativeJump),
            "jmpt" => Some(TokenKind::JumpIfTrue),
            "jmpf" => Some(TokenKind::JumpIfFalse),
            "rjmpt" => Some(TokenKind::RelativeJumpIfTrue),
            "rjmpf" => Some(TokenKind::RelativeJumpIfFalse),
            "print" => Some(TokenKind::Print),
            "printn" => Some(TokenKind::PrintNewLine),
            "set" => Some(TokenKind::Set),
            "call" => Some(TokenKind::Call),

            _ => None,
        }
    }
}
