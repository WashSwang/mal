mod core;
mod env;
mod printer;
mod reader;
mod types;

use crate::core::NameSpace;
use env::Env;
use printer::print_str;
use reader::read_str;
use types::MalType;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::{cell::RefCell, rc::Rc};

fn read(input: &str) -> Option<Rc<MalType>> {
    match read_str(input) {
        Ok((_, mal)) => Some(mal),
        _ => None,
    }
}

fn eval_ast(ast: Rc<MalType>, env: Rc<RefCell<Env>>) -> Option<Rc<MalType>> {
    match &*ast {
        MalType::Symbol(symbol) => {
            let mal = env.borrow().get(symbol);
            if mal.is_none() {
                println!("{} not found", symbol);
            }
            mal
        }
        MalType::List(list) => {
            let mut eval_list = vec![];
            for item in list.iter() {
                match eval(item.clone(), env.clone()) {
                    Some(mal) => eval_list.push(mal),
                    _ => return None,
                }
            }
            Some(Rc::new(MalType::List(eval_list)))
        }
        MalType::Vector(vec) => {
            let mut eval_vec = vec![];
            for item in vec.iter() {
                match eval(item.clone(), env.clone()) {
                    Some(mal) => eval_vec.push(mal),
                    _ => return None,
                }
            }
            Some(Rc::new(MalType::Vector(eval_vec)))
        }
        MalType::HashMap(kvs) => {
            let mut eval_map = vec![];
            for (k, v) in kvs.iter() {
                match eval(v.clone(), env.clone()) {
                    Some(mal) => eval_map.push((k.clone(), mal)),
                    _ => return None,
                }
            }
            Some(Rc::new(MalType::HashMap(eval_map)))
        }
        _ => Some(ast),
    }
}

fn eval_def(ast: Rc<MalType>, env: Rc<RefCell<Env>>) -> Option<Rc<MalType>> {
    if let MalType::List(list) = &*ast {
        if list.len() != 3 {
            println!("Wrong amount of arguments for def!");
            return None;
        }
        match &*list[1] {
            MalType::Symbol(bind) => {
                let value = eval(list[2].clone(), env.clone());
                if let Some(mal) = &value {
                    env.borrow_mut().set(bind, mal.clone())
                }
                value
            }
            _ => {
                println!("{} is not a symbol", print_str(list[1].clone(), true, true));
                None
            }
        }
    } else {
        panic!()
    }
}

fn eval_let(ast: Rc<MalType>, env: Rc<RefCell<Env>>) -> Option<Rc<MalType>> {
    if let MalType::List(list) = &*ast {
        if list.len() != 3 {
            println!("Wrong amount of arguments for let*");
            return None;
        }
        match &*list[1] {
            MalType::List(bind_list) | MalType::Vector(bind_list) => {
                if bind_list.len() % 2 != 0 {
                    println!("Wrong amount of arguments for bind of let*");
                }
                let new_env = Rc::new(RefCell::new(Env::new(env)));
                for i in 0..bind_list.len() / 2 {
                    match &*bind_list[i * 2] {
                        MalType::Symbol(bind) => {
                            let value = eval(bind_list[i * 2 + 1].clone(), new_env.clone());
                            if let Some(mal) = value {
                                new_env.borrow_mut().set(bind, mal);
                            } else {
                                return value;
                            }
                        }
                        _ => {
                            println!(
                                "{} is not a symbol",
                                print_str(bind_list[i * 2].clone(), true, true)
                            );
                            return None;
                        }
                    }
                }
                eval(list[2].clone(), new_env)
            }
            _ => {
                println!("Wrong bind format");
                None
            }
        }
    } else {
        panic!()
    }
}

