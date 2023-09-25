use crate::token::{lookup_ident, Token, TokenType};

#[derive(Debug, Clone, PartialEq)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0 as char,
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0 as char;
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.ch {
            '=' => {
                if let Some('=') = self.peek_char() {
                    self.read_char();
                    Token::new(TokenType::EQ, "==")
                } else {
                    Token::new(TokenType::ASSIGN, self.ch)
                }
            }
            '+' => Token::new(TokenType::PLUS, self.ch),
            '-' => Token::new(TokenType::MINUS, self.ch),
            '!' => {
                if let Some('=') = self.peek_char() {
                    self.read_char();
                    Token::new(TokenType::NOT_EQ, "!=")
                } else {
                    Token::new(TokenType::BANG, self.ch)
                }
            }
            '/' => Token::new(TokenType::SLASH, self.ch),
            '*' => Token::new(TokenType::ASTERISK, self.ch),
            '<' => Token::new(TokenType::LT, self.ch),
            '>' => Token::new(TokenType::GT, self.ch),
            ';' => Token::new(TokenType::SEMICOLON, self.ch),
            ',' => Token::new(TokenType::COMMA, self.ch),
            '(' => Token::new(TokenType::LPAREN, self.ch),
            ')' => Token::new(TokenType::RPAREN, self.ch),
            '{' => Token::new(TokenType::LBRACE, self.ch),
            '}' => Token::new(TokenType::RBRACE, self.ch),
            '\0' => Token::new(TokenType::EOF, ""),
            _ => {
                if self.ch.is_ascii_alphabetic() {
                    let identifier = self.read_identifier();
                    let token_type = lookup_ident(&identifier);
                    return Token::new(token_type, identifier);
                } else if self.ch.is_digit(10) {
                    let literal = self.read_number();
                    return Token::new(TokenType::INT, literal);
                } else {
                    Token::new(TokenType::ILLEGAL, self.ch)
                }
            }
        };
        self.read_char();
        tok
    }

    pub fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_alphabetic() {
            self.read_char();
        }
        return self.input[position..self.position].to_string();
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    pub fn read_number(&mut self) -> String {
        let position = self.position;

        while self.ch.is_digit(10) {
            self.read_char();
        }
        return self.input[position..self.position].to_string();
    }

    fn peek_char(&self) -> Option<char> {
        self.input.chars().nth(self.read_position)
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        let token = match self.ch {
            '=' => Token::new(TokenType::ASSIGN, self.ch),
            ';' => Token::new(TokenType::SEMICOLON, self.ch),
            '(' => Token::new(TokenType::LPAREN, self.ch),
            ')' => Token::new(TokenType::RPAREN, self.ch),
            ',' => Token::new(TokenType::COMMA, self.ch),
            '+' => Token::new(TokenType::PLUS, self.ch),
            '{' => Token::new(TokenType::LBRACE, self.ch),
            '}' => Token::new(TokenType::RBRACE, self.ch),
            '\0' => Token::new(TokenType::EOF, ""),
            _ => {
                let tok = if self.ch.is_ascii_alphabetic() {
                    let identifier = self.read_identifier();
                    let token_type = lookup_ident(&identifier);
                    Token::new(token_type, identifier)
                } else if self.ch.is_digit(10) {
                    let literal = self.read_number();
                    Token::new(TokenType::INT, literal)
                } else {
                    Token::new(TokenType::ILLEGAL, self.ch)
                };
                return Some(tok);
            }
        };
        self.read_char();
        return Some(token);
    }
}
