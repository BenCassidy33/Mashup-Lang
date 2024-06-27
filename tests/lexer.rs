#![allow(warnings)]
#[cfg(test)]
mod tests {
    use crate::expected_outputs;
    use lang::lexer::Lexer;
    use lang::utils::tokens::*;
    use pretty_assertions::assert_eq;

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
        let r = l
            .read_to_eol()
            .iter()
            .map(|f| f.clone())
            .collect::<Vec<String>>();
        assert_eq!(r, e, "Found: {:?}, Expected: {:?}", r, e);
    }

    #[test]
    pub fn read_test1() {
        let mut i = String::from("if i == 3 -> yeild fruit;");
        let e = vec!["if", "i", "==", "3", "->", "yeild", "fruit", ";"];

        let mut l = Lexer::new(&mut i);
        let r = l.read_to_eol();
        // println!("{:?}", r);
        // assert_eq!(r, e, "Found: {:?}, Expected: {:?}", r, e);
    }

    #[test]
    pub fn test_start_end_poss() {
        let mut i = String::from("if i == 3 -> yeild fruit;");
        let e = vec![
            (0, 2),
            (3, 4),
            (5, 7),
            (8, 9),
            (10, 11),
            (12, 17),
            (18, 23),
            (24, 25),
        ];

        let r = Lexer::new(&mut i).read_to_eol();
    }
    #[ignore]
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

        let mut r = Lexer::new(&mut i).read_to_eof();
        //assert_eq!(e, r, "Expected: \n{:?}, \nFound:\n {:?}\n\n", e, r);
    }

    #[ignore]
    #[test]
    pub fn test_tokens() {
        println!("Testing");

        let mut i = String::from("let add = fun(x,y) -> int; do yeild x + y; end;");
        let mut e = expected_outputs::expected_token_test();
        let mut r = Lexer::new(&mut i).lex();
        assert_eq!(e, r, "\n\nExpected: {:#?},\n\nFound: {:#?}\n\n", e, r);
    }

    #[ignore]
    #[test]
    pub fn test_tokens_2() {
        let mut i = String::from(
            "
            fun fib = rec (num: int) : (); do
                nums_list :: append match num with n 
                | 0 -> 0;
                | 1 -> 1;
                | _ -> fib(n - 1) + fib(n - 2);
            end;",
        );

        let mut r = Lexer::new(&mut i).lex();
        let e = expected_outputs::expected_token_test_2();

        let r_types = r
            .iter()
            .map(|f| f.token_type.clone())
            .collect::<Vec<TokenType>>();
        let e_types = e
            .iter()
            .map(|f| f.token_type.clone())
            .collect::<Vec<TokenType>>();
        for (idx, (e, r)) in e_types.iter().zip(r_types.iter()).enumerate() {
            print!("{}: {:#?} == {:?}\n", idx, e, r);
        }
    }

    // pub fn test_real_file() {
    //     //let input = std::fs::read_to_string("./test_code.mashup").unwrap();
    //     println!("Need to impliment");
    //     assert_eq!(1, 2);
    // }
}

mod expected_outputs {
    use lang::utils::tokens::Token;

    pub fn expected_token_test() -> Vec<Token> {
        todo!()
    }

    pub fn expected_token_test_2() -> Vec<Token> {
        todo!()
    }
}
