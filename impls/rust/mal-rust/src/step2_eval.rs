mod core;
mod env;
mod printer;
mod reader;
mod types;

use env::Env;
use printer::print_str;
use reader::read_str;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::rc::Rc;
use types::MalType;

fn read(input: &str) -> Option<Rc<MalType>> {
    match read_str(input) {
        Ok((_, mal)) => Some(mal),
        _ => None,
    }
}

fn eval_ast(ast: Rc<MalType>, env: &mut Env) -> Option<Rc<MalType>> {
    match &*ast {
        MalType::Symbol(symbol) => {
            let mal = env.get(symbol);
            if mal.is_none() {
                println!("Cannot resolve symbol \"{}\" in the current scope", symbol);
            }
            mal
        }
        MalType::List(list) => {
            let mut eval_list = vec![];
            for item in list.iter() {
                match eval(item.clone(), env) {
                    Some(mal) => eval_list.push(mal),
                    _ => return None,
                }
            }
            Some(Rc::new(MalType::List(eval_list)))
        }
        MalType::Vector(vec) => {
            let mut eval_vec = vec![];
            for item in vec.iter() {
                match eval(item.clone(), env) {
                    Some(mal) => eval_vec.push(mal),
                    _ => return None,
                }
            }
            Some(Rc::new(MalType::Vector(eval_vec)))
        }
        MalType::HashMap(kvs) => {
            let mut eval_map = vec![];
            for (k, v) in kvs.iter() {
                match eval(v.clone(), env) {
                    Some(mal) => eval_map.push((k.clone(), mal)),
                    _ => return None,
                }
            }
            Some(Rc::new(MalType::HashMap(eval_map)))
        }
        _ => Some(ast),
    }
}

fn eval(ast: Rc<MalType>, env: &mut Env) -> Option<Rc<MalType>> {
    match &*ast {
        MalType::List(list) => {
            if list.is_empty() {
                return Some(ast);
            }
            let res = eval_ast(ast, env);
            match res {
                Some(mal) => match &*mal {
                    MalType::List(list) => match &*list[0] {
                        MalType::BuiltinFunc(func) => func(&list[1..]),
                        _ => None,
                    },
                    _ => panic!(),
                },
                _ => None,
            }
        }
        _ => eval_ast(ast, env),
    }
}

fn print(input: Option<Rc<MalType>>) -> String {
    match input {
        Some(mal) => print_str(mal, false, true),
        _ => String::from("Error"),
    }
}

fn rep(input: &str, env: &mut Env) {
    match read(input) {
        Some(ast) => println!("{}", print(eval(ast, env))),
        _ => println!("EOF"),
    }
}

fn main() {
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let mut env = Env::new_root();
    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(input) => {
                rl.add_history_entry(input.as_str());
                rep(input.as_str(), &mut env);
            }
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
