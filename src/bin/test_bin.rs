use lang::{parser::parser::Parser, Lexer, Token, TokenType};

fn main() {
    let tokens = Lexer::new(&mut String::from(
        "function test { if true { } }; let other = 5;",
    ))
    .lex();
    let mut parser = Parser::new(tokens);
    let scope = parser.get_block_scope(TokenType::LBRACE, TokenType::RBRACE);

    let expected: &[Token] = &[
        Token {
            token_type: TokenType::LBRACE,
            start: 14,
            end: 15,
            literal: "{".to_string(),
        },
        Token {
            token_type: TokenType::IF,
            start: 15,
            end: 16,
            literal: "if".to_string(),
        },
        Token {
            token_type: TokenType::TRUE,
            start: 16,
            end: 20,
            literal: "true".to_string(),
        },
        Token {
            token_type: TokenType::LBRACE,
            start: 20,
            end: 21,
            literal: "{".to_string(),
        },
        Token {
            token_type: TokenType::RBRACE,
            start: 21,
            end: 22,
            literal: "}".to_string(),
        },
        Token {
            token_type: TokenType::RBRACE,
            start: 22,
            end: 23,
            literal: "}".to_string(),
        },
    ];
}
