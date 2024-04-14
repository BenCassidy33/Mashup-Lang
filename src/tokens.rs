pub struct Token<'a> {
    literal: &'a str,
    token_type: TokenType,
}

pub enum TokenType {
    ILLEGAL,
    EOF,

    IDENT(String),
    INT,
    USIZE,
    STRING,

    ASSIGN,
    PLUS,
    MINUS,
    EQUALS,

    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,

    FUNCTION,
    LET,
}
