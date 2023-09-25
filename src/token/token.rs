use core::fmt;
use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let m = HashMap::from([
            ("fn", TokenType::FUNCTION),
            ("let", TokenType::LET),
            ("true", TokenType::TRUE),
            ("false", TokenType::FALSE),
            ("if", TokenType::IF),
            ("else", TokenType::ELSE),
            ("return", TokenType::RETURN),
        ]);
        m
    };
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new<T>(token_type: TokenType, ch: T) -> Self
    where
        T: fmt::Display,
    {
        Token {
            token_type,
            literal: ch.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT,
    INT,

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LT,
    GT,

    EQ,
    #[allow(non_camel_case_types)]
    NOT_EQ,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

pub fn lookup_ident(ident: &str) -> TokenType {
    if let Some(token_type) = KEYWORDS.get(ident) {
        return token_type.clone();
    }
    return TokenType::IDENT;
}
