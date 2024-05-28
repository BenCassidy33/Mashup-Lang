#![allow(warnings)]

pub mod ast;
pub mod lexer;
pub mod tests;
pub mod tokens;
pub mod utils;

use lexer::Lexer;
use std::{panic, path::Path, process::exit};

use crate::utils::Colors;

struct ArgOption {
    short: &'static str,
    long: &'static str,
    callback: fn(),
    conflicts: Option<Vec<String>>,
}

enum ArgConfliction {
    Confliction(Vec<String>),
    NoConfliction,
}

fn transpile() {
    todo!();
}

fn check_argument_conflicts(arg: &String, options: &Vec<ArgOption>) -> ArgConfliction {
    todo!();
}

const HELP: &'static str = "Use '-h' or '--help' for help.";

fn main() {
    panic::set_hook(Box::new(|e| {
        println!("{}\n", e);
    }));

    let options = vec![
        ArgOption {
            short: "-t",
            long: "--transpile",
            callback: transpile,
            conflicts: Some(vec![String::from("-h"), String::from("--help")]),
        },
        ArgOption {
            short: "-h",
            long: "--help",
            callback: || {
                println!("Usage: ./main [options]");
                println!("Options:");
            },
            conflicts: None,
        },
    ];

    let args: Vec<String> = std::env::args().collect::<Vec<String>>()[1..].to_vec();
    println!("{:?}", args);

    if args.contains(&String::from("-h"))
        || args.contains(&String::from("--help"))
        || args.len() == 0
    {
        (options[1].callback)();
        exit(0);
    }

    let file = || -> &Path {
        for arg in args.iter() {
            if Path::new(arg).exists() && Path::new(arg).is_file() {
                if !arg.ends_with("mash") {
                    eprintln!(
                        "\n{}.\nExpected file extension: '.mash'\n",
                        "Compliation Error: Invalid File Extension".red()
                    );
                    exit(1);
                }
                return Path::new(arg);
            }
        }

        eprintln!(
            "\n{} {}.",
            "Compliation Error: No file specified.\n".red(),
            HELP,
        );
        exit(1);
    };

    'outer: for arg in &args {
        if !arg.starts_with("-") || !arg.starts_with("--") {
            continue;
        }

        for option in options.iter() {
            if arg == option.short || arg == option.long {
                if let ArgConfliction::Confliction(arg) = check_argument_conflicts(arg, &options) {
                    eprintln!(
                        "{}. {}\nConflictions: {}",
                        "Compliation Error: Argument Confliction".red(),
                        HELP,
                        arg.join(", ")
                    );
                    exit(1);
                }

                (option.callback)();
                continue 'outer;
            }
        }

        eprintln!(
            "\n{}: {}\n{}.",
            "Compliation Error: Invalid Arugment".red(),
            arg,
            HELP,
        );
        exit(1);
    }

    let mut file = std::fs::read_to_string(file());

    if file.is_err() {
        eprintln!(
            "\n{}: Could not read file: {}",
            "Compliation Error".red(),
            file.err().unwrap()
        );
        exit(1);
    }

    let mut lexer = Lexer::new(&mut file.unwrap()).lex();
    println!("{:?}", lexer);
    todo!("Implement newline tokens.");
}
