use std::collections::VecDeque;

pub mod lexer;
pub mod tests;
pub mod tokens;
pub mod utils;

fn main() {
    let input = &String::from(
        "
fn sub(x,y) {
    x - y
};",
    );

    let mut l = lexer::Lexer::new(input);

    println!("{:?}", &l.read_to_end_of_line());
}
