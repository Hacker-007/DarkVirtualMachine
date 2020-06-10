use crate::{
    utils::error::{Error, ErrorKind},
    utils::token::{Token, TokenKind},
};

use std::{collections::VecDeque, iter::Peekable, str::Chars};

pub struct Lexer {
    current_position: usize,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            current_position: 0,
        }
    }

    pub fn lex(&mut self, contents: &str) -> Result<VecDeque<Token>, Error> {
        let mut iter = contents.chars().peekable();
        let mut tokens = VecDeque::new();
        while let Some(ch) = iter.next() {
            self.current_position += 1;

            if ch.is_ascii_whitespace() || self.handle_comments(ch, &mut iter) {
                continue;
            }

            match ch {
                '0'..='9' | '-' => tokens.push_back(self.make_number(ch, &mut iter)?),
                '\'' | '"' => tokens.push_back(self.make_string(ch, &mut iter)?),
                letter if ch.is_ascii_alphabetic() => {
                    tokens.push_back(self.make_word(letter, &mut iter))
                }
                _ => {
                    return Err(Error::new(
                        ErrorKind::UnknownCharacter,
                        self.current_position,
                    ))
                }
            }
        }

        Ok(tokens)
    }

    fn make_number(&mut self, digit: char, iter: &mut Peekable<Chars>) -> Result<Token, Error> {
        let initial_point = self.current_position;
        let mut number = digit.to_string();
        let mut has_decimal_point = false;
        while let Some(ch) = iter.peek() {
            if ch.is_ascii_digit() {
                number.push(self.advance(iter));
            } else if ch == &'.' && !has_decimal_point {
                number.push(self.advance(iter));
                has_decimal_point = true;
            } else {
                break;
            }
        }

        if !has_decimal_point {
            if let Ok(value) = number.parse() {
                Ok(Token::new(TokenKind::IntegerLiteral(value), initial_point))
            } else {
                Err(Error::new(
                    ErrorKind::InvalidNumberFormat,
                    self.current_position,
                ))
            }
        } else {
            if let Ok(value) = number.parse() {
                Ok(Token::new(TokenKind::FloatLiteral(value), initial_point))
            } else {
                Err(Error::new(
                    ErrorKind::InvalidNumberFormat,
                    self.current_position,
                ))
            }
        }
    }

    fn make_word(&mut self, letter: char, iter: &mut Peekable<Chars>) -> Token {
        let initial_point = self.current_position;
        let mut word = letter.to_string();
        while let Some(ch) = iter.peek() {
            if ch.is_ascii_whitespace() {
                self.advance(iter);
                break;
            } else {
                word.push(self.advance(iter));
            }
        }

        if word.eq_ignore_ascii_case(&"void") {
            Token::new(TokenKind::Void, initial_point)
        } else if word.eq_ignore_ascii_case(&"any") {
            Token::new(TokenKind::Any, initial_point)
        } else if let Some(instruction) = TokenKind::is_instruction(&word) {
            Token::new(instruction, initial_point)
        } else {
            Token::new(TokenKind::Identifier(word), initial_point)
        }
    }

    fn make_string(
        &mut self,
        beginning_of_string: char,
        iter: &mut Peekable<Chars>,
    ) -> Result<Token, Error> {
        let initial_point = self.current_position;
        let mut string = beginning_of_string.to_string();
        while let Some(ch) = iter.peek() {
            if ch == &beginning_of_string {
                string.push(self.advance(iter));
                break;
            } else {
                string.push(self.advance(iter));
            }
        }

        if !string.ends_with(beginning_of_string) {
            Err(Error::new(ErrorKind::UnterminatedString, initial_point))
        } else {
            Ok(Token::new(TokenKind::StringLiteral(string), initial_point))
        }
    }

    fn handle_comments(&mut self, ch: char, iter: &mut Peekable<Chars>) -> bool {
        if ch == '-' {
            match iter.peek() {
                Some('-') => {
                    self.handle_single_line_comments(iter);
                    true
                }
                Some('!') => {
                    self.handle_multi_line_comments(iter);
                    true
                }
                _ => false,
            }
        } else {
            false
        }
    }

    fn handle_single_line_comments(&mut self, iter: &mut Peekable<Chars>) {
        self.advance(iter);
        while let Some(c) = iter.next() {
            self.current_position += 1;
            if c == '\n' {
                break;
            }
        }
    }

    fn handle_multi_line_comments(&mut self, iter: &mut Peekable<Chars>) {
        self.advance(iter);
        while let Some(c) = iter.next() {
            self.current_position += 1;
            if c == '!' {
                if let Some('-') = iter.peek() {
                    self.advance(iter);
                    break;
                }
            }
        }
    }

    fn advance(&mut self, iter: &mut Peekable<Chars>) -> char {
        self.current_position += 1;
        iter.next().unwrap()
    }
}
