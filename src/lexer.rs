use std::collections::{hash_set, HashSet};
use std::process::exit;

use crate::tokens::{Token, TokenType};
use crate::utils::{self, SpecialImplmentation, VecU8Impl, SPECIAL_CHARACTERS};

pub struct Lexer<'a> {
    input: &'a String,
    size: usize,
    cursor_pos: usize,
}

impl<'a> Default for Lexer<'_> {
    fn default() -> Self {
        static EMPTY: String = String::new();

        Lexer {
            input: &EMPTY,
            size: 0,
            cursor_pos: 0,
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &mut String) -> Lexer<'_> {
        let _ = input.retain(|c| c != '\n');

        return Lexer {
            input,
            size: input.len(),
            ..Default::default()
        };
    }

    pub fn char_under_cusor(&self) -> u8 {
        return self.input.chars().nth(self.cursor_pos).unwrap() as u8;
    }

    pub fn increment_cursor(&mut self) -> Result<(), &'static str> {
        if self.cursor_pos == self.size - 1 {
            return Err("Attempt to index cursor out of range.");
        } else {
            self.cursor_pos += 1;
            return Ok(());
        }
    }

    /**
    Reads to whitespace and returns a vector of the characters in a Vec<u8>.
    Returns (Vec<u8>, Some(character)) if the cursor runs into a special character where the
    'character' in the Some(()) is the special character, returns None otherwise
    */
    pub fn read_to_whitespace_or_special(&mut self) -> (Vec<u8>, Option<()>) {
        let mut characters = Vec::new();

        loop {
            let curr = self.char_under_cusor();

            if curr.is_ascii_whitespace() {
                break;
            } else if u8::is_special_character(curr) {
                characters.push(curr);
                let _ = self.increment_cursor();
                return (characters, Some(()));
            }

            characters.push(curr);
            let _ = self.increment_cursor();
        }

        let _ = self.increment_cursor();
        return (characters, None);
    }

    pub fn read_to_eol(&mut self) -> Vec<String> {
        let mut tokens: Vec<String> = Vec::new();

        loop {
            let (token, has_special) = self.read_to_whitespace_or_special();

            if has_special.is_some() {
                let left = token[0..token.len() - 1].to_vec().to_string().unwrap();
                let right = (token[token.len() - 1] as char).to_string();

                if right == ";" {
                    tokens.push(left);
                    tokens.push(right);

                    return tokens.into_iter().filter(|c| !c.is_empty()).collect();
                } else {
                    tokens.push(left);
                    tokens.push(right);
                }
            } else {
                tokens.push(token.to_string().unwrap());
            }
        }
    }

    pub fn read_to_eof(&mut self) -> Vec<String> {
        let mut lines = Vec::new();

        while self.cursor_pos < self.size - 1 {
            println!(
                "Cursor Position: {:?}, Size: {:?}",
                self.cursor_pos, self.size
            );

            lines.append(&mut self.read_to_eol());
        }

        return lines;
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let raw = self.read_to_eof();

        for item in raw {
            let token_type = Lexer::generate_token_type(item.clone());

            tokens.push(Token {
                token_type,
                literal: item,
            });
        }

        return tokens;
    }

    pub fn generate_token_type(input: String) -> TokenType {
        return match input.as_str() {
            "int" => TokenType::INT,
            "float" => TokenType::FLOAT,
            "usize" => TokenType::USIZE,
            "string" => TokenType::STRING,
            "bool" => TokenType::BOOL,
            "char" => TokenType::CHAR,

            "struct" => TokenType::STRUCT,
            "enum" => TokenType::ENUM,
            "f" => TokenType::F_STRING,

            "static" => TokenType::STATIC,
            "dynamic" => TokenType::DYNAMIC,
            "auto" => TokenType::AUTO,

            "some" => TokenType::SOME,
            "none" => TokenType::NONE,

            "=" => TokenType::ASSIGN,
            "==" => TokenType::EQUALS,
            "+" => TokenType::ADD,

            "," => TokenType::COMMA,
            ";" => TokenType::SEMICOLON,

            "(" => TokenType::LPAREN,
            ")" => TokenType::RPAREN,
            "[" => TokenType::LBRACKET,
            "]" => TokenType::RBRACKET,
            "{" => TokenType::LBRACE,
            "}" => TokenType::RBRACE,

            "fun" => TokenType::FUNCTION,
            "yeild" => TokenType::YEILD,
            "let" => TokenType::LET,
            "then" => TokenType::THEN,

            _ => {
                if !input.is_ascii() {
                    return TokenType::ILLEGAL;
                } else {
                    return TokenType::IDENT(input.clone());
                }
            }
        };
    }
}
