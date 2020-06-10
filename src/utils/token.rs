#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: usize,
}

impl Token {
    pub fn new(kind: TokenKind, pos: usize) -> Token {
        Token { kind, pos }
    }
}

#[derive(Debug)]
pub enum TokenKind {
    Void,
    Any,
    IntegerLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    Identifier(String),
    Push,
}

impl TokenKind {
    pub fn is_instruction(name: &String) -> Option<TokenKind> {
        match name.to_ascii_lowercase().as_str() {
            "push" => Some(TokenKind::Push),
            _ => None,
        }
    }
}
