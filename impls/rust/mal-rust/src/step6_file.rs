mod core;
mod env;
mod printer;
mod reader;
mod types;

use crate::core::NameSpace;
use env::Env;
use printer::print_str;
use reader::read_str;
use types::{ClosureType, MalType};

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

fn eval_let(ast: Rc<MalType>, env: Rc<RefCell<Env>>) -> Option<(Rc<MalType>, Rc<RefCell<Env>>)> {
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
                                return None;
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
                Some((list[2].clone(), new_env))
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
                Some(Rc::new(MalType::Func(ClosureType {
                    ast: list[2].clone(),
                    params: parameters.clone(),
                    env: env.clone(),
                    func: Rc::new(move |args| {
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
                                if i > args.len() - 1 {
                                    break;
                                }
                                binds.push(parameters[i].as_str());
                                exprs.push(args[i].clone());
                            }
                        }
                        let new_env =
                            Rc::new(RefCell::new(Env::new_bind(env.clone(), &binds, &exprs)));
                        eval(body.clone(), new_env)
                    }),
                })))
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
        let cond = eval(list[1].clone(), env);
        if let Some(value) = cond {
            match &*value {
                MalType::Bool(false) | MalType::Nil => {
                    if list.len() >= 4 {
                        Some(list[3].clone())
                    } else {
                        Some(Rc::new(MalType::Nil))
                    }
                }
                _ => {
                    if list.len() >= 3 {
                        Some(list[2].clone())
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
        let mut parameters = vec![];
        for parameter in list.iter().take(list.len() - 1).skip(1) {
            parameters.push(parameter.clone());
        }
        eval_ast(Rc::new(MalType::List(parameters)), env);
        Some(list.last().unwrap().clone())
    } else {
        panic!()
    }
}

fn eval(ast: Rc<MalType>, env: Rc<RefCell<Env>>) -> Option<Rc<MalType>> {
    let mut ast = ast;
    let mut env = env;
    loop {
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
                        // Tail Call Optimization
                        let (new_ast, new_env) = eval_let(ast, env)?;
                        ast = new_ast;
                        env = new_env;
                        continue;
                    }
                    if symbol == "fn*" {
                        return eval_fn(ast, env);
                    }
                    if symbol == "if" {
                        ast = eval_if(ast, env.clone())?;
                        continue;
                    }
                    if symbol == "do" {
                        // Tail Call Optimization
                        ast = eval_do(ast, env.clone())?;
                        continue;
                    }
                }

                let res = eval_ast(ast, env);
                return match res {
                    Some(mal) => match &*mal {
                        MalType::List(list) => match &*list[0] {
                            MalType::BuiltinFunc(func) => func(&list[1..]),
                            MalType::Func(closure) => {
                                let mut binds = vec![];
                                let mut exprs = vec![];
                                for i in 0..closure.params.len() {
                                    if closure.params[i] == "&" {
                                        if i + 1 < closure.params.len() {
                                            binds.push(closure.params[i + 1].as_str());
                                            let mut rest = vec![];
                                            for arg in list.iter().skip(i + 1) {
                                                rest.push(arg.clone());
                                            }
                                            exprs.push(Rc::new(MalType::List(rest)));
                                        }
                                        break;
                                    } else {
                                        if i + 1 > list.len() - 1 {
                                            break;
                                        }
                                        binds.push(closure.params[i].as_str());
                                        exprs.push(list[i + 1].clone());
                                    }
                                }
                                let new_env = Rc::new(RefCell::new(Env::new_bind(
                                    closure.env.clone(),
                                    &binds,
                                    &exprs,
                                )));
                                env = new_env;
                                ast = closure.ast.clone();
                                continue;
                            }
                            _ => Some(mal.clone()),
                        },
                        _ => panic!(),
                    },
                    _ => None,
                };
            }
            _ => return eval_ast(ast, env),
        }
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

fn load_builtin(repl_env: Rc<RefCell<Env>>) {
    for (name, func) in NameSpace::new().builtin {
        repl_env.borrow_mut().set(name, Rc::new(func));
    }
    eval(
        read("(def! not (fn* (a) (if a false true)))").unwrap(),
        repl_env.clone(),
    );
    eval(
        read("(def! load-file (fn* (f) (eval (read-string (str \"(do \" (slurp f) \"\nnil)\")))))")
            .unwrap(),
        repl_env.clone(),
    );

    // Looks silly, need to beutify
    let clone_env = repl_env.clone();
    repl_env.borrow_mut().set(
        "eval",
        Rc::new(MalType::BuiltinFunc(Rc::new(move |args| {
            if args.is_empty() {
                Some(Rc::new(MalType::Nil))
            } else {
                eval(args[0].clone(), clone_env.clone())
            }
        }))),
    );
}

fn main() {
    let mut rl = Editor::<()>::new();

    let env = Rc::new(RefCell::new(Env::new_root()));
    load_builtin(env.clone());

    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {
        let filename = &args[1];
        let mut argv = vec![];
        for arg in args.iter().skip(2) {
            argv.push(Rc::new(MalType::Str(arg.clone())));
        }
        env.borrow_mut().set("*ARGV*", Rc::new(MalType::List(argv)));
        let command = format!("(load-file \"{}\")", filename);
        if let Some(ast) = read(command.as_str()) {
            eval(ast, env);
        }
    } else {
        env.borrow_mut()
            .set("*ARGV*", Rc::new(MalType::List(vec![])));
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
}
