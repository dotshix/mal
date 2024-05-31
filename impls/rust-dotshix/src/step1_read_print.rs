mod reader;

use pest::error::Error;
use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use std::result::Result as StdResult;
use reader::mal_parser::{MalValue, Rule, parse_input, format_pest_error};

fn read(input: String) -> StdResult<Vec<MalValue>, Error<Rule>> {
    parse_input(&input)
}

fn eval(input: Vec<MalValue>) -> Vec<MalValue> {
    // For now, eval just returns the input
    input
}

fn print_list(list: &Vec<MalValue>, open_delim: &str, close_delim: &str) {
    print!("{}", open_delim);
    let mut firsttime = true;
    for v in list {
        if !firsttime {
            print!(" ");
        }
        print_node(v);
        firsttime = false;
    }
    print!("{}", close_delim);
}

fn print_node(node: &MalValue) {
    match node {
        MalValue::String(s) => print!("{}", s),
        MalValue::Symbol(s) => print!("{}", s),
        MalValue::Number(n) => print!("{}", n),
        MalValue::Bool(b) => print!("{}", b),
        MalValue::Nil => print!("nil"),
        MalValue::Round(r) => {
            print_list(r, "(", ")");
        }
        MalValue::Square(r) => {
            print_list(r, "[", "]");
        }
        MalValue::Curly(r) => {
            print_list(r, "{", "}");
        }

        MalValue::Comment(c) => print!("{}", c),
        MalValue::NonSpecialSeq(s) => print!("{}", s),
        MalValue::Mal(content) => {
            for (i, item) in content.iter().enumerate() {
                if i > 0 {
                    print!(" ");
                }
                print_node(item);
            }
        }
        MalValue::EOI => {} // Do nothing for EOI
    }
}

fn print(input: Vec<MalValue>) -> String {
    for node in input.iter() {
        print_node(node);
        print!(" "); // Add space after each top-level element
    }

    // Return empty string for now
    String::new()
}

fn rep(input: String) -> String {
    match read(input) {
        Ok(parsed) => {
            let evaluated = eval(parsed);
            print(evaluated)
        }
        Err(e) => format!("Error: {:?}", format_pest_error(e)),
    }
}

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    rl.set_auto_add_history(true);

    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                let result = rep(line);
                println!("{}", result);
            }

            Err(ReadlineError::Interrupted) => {
                break;
            }

            Err(ReadlineError::Eof) => {
                break;
            }

            Err(err) => {
                eprintln!("Error {}", err);
                break;
            }
        }
    }

    Ok(())
}
