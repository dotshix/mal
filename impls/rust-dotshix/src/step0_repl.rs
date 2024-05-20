use std::io::Write;
use text_io::try_read;

fn read(input: String) -> String {
    input
}

fn eval(input: String) -> String {
    input
}

fn print(input: String) -> String {
    input
}

fn rep(input: String) -> String {
    print(eval(read(input)))
}

fn main() {
    loop {
        print!("user> ");
        std::io::stdout().flush().unwrap(); // Ensure prompt is displayed

        let input: Result<String, _> = try_read!("{}\n");

        match input {
            Ok(line) => {
                let output = rep(line);
                println!("{}", output);
            }
            Err(_) => {
                break; // Exit loop on EOF or any read error
            }
        }
    }
}
