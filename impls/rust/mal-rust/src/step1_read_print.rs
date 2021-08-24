mod printer;
mod reader;
mod types;

use printer::print_str;
use reader::read_str;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use types::MalType;

fn read(input: &str) -> Option<MalType> {
    match read_str(input) {
        Ok((_, mal)) => Some(mal),
        _ => None,
    }
}

fn eval(input: MalType) -> MalType {
    input
}

fn print(input: MalType) -> String {
    print_str(&input, false)
}

fn rep(input: &str) {
    match read(input) {
        Some(ast) => println!("{}", print(eval(ast))),
        _ => println!("EOF"),
    }
}

fn main() {
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(input) => {
                rl.add_history_entry(input.as_str());
                rep(input.as_str());
            }
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
