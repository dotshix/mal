mod reader;

use pest::error::Error;
use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

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
    fn print_node(node: &reader::MalValue) {
        match node {
            reader::MalValue::String(s) => print!("\"{}\"", s),
            reader::MalValue::Comment(c) => print!("{}", c),
            reader::MalValue::NonSpecialSeq(s) => print!("{}", s),
            reader::MalValue::List(list) => {
                print!("(");
                for (i, item) in list.iter().enumerate() {
                    if i > 0 {
                        print!(" ");
                    }
                    print_node(item);
                }
                print!(")");
            },
            reader::MalValue::Mal(content) => {
                for (i, item) in content.iter().enumerate() {
                    if i > 0 {
                        print!(" ");
                    }
                    print_node(item);
                }
            },
            reader::MalValue::Other(_) => {}, // Do nothing for other types
            reader::MalValue::EOI => {}, // Do nothing for EOI
        }
    }

    for node in input.iter() {
        print_node(node);
        print!(" "); // Add space after each top-level element
    }

    // Return an empty string as specified
    String::new()
}



// fn print(input: Vec<reader::MalValue>) -> String {
//     fn print_node(node: reader::MalValue) -> String {
//     match node {
//         reader::MalValue::String(s) => s,
//         reader::MalValue::Comment(c) => format!(";{}", c),
//         reader::MalValue::NonSpecialSeq(s) => s,
//         reader::MalValue::List(list) => {
//             let list_str: Vec<String> = list.into_iter().map(|v| print_node(v)).collect();
//             format!("({})", list_str.join(" "))
//         },
//         reader::MalValue::Other(s) => s.split_whitespace().collect::<Vec<_>>().join(" "), // Normalize spaces
//         reader::MalValue::EOI => "".to_string(), // Handle EOI variant
//     }
//     }

//     input
//         .into_iter()
//         .map(print_node)
//         .collect::<Vec<String>>()
//         .join(" ")
// }

fn rep(input: String) -> String {
    match read(input) {
        Ok(parsed) => {
            let evaluated = eval(parsed);
            print(evaluated)
        }
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
