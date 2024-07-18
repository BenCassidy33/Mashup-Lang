use crate::parser::variable::{VariableGenerationError, VariableTypeId};

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

    RESULT,
    OPTION,

    VECTOR,
}

impl From<TokenType> for VariableTypeId {
    fn from(token: TokenType) -> VariableTypeId {
        println!("{:#?}", token);
        match token {
            TokenType::INT => VariableTypeId::Int,
            TokenType::FLOAT => VariableTypeId::Float,
            TokenType::USIZE => VariableTypeId::Usize,
            TokenType::STRING => VariableTypeId::String,
            TokenType::BOOL => VariableTypeId::Bool,
            TokenType::CHAR => VariableTypeId::Char,
            TokenType::VOID => VariableTypeId::Unit,
            TokenType::RESULT => VariableTypeId::Result(Box::new(VariableTypeId::Unit)),
            TokenType::OPTION => VariableTypeId::Option(Box::new(VariableTypeId::Unit)),
            TokenType::VECTOR => VariableTypeId::Vector(Box::new(VariableTypeId::Unit)),

            _ => panic!("invalid type"),
        }
    }
}
