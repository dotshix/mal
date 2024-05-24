mod reader;

use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use pest::error::Error;

// Import the standard Result type alias
use std::result::Result as StdResult;

fn read(input: String) -> StdResult<Vec<reader::MalValue>, Error<reader::Rule>> {
    reader::parse_input(&input)
}

fn eval(input: Vec<reader::MalValue>) -> Vec<reader::MalValue> {
    // For now, eval just returns the input
    input
}


fn print(input: Vec<reader::MalValue>) -> String {
    fn print_node(node: reader::MalValue) -> String {
        match node {
            reader::MalValue::String(s) => s,
            reader::MalValue::Comment(c) => format!(";{}", c),
            reader::MalValue::NonSpecialSeq(s) => s,
            reader::MalValue::List(list) => {
                let elements: Vec<String> = list.into_iter().map(print_node).collect();
                format!("({})", elements.join(" "))
            }
            reader::MalValue::Other(s) => s.split_whitespace().collect::<Vec<_>>().join(" "), // Normalize spaces
        }
    }

    input.into_iter().map(print_node).collect::<Vec<String>>().join(" ")
}

fn rep(input: String) -> String {
    match read(input) {
        Ok(parsed) => {
            let evaluated = eval(parsed);
            print(evaluated)
        },
        Err(e) => format!("Error: {:?}", e),
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
