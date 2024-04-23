#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    EOL,

    IDENT(String),

    INT,
    FLOAT,
    USIZE,
    STRING,
    BOOL,
    CHAR,
    VOID,

    STRUCT,
    ENUM,

    F_STRING,

    STATIC,
    DYNAMIC,
    AUTO,

    SOME,
    NONE,

    ASSIGN,
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
    MUT,
    PRED,
    IF,
    YEILD,
    DO,
    END,
    NOT,

    BAR,

    RIGHT_ARROW,
    LEFT_ARROW,
    DOUBLE_COLON,

    ADD,
    SUB,
    MULT,
    DIV,
    POW,
    SQRT,
    MOD,

    RETURN,
    THEN,

    COMMENT,
}
