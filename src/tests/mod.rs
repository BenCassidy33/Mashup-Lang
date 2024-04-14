#[cfg(test)]
mod tests {
    use crate::lexer::*;

    #[test]
    pub fn test_char_read() {
        let input = &String::from("Let x = 3");

        let lexer = Lexer::new(input);
        let expected_character = 'L' as u8;

        assert_eq!(lexer.char_under_cusor(), expected_character)
    }

    #[test]
    pub fn test_read_to_whitespace() {
        let input = &String::from("let x = 3");
        let expected = String::from("let")
            .chars()
            .into_iter()
            .collect::<Vec<char>>()
            .into_iter()
            .map(|f| f as u8)
            .collect::<Vec<u8>>();

        let mut l = Lexer::new(input);

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
        let input = &String::from("let;");
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

        let mut l = Lexer::new(input);

        assert_eq!(l.read_to_whitespace_or_special(), expected)
    }

    #[test]
    pub fn read_to_eol() {
        let i = &String::from("let add = fn(x,y) { return x + y };");
        let e = vec![
            "let", "add", "=", "fn", "(", "x", ",", "y", ")", "{", "return", "x", "+", "y", "}",
            ";",
        ];

        let mut l = Lexer::new(i);
        assert_eq!(l.read_to_end_of_line(), e);
    }
}
