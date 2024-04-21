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
    let mut i = String::from(
        "
            fun fib = rec (num: int) : int; do
                nums_list :: append match num with n 
                | 0 -> 0;
                | 1 -> 1;
                | _ -> fib(n - 1) + fib(n - 2);
            end;",
    );

    let e = vec!["if", "i", "==", "3", "->", "yeild", "fruit", ";"];

    let mut l = Lexer::new(&mut i);
    let r = l.read_to_eof();
    println!("{:?}", r);
}

fn print_type<T>(_: &T) -> &str {
    return std::any::type_name::<T>();
}
