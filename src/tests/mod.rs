#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::*, tokens::*};

    #[test]
    pub fn test_char_read() {
        let mut input = String::from("Let x = 3");

        let lexer = Lexer::new(&mut input);
        let expected_character = 'L' as u8;

        assert_eq!(lexer.char_under_cusor(), expected_character)
    }

    #[test]
    pub fn test_read_to_whitespace() {
        let mut input = String::from("let x = 3");
        let expected = String::from("let")
            .chars()
            .into_iter()
            .collect::<Vec<char>>()
            .into_iter()
            .map(|f| f as u8)
            .collect::<Vec<u8>>();

        let mut l = Lexer::new(&mut input);

        assert_eq!(
            l.read_to_whitespace_or_special().0,
            expected,
            "\nExpected: {:?}\nFound: {:?}\n",
            expected,
            l.read_to_whitespace_or_special().0,
        );
    }
    #[test]
    pub fn test_read_to_whitespace_with_special() {
        let mut input = String::from("let;");
        let expected = (
            String::from("let;")
                .chars()
                .into_iter()
                .collect::<Vec<char>>()
                .into_iter()
                .map(|f| f as u8)
                .collect::<Vec<u8>>(),
            Some(()),
        );

        let mut l = Lexer::new(&mut input);

        assert_eq!(l.read_to_whitespace_or_special(), expected)
    }

    #[test]
    pub fn read_to_eol() {
        let mut i = String::from("let add = fn(x,y) { return x + y };");
        let e = vec![
            "let", "add", "=", "fn", "(", "x", ",", "y", ")", "{", "return", "x", "+", "y", "}",
            ";",
        ];

        let mut l = Lexer::new(&mut i);
        let r = l.read_to_eol();
        assert_eq!(r, e, "Found: {:?}, Expected: {:?}", r, e);
    }

    #[test]
    pub fn read_test1() {
        let mut i = String::from("if i == 3 -> yeild fruit;");
        let e = vec!["if", "i", "==", "3", "->", "yeild", "fruit", ";"];

        let mut l = Lexer::new(&mut i);
        let r = l.read_to_eol();
        println!("{:?}", r);
        assert_eq!(r, e, "Found: {:?}, Expected: {:?}", r, e);
    }

    #[test]
    pub fn read_test2() {
        let mut i = String::from(
            "
            fun fib = rec (num: int) : (); do
                nums_list :: append match num with n 
                | 0 -> 0;
                | 1 -> 1;
                | _ -> fib(n - 1) + fib(n - 2);
            end;",
        );

        let e = vec![
            "fun",
            "fib",
            "=",
            "rec",
            "(",
            "num",
            ":",
            "int",
            ")",
            ":",
            "(",
            ")",
            ";",
            "do",
            "nums_list",
            "::",
            "append",
            "match",
            "num",
            "with",
            "n",
            "|",
            "0",
            "->",
            "0",
            ";",
            "|",
            "1",
            "->",
            "1",
            ";",
            "|",
            "_",
            "->",
            "fib",
            "(",
            "n",
            "-",
            "1",
            ")",
            "+",
            "fib",
            "(",
            "n",
            "-",
            "2",
            ")",
            ";",
            "end",
            ";",
        ];

        let mut l = Lexer::new(&mut i);
        let r = l.read_to_eof();
    }

    #[test]
    pub fn test_tokens() {
        let mut i = String::from("let add = fn(x,y) { return x + y };");
        let e: Vec<Token> = vec![
            Token {
                token_type: TokenType::LET,
                literal: "let",
            },
            Token {
                token_type: TokenType::IDENT("add"),
                literal: "add",
            },
            Token {
                token_type: TokenType::ASSIGN,
                literal: "=",
            },
            Token {
                token_type: TokenType::FUNCTION,
                literal: "fn",
            },
            Token {
                token_type: TokenType::LPAREN,
                literal: "(",
            },
            Token {
                token_type: TokenType::IDENT("x"),
                literal: "x",
            },
            Token {
                token_type: TokenType::COMMA,
                literal: ",",
            },
            Token {
                token_type: TokenType::IDENT("y"),
                literal: "y",
            },
            Token {
                token_type: TokenType::RPAREN,
                literal: ")",
            },
            Token {
                token_type: TokenType::LBRACE,
                literal: "{",
            },
            Token {
                token_type: TokenType::IDENT("return"),
                literal: "return",
            },
            Token {
                token_type: TokenType::IDENT("x"),
                literal: "x",
            },
            Token {
                token_type: TokenType::ADD,
                literal: "+",
            },
            Token {
                token_type: TokenType::IDENT("y"),
                literal: "y",
            },
            Token {
                token_type: TokenType::RBRACE,
                literal: "}",
            },
            Token {
                token_type: TokenType::SEMICOLON,
                literal: ";",
            },
        ];

        let mut l = Lexer::new(&mut i);
        let r = l.read_and_tokenize_input();
        assert_eq!(r, e, "Found: {:#?}, Expected: {:#?}", r, e);
    }
}
