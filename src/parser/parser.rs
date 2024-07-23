use super::{
    constructs,
    variable::{Variable, VariableGenerationError, VariableTypeId, VariableTypeLiteral},
};
use crate::{parser::constructs::ConditionType, Token, TokenType};

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
        Parser {
            cursor_position: 0,
            tokens,
        }
    }

    pub fn parse() {
        todo!()
    }

    pub fn read_to_semicolon(&mut self) -> &[Token] {
        let start = self.cursor_position;

        while self.get_token_under_cursor().token_type != TokenType::SEMICOLON {
            self.increment_cursor();
        }

        self.increment_cursor();
        &self.tokens[start..self.cursor_position]
    }

    pub fn increment_cursor(&mut self) {
        self.cursor_position += 1;
    }

    pub fn get_token_under_cursor(&self) -> &Token {
        &self.tokens[self.cursor_position]
    }

    pub fn get_cursor_position(&mut self) -> usize {
        self.cursor_position
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
                self.cursor_position = end;
                return Ok(&self.tokens[start..start + end]);
            }

            if self.cursor_position + 1 == self.tokens.len() {
                return Err(BlockScopeError::NoClosingScopeDelimiter);
            }
        }

        Err(BlockScopeError::NoScopeDelimiters)
    }

    pub fn read_to(&mut self, token: &str) -> &[Token] {
        let start = self.cursor_position;
        while self.get_token_under_cursor().literal != token {
            self.increment_cursor();
        }
        self.increment_cursor();

        &self.tokens[start..self.cursor_position]
    }

    /// this assumes that the cursor is on the 'let' or 'const' statement
    pub fn generate_variable(&mut self) -> Result<Variable, VariableGenerationError> {
        let mut variable = Variable {
            ..Default::default()
        };

        let mut type_token: Vec<&TokenType> = Vec::new();
        let value: Vec<&TokenType> = Vec::new();

        let tokens = self.read_to_semicolon();

        for (idx, token) in tokens.iter().enumerate() {
            match &token.token_type {
                TokenType::IDENT(name) => {
                    if variable.name.is_empty() {
                        variable.name = name.to_string();
                    }

                    if name.chars().collect::<Vec<char>>()[0] == '\"' {
                        let mut stri = String::new();

                        for tok in &tokens[idx..] {
                            println!("{:?}", stri);

                            if tok.token_type == TokenType::SEMICOLON {
                                variable.type_id = VariableTypeId::String;
                                println!("STRING: {:?}", stri);
                                variable.value = Some(VariableTypeLiteral::String(stri.clone()))
                            }
                            stri += &tok.literal;
                        }
                    }
                }

                TokenType::COLON => {
                    for tok in &tokens[idx + 1..] {
                        if tok.token_type == TokenType::ASSIGN {
                            break;
                        }

                        type_token.push(&tok.token_type);
                    }
                }
                TokenType::RESULT | TokenType::OPTION | TokenType::VECTOR => {
                    variable.type_id = Self::genereate_inner_type(type_token.clone())?;
                }

                TokenType::ASSIGN => {
                    continue;
                }

                TokenType::SEMICOLON => return Ok(variable),

                TokenType::NUMBER(t) => {
                    variable.value = Some(VariableTypeLiteral::Usize(t.parse().unwrap()))
                }

                TokenType::STRING_LITERAL(t) => {
                    variable.value = Some(VariableTypeLiteral::String(t.parse().unwrap()))
                }

                t => {
                    //println!("T: {:#?}", t);
                }
            }
        }

        Ok(variable)

        //Err(VariableGenerationError::InvalidSyntax)
    }

    pub fn genereate_inner_type(
        tokens: Vec<&TokenType>,
    ) -> Result<VariableTypeId, VariableGenerationError> {
        let current_token = tokens[0];

        match current_token {
            TokenType::RESULT => Ok(VariableTypeId::Result(Box::new(
                Self::genereate_inner_type(tokens[2..].to_vec()).unwrap(),
            ))),

            TokenType::VECTOR => Ok(VariableTypeId::Vector(Box::new(
                Self::genereate_inner_type(tokens[2..].to_vec()).unwrap(),
            ))),

            TokenType::OPTION => Ok(VariableTypeId::Option(Box::new(
                Self::genereate_inner_type(tokens[2..].to_vec()).unwrap(),
            ))),

            _ => Ok(Into::<VariableTypeId>::into(current_token.clone())),
        }
    }

    pub fn generate_statement_condition(
        &mut self,
    ) -> Result<constructs::StatementCondition, constructs::StatementGenerationError> {
        let mut condition = constructs::StatementCondition::default();

        let cond = Self::get_block_scope(self, TokenType::LPAREN, TokenType::RPAREN).unwrap();
        for token in &cond[1..cond.len() - 1] {
            match token.token_type {
                //TokenType::IDENT()
            }
        }

        todo!()
    }
}
