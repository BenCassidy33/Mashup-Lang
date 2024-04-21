#![allow(warnings)]

pub mod lexer;
pub mod tests;
pub mod tokens;
pub mod utils;

use lexer::Lexer;

fn main() {
    read_test2()
}

pub fn read_test2() {
    let mut input = std::fs::read_to_string("./src/tests/test_code.mashup").unwrap();
    let mut l = Lexer::new(&mut input);
    let r = l.read_and_tokenize_input();

    println!("{}", input);
    println!("Results: {:#?}", r);

    todo!("Add in more token parsing, update so that numbers can be identified as numebrs.");
}

fn print_type<T>(_: &T) -> &str {
    return std::any::type_name::<T>();
}
