use lang::{parser::parser::Parser, Lexer, Token, TokenType};

fn main() {
    let tokens = Lexer::new(&mut String::from("let x: Result<Vector<usize>> = 5;")).lex();
    let mut parser = Parser::new(tokens);
    let res = parser.generate_variable();
    println!("{:?}", res);
}