fn eval_fn(ast: Rc<MalType>, env: Rc<RefCell<Env>>) -> Option<Rc<MalType>> {
    if let MalType::List(list) = &*ast {
        if list.len() != 3 {
            println!("Wrong amount of arguments for fn*");
            return None;
        }
        match &*list[1] {
            MalType::List(bind_list) | MalType::Vector(bind_list) => {
                let mut parameters = vec![];
                for bind in bind_list.iter() {
                    if let MalType::Symbol(symbol) = &**bind {
                        parameters.push(symbol.clone());
                    } else {
                        println!("{} is not a symbol", print_str((*bind).clone(), true, true));
                        return None;
                    }
                }
                let body = list[2].clone();
                Some(Rc::new(MalType::Func(Rc::new(move |args| {
                    let mut binds = vec![];
                    let mut exprs = vec![];
                    for i in 0..parameters.len() {
                        if parameters[i] == "&" {
                            if i + 1 < parameters.len() {
                                binds.push(parameters[i + 1].as_str());
                                let mut rest = vec![];
                                for arg in args.iter().skip(i) {
                                    rest.push(arg.clone());
                                }
                                exprs.push(Rc::new(MalType::List(rest)));
                            }
                            break;
                        } else {
                            binds.push(parameters[i].as_str());
                            exprs.push(args[i].clone());
                        }
                    }
                    let new_env = Rc::new(RefCell::new(Env::new_bind(env.clone(), &binds, &exprs)));
                    eval(body.clone(), new_env)
                }))))
            }
            _ => None,
        }
    } else {
        panic!()
    }
}

fn eval_if(ast: Rc<MalType>, env: Rc<RefCell<Env>>) -> Option<Rc<MalType>> {
    if let MalType::List(list) = &*ast {
        if list.len() <= 2 {
            println!("Wrong amount of arguments for if");
            return None;
        }
        let cond = eval(list[1].clone(), env.clone());
        if let Some(value) = cond {
            match &*value {
                MalType::Bool(false) | MalType::Nil => {
                    if list.len() >= 4 {
                        eval(list[3].clone(), env)
                    } else {
                        Some(Rc::new(MalType::Nil))
                    }
                }
                _ => {
                    if list.len() >= 3 {
                        eval(list[2].clone(), env)
                    } else {
                        Some(Rc::new(MalType::Nil))
                    }
                }
            }
        } else {
            None
        }
    } else {
        panic!()
    }
}

fn eval_do(ast: Rc<MalType>, env: Rc<RefCell<Env>>) -> Option<Rc<MalType>> {
    if let MalType::List(list) = &*ast {
        if list.len() <= 1 {
            println!("Wrong amount of arguments for do");
            return None;
        }
        let mut result = None;
        for mal in list[1..].iter() {
            result = eval(mal.clone(), env.clone());
        }
        result
    } else {
        panic!()
    }
}

fn eval(ast: Rc<MalType>, env: Rc<RefCell<Env>>) -> Option<Rc<MalType>> {
    match &*ast {
        MalType::List(list) => {
            if list.is_empty() {
                return Some(ast);
            }

            if let MalType::Symbol(symbol) = &*list[0] {
                if symbol == "def!" {
                    return eval_def(ast, env);
                }
                if symbol == "let*" {
                    return eval_let(ast, env);
                }
                if symbol == "fn*" {
                    return eval_fn(ast, env);
                }
                if symbol == "if" {
                    return eval_if(ast, env);
                }
                if symbol == "do" {
                    return eval_do(ast, env);
                }
            }

            let res = eval_ast(ast, env);
            match res {
                Some(mal) => match &*mal {
                    MalType::List(list) => match &*list[0] {
                        MalType::BuiltinFunc(func) => func(&list[1..]),
                        MalType::Func(func) => func(&list[1..]),
                        _ => Some(mal.clone()),
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

fn rep(input: &str, env: Rc<RefCell<Env>>) {
    match read(input) {
        Some(ast) => println!("{}", print(eval(ast, env))),
        _ => println!("EOF"),
    }
}

fn load_builtin(env: Rc<RefCell<Env>>) {
    for (name, func) in NameSpace::new().builtin {
        env.borrow_mut().set(name, Rc::new(func));
    }
    eval(read("(def! not (fn* (a) (if a false true)))").unwrap(), env);
}

fn main() {
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let env = Rc::new(RefCell::new(Env::new_root()));
    load_builtin(env.clone());
    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(input) => {
                rl.add_history_entry(input.as_str());
                rep(input.as_str(), env.clone());
            }
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
