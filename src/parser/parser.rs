use std::{any::Any, default};

use crate::{methods::TokenMethods, Token, TokenType};

use super::variable::{Variable, VariableGenerationError, VariableType};

#[derive(Debug)]
pub enum BlockScopeError {
    NoScopeDelimiters,
    NoClosingScopeDelimiter,
}

#[derive(Debug)]
pub struct Parser {
    cursor_position: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        return Parser {
            cursor_position: 0,
            tokens,
        };
    }

    pub fn parse() {}

    pub fn read_to_semicolon(&mut self) -> &[Token] {
        let start = self.cursor_position;

        while self.get_token_under_cursor().token_type != TokenType::SEMICOLON {
            self.increment_cursor();
        }

        self.increment_cursor();
        return &self.tokens[start..self.cursor_position];
    }

    pub fn increment_cursor(&mut self) {
        self.cursor_position += 1;
    }

    pub fn get_token_under_cursor(&self) -> &Token {
        return &self.tokens[self.cursor_position];
    }

    pub fn get_cursor_position(&mut self) -> usize {
        return self.cursor_position;
    }

    pub fn get_block_scope(
        &mut self,
        opening_delim: TokenType,
        closing_delim: TokenType,
    ) -> Result<&[Token], BlockScopeError> {
        let (mut open_brace, mut closed_brace) = (0, 0);
        while self.tokens[self.cursor_position].token_type != opening_delim {
            self.increment_cursor();
        }

        let start = self.cursor_position;
        let mut end = 0;

        for token in &self.tokens[start..] {
            match &token.token_type {
                t if *t == opening_delim => open_brace += 1,
                t if *t == closing_delim => closed_brace += 1,
                _ => {}
            }

            end += 1;

            if open_brace == closed_brace {
                println!("{:?}", end);
                self.cursor_position = end;
                return Ok(&self.tokens[start..start + end]);
            }

            if self.cursor_position + 1 == self.tokens.len() {
                return Err(BlockScopeError::NoClosingScopeDelimiter);
            }
        }

        return Err(BlockScopeError::NoScopeDelimiters);
    }

    pub fn read_to(&mut self, token: &str) -> &[Token] {
        let start = self.cursor_position;
        while self.get_token_under_cursor().literal != token {
            self.increment_cursor();
        }
        self.increment_cursor();

        return &self.tokens[start..self.cursor_position];
    }

    /// this assumes that the cursor is on the 'let' or 'const' statement
    pub fn generate_variable(&mut self) -> Result<Variable, VariableGenerationError> {
        let mut variable = Variable {
            ..Default::default()
        };

        let mut type_token: Vec<&TokenType> = Vec::new();
        let mut value: Vec<&TokenType> = Vec::new();

        let mut name_seen: bool = false;
        let tokens = self.read_to_semicolon();

        for (idx, token) in tokens.iter().enumerate() {
            if let TokenType::IDENT(ref name) = token.token_type {
                if !name_seen {
                    variable.name = name.to_string();
                    name_seen = true;
                }
            }

            if token.token_type == TokenType::COLON {
                for tok in &tokens[idx + 1..] {
                    if tok.token_type == TokenType::ASSIGN {
                        break;
                    }

                    type_token.push(&tok.token_type);
                }
            }

            if token.token_type == TokenType::ASSIGN {
                tokens[idx..]
                    .iter()
                    .for_each(|tok| value.push(&tok.token_type))
            }
        }

        println!("name: {:?}, type_token: {:?}", variable.name, type_token);

        Err(VariableGenerationError::InvalidSyntax)
    }

    pub fn compact_and_generate_variable_type(
        tokens: &[Token],
    ) -> Result<VariableType, VariableGenerationError> {
        todo!()
    }
}
