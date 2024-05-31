pub enum ReadWhitespace {
    EOL,
    SPECIAL,
    SPACE,
    NONE,
}

impl ReadWhitespace {
    pub fn is_not_none(&self) -> bool {
        match self {
            ReadWhitespace::NONE => false,
            _ => true,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct Tok {
    pub token: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    EOL,
    IDENT(String),
    UNIT,

    INT,
    FLOAT,
    LONG_FLOAT,
    SUPER_LONG_FLOAT,
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
    ELSE,
    YEILD,
    DO,
    END,
    NOT,

    BAR,

    RIGHT_ARROW,
    LEFT_ARROW,
    COLON,
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
    WHILE,
    FOR,
    IN,
    MATCH,
    WITH,
    LCARET,
    RCARET,
    UNDERSCORE,

    NUMBER(String),
    FLOAT_LITERAL(f32),
    LONG_FLOAT_LITERAL(f64),
    INT_LITERAL(isize),
    USIZE_LITERAL(usize),
    STRING_LITERAL(String),

    AND,
    OR,
    LT,
    GT,

    COMMENT_START,
    COMMENT_END,

    TRUE,
    FALSE,
}
