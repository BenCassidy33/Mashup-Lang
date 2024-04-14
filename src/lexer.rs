use std::collections::HashSet;

use crate::tokens::TokenType;
use crate::utils::{self, SpecialImplmentation};

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
    pub fn new(input: &String) -> Lexer<'_> {
        let _ = input.replace("\n", "");
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
    'character' in the Some(character) is the special character.
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

    pub fn read_to_end_of_line(&mut self) -> Vec<String> {
        let mut tokens_to_be_parsed: Vec<String> = Vec::new();

        loop {
            let curr = self.read_to_whitespace_or_special();
            tokens_to_be_parsed.push(
                curr.0
                    .iter()
                    .map(|f| *f as char)
                    .collect::<Vec<char>>()
                    .iter()
                    .collect::<String>(),
            );

            if curr.1.is_some() && curr.0.len() == 1 && curr.0[0] == ';' as u8 {
                let specials: HashSet<_> = utils::SPECIAL_CHARACTERS.iter().cloned().collect();
                let mut parsed_elements: Vec<String> = Vec::new();

                for elem in tokens_to_be_parsed {
                    if let Some((index, _)) = elem
                        .chars()
                        .enumerate()
                        .find(|(_, c)| specials.contains(&c))
                    {
                        parsed_elements.push(elem.split_at(index).0.to_string());
                        parsed_elements.push(elem.split_at(index).1.to_string());
                    } else {
                        parsed_elements.push(elem)
                    }
                }

                return parsed_elements.into_iter().filter(|f| *f != "").collect();
            }
        }
    }
}
