use crate::{Token, TokenType};
use core::ops::Range;

pub trait TokenMethods {
    fn token_types(&self, range: Range<usize>) -> Vec<&TokenType>;
    fn literals(&self, range: Range<usize>) -> Vec<&str>;
}

impl TokenMethods for &[Token] {
    fn token_types(&self, range: Range<usize>) -> Vec<&TokenType> {
        let mut token_types: Vec<&TokenType> = Vec::with_capacity(range.len() + 1);
        self[range]
            .iter()
            .for_each(|token| token_types.push(&token.token_type));
        return token_types;
    }

    fn literals(&self, range: Range<usize>) -> Vec<&str> {
        let mut literals: Vec<&str> = Vec::with_capacity(range.len() + 1);
        self[range]
            .iter()
            .for_each(|token| literals.push(&token.literal));
        return literals;
    }
}
