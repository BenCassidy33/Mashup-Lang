use std::default;

use crate::{Token, TokenType};

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

        for token in self.read_to_semicolon() {
            match &token.token_type {
                TokenType::LET => continue,
                TokenType::IDENT(n) => {
                    if variable.name.is_empty() {
                        variable.name = n.to_string();
                    } else {
                        todo!("Something goes here")
                    }
                }
                TokenType::COLON => {
                    let type = self.read_to("=");
                }

                _ => todo!(),
            };
        }
        //let type = compact_and_generate_variable_type()
        //
        todo!()
    }

    pub fn compact_and_generate_variable_type(
        tokens: &[Token],
    ) -> Result<VariableType, VariableGenerationError> {
        todo!()
    }
}
