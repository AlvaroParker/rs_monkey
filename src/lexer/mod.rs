mod lexer;
pub use lexer::Lexer;

#[test]
pub fn test_next_token() {
    use crate::lexer::lexer::Lexer;
    use crate::token::{Token, TokenType};
    let input = r#"
    let five = 5;
    let ten = 10;

    let add = fn(x, y) {
        x + y;
    };

    let result = add(five, ten);
    !-/*5;
    5 < 10 > 5;
    if (5 < 10) {
        return true;
    } else {
        return false;
    }

    10 == 10;
    5 != 10;
    "#;

    let expected = vec![
        Token::new(TokenType::LET, "let"),
        Token::new(TokenType::IDENT, "five"),
        Token::new(TokenType::ASSIGN, "="),
        Token::new(TokenType::INT, "5"),
        Token::new(TokenType::SEMICOLON, ";"),
        Token::new(TokenType::LET, "let"),
        Token::new(TokenType::IDENT, "ten"),
        Token::new(TokenType::ASSIGN, "="),
        Token::new(TokenType::INT, "10"),
        Token::new(TokenType::SEMICOLON, ";"),
        Token::new(TokenType::LET, "let"),
        Token::new(TokenType::IDENT, "add"),
        Token::new(TokenType::ASSIGN, "="),
        Token::new(TokenType::FUNCTION, "fn"),
        Token::new(TokenType::LPAREN, "("),
        Token::new(TokenType::IDENT, "x"),
        Token::new(TokenType::COMMA, ","),
        Token::new(TokenType::IDENT, "y"),
        Token::new(TokenType::RPAREN, ")"),
        Token::new(TokenType::LBRACE, "{"),
        Token::new(TokenType::IDENT, "x"),
        Token::new(TokenType::PLUS, "+"),
        Token::new(TokenType::IDENT, "y"),
        Token::new(TokenType::SEMICOLON, ";"),
        Token::new(TokenType::RBRACE, "}"),
        Token::new(TokenType::SEMICOLON, ";"),
        Token::new(TokenType::LET, "let"),
        Token::new(TokenType::IDENT, "result"),
        Token::new(TokenType::ASSIGN, "="),
        Token::new(TokenType::IDENT, "add"),
        Token::new(TokenType::LPAREN, "("),
        Token::new(TokenType::IDENT, "five"),
        Token::new(TokenType::COMMA, ","),
        Token::new(TokenType::IDENT, "ten"),
        Token::new(TokenType::RPAREN, ")"),
        Token::new(TokenType::SEMICOLON, ";"),
        Token::new(TokenType::BANG, "!"),
        Token::new(TokenType::MINUS, "-"),
        Token::new(TokenType::SLASH, "/"),
        Token::new(TokenType::ASTERISK, "*"),
        Token::new(TokenType::INT, "5"),
        Token::new(TokenType::SEMICOLON, ";"),
        Token::new(TokenType::INT, "5"),
        Token::new(TokenType::LT, "<"),
        Token::new(TokenType::INT, "10"),
        Token::new(TokenType::GT, ">"),
        Token::new(TokenType::INT, "5"),
        Token::new(TokenType::SEMICOLON, ";"),
        Token::new(TokenType::IF, "if"),
        Token::new(TokenType::LPAREN, "("),
        Token::new(TokenType::INT, "5"),
        Token::new(TokenType::LT, "<"),
        Token::new(TokenType::INT, "10"),
        Token::new(TokenType::RPAREN, ")"),
        Token::new(TokenType::LBRACE, "{"),
        Token::new(TokenType::RETURN, "return"),
        Token::new(TokenType::TRUE, "true"),
        Token::new(TokenType::SEMICOLON, ";"),
        Token::new(TokenType::RBRACE, "}"),
        Token::new(TokenType::ELSE, "else"),
        Token::new(TokenType::LBRACE, "{"),
        Token::new(TokenType::RETURN, "return"),
        Token::new(TokenType::FALSE, "false"),
        Token::new(TokenType::SEMICOLON, ";"),
        Token::new(TokenType::RBRACE, "}"),
        Token::new(TokenType::INT, "10"),
        Token::new(TokenType::EQ, "=="),
        Token::new(TokenType::INT, "10"),
        Token::new(TokenType::SEMICOLON, ";"),
        Token::new(TokenType::INT, "5"),
        Token::new(TokenType::NOT_EQ, "!="),
        Token::new(TokenType::INT, "10"),
        Token::new(TokenType::SEMICOLON, ";"),
        Token::new(TokenType::EOF, ""),
    ];

    let mut lexer = Lexer::new(input.into());

    for i in expected {
        let tok = lexer.next_token();

        if tok.token_type != i.token_type {
            panic!(
                "Expected token type {:?}, got {:?}\nLexer: {:?}",
                i.token_type, tok.token_type, lexer
            );
        }

        if tok.literal != i.literal {
            panic!("Expected literal {:?}, got {:?}", i.literal, tok.literal);
        }
    }
}
