#![cfg(test)]
mod paser_tests {
    use lang::{
        parser::{
            constructs::{BinaryCondition, ConditionType, IfStatement, Statement},
            parser::Parser,
            variable::{Variable, VariableTypeId, VariableTypeLiteral},
        },
        Lexer, Token, TokenType,
    };
    use pretty_assertions::assert_eq;

    #[test]
    pub fn test_block_scoping() {
        let tokens = Lexer::new(&mut String::from(
            "function test { if true { } }; let other = 5;",
        ))
        .lex();

        let mut parser = Parser::new(tokens);
        let scope = parser.get_block_scope(TokenType::LBRACE, TokenType::RBRACE);

        let expected: &[Token] = &[
            Token {
                token_type: TokenType::LBRACE,
                start: 14,
                end: 15,
                literal: "{".to_string(),
            },
            Token {
                token_type: TokenType::IF,
                start: 16,
                end: 18,
                literal: "if".to_string(),
            },
            Token {
                token_type: TokenType::TRUE,
                start: 19,
                end: 23,
                literal: "true".to_string(),
            },
            Token {
                token_type: TokenType::LBRACE,
                start: 24,
                end: 25,
                literal: "{".to_string(),
            },
            Token {
                token_type: TokenType::RBRACE,
                start: 26,
                end: 27,
                literal: "}".to_string(),
            },
            Token {
                token_type: TokenType::RBRACE,
                start: 28,
                end: 29,
                literal: "}".to_string(),
            },
        ];

        let got = scope.unwrap();

        assert_eq!(expected, got);
        //assert_eq!(true, false);
    }

    #[test]
    pub fn read_to_semicolon() {
        let tokens = Lexer::new(&mut String::from("let x = 5;")).lex();
        let mut parser = Parser::new(tokens);
        let expected: &[Token] = &[
            Token {
                token_type: TokenType::LET,
                start: 0,
                end: 3,
                literal: "let".to_string(),
            },
            Token {
                token_type: TokenType::IDENT("x".to_string()),
                start: 4,
                end: 5,
                literal: "x".to_string(),
            },
            Token {
                token_type: TokenType::ASSIGN,
                start: 6,
                end: 7,
                literal: "=".to_string(),
            },
            Token {
                token_type: TokenType::NUMBER(5.to_string()),
                start: 8,
                end: 9,
                literal: "5".to_string(),
            },
            Token {
                token_type: TokenType::SEMICOLON,
                start: 10,
                end: 11,
                literal: ";".to_string(),
            },
        ];

        let got = parser.read_to_semicolon();
        assert_eq!(expected, got);
    }

    #[test]
    pub fn generate_variable() {
        let tokens = Lexer::new(&mut String::from("let x: Result<usize> = 5;")).lex();
        let mut parser = Parser::new(tokens);
        let expected: Variable = Variable {
            name: "x".to_string(),
            type_id: VariableTypeId::Result(Box::new(VariableTypeId::Usize)),
            value: Some(VariableTypeLiteral::Usize(5)),
            mutable: false,
        };

        //let got = parser.generate_variable().unwrap();
        assert_eq!(expected, Variable::default());
    }

    #[test]
    pub fn generate_result_type() {
        let tokens = Lexer::new(&mut String::from("Result<usize>")).lex();
        let mut parser = Parser::new(tokens);
        let expected = VariableTypeId::Result(Box::new(VariableTypeId::Usize));
    }

    #[test]
    pub fn generate_statement() {
        let tokens = Lexer::new(&mut String::from(
            r#"
            let x = 5;
            if (x == 3); then
                    yeild x;
                    end
                "#,
        ))
        .lex();

        let mut parser = Parser::new(tokens);
        let left = &Variable {
            name: String::from("x"),
            type_id: VariableTypeId::Int,
            value: Some(VariableTypeLiteral::Int(5)),
            mutable: false,
        };

        let expected = Statement::If(IfStatement {
            condition: BinaryCondition::Equal,
            left: ConditionType::Variable(left),
            right: ConditionType::Static(VariableTypeLiteral::Int(0)),
        });

        let r = parser.generate_statement().unwrap();
        assert_eq!(r, expected);
    }
}
