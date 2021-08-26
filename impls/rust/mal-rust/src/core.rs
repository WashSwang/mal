use crate::printer::print_str;
use crate::reader::read_str;
use crate::types::MalType;
use std::cell::RefCell;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::rc::Rc;

pub struct NameSpace {
    pub builtin: Vec<(&'static str, MalType)>,
}

impl NameSpace {
    pub fn new() -> Self {
        let mut builtin = vec![];

        builtin.push((
            "+",
            MalType::BuiltinFunc(Rc::new(|args| {
                if args.len() != 2 {
                    println!("Wrong amount of arguments for +");
                    return None;
                }
                match (&*args[0], &*args[1]) {
                    (MalType::Int(a), MalType::Int(b)) => Some(Rc::new(MalType::Int(a + b))),
                    _ => None,
                }
            })),
        ));

        builtin.push((
            "-",
            MalType::BuiltinFunc(Rc::new(|args| {
                if args.len() != 2 {
                    println!("Wrong amount of arguments for -");
                    return None;
                }
                match (&*args[0], &*args[1]) {
                    (MalType::Int(a), MalType::Int(b)) => Some(Rc::new(MalType::Int(a - b))),
                    _ => None,
                }
            })),
        ));

        builtin.push((
            "*",
            MalType::BuiltinFunc(Rc::new(|args| {
                if args.len() != 2 {
                    println!("Wrong amount of arguments for *");
                    return None;
                }
                match (&*args[0], &*args[1]) {
                    (MalType::Int(a), MalType::Int(b)) => Some(Rc::new(MalType::Int(a * b))),
                    _ => None,
                }
            })),
        ));

        builtin.push((
            "/",
            MalType::BuiltinFunc(Rc::new(|args| {
                if args.len() != 2 {
                    println!("Wrong amount of arguments for /");
                    return None;
                }
                match (&*args[0], &*args[1]) {
                    (MalType::Int(a), MalType::Int(b)) => Some(Rc::new(MalType::Int(a / b))),
                    _ => None,
                }
            })),
        ));

        builtin.push((
            "prn",
            MalType::BuiltinFunc(Rc::new(|args| {
                let mut result = String::from("");
                for i in 0..args.len() {
                    result += &print_str(args[i].clone(), false, true);
                    if i != args.len() - 1 {
                        result.push(' ');
                    }
                }
                println!("{}", result);
                Some(Rc::new(MalType::Nil))
            })),
        ));

        builtin.push((
            "pr-str",
            MalType::BuiltinFunc(Rc::new(|args| {
                let mut result = String::from("");
                for i in 0..args.len() {
                    result += &print_str(args[i].clone(), false, true);
                    if i != args.len() - 1 {
                        result.push(' ');
                    }
                }
                Some(Rc::new(MalType::Str(result)))
            })),
        ));

        builtin.push((
            "str",
            MalType::BuiltinFunc(Rc::new(|args| {
                let mut result = String::from("");
                for arg in args.iter() {
                    result += &print_str(arg.clone(), false, false);
                }
                Some(Rc::new(MalType::Str(result)))
            })),
        ));

        builtin.push((
            "println",
            MalType::BuiltinFunc(Rc::new(|args| {
                let mut result = String::from("");
                for i in 0..args.len() {
                    result += &print_str(args[i].clone(), false, false);
                    if i != args.len() - 1 {
                        result.push(' ');
                    }
                }
                println!("{}", result);
                Some(Rc::new(MalType::Nil))
            })),
        ));

        builtin.push((
            "list",
            MalType::BuiltinFunc(Rc::new(|args| {
                let mut list = vec![];
                for item in args.iter() {
                    list.push(item.clone());
                }
                Some(Rc::new(MalType::List(list)))
            })),
        ));

        builtin.push((
            "list?",
            MalType::BuiltinFunc(Rc::new(|args| {
                if args.is_empty() {
                    Some(Rc::new(MalType::Nil))
                } else if let MalType::List(_) = &*args[0] {
                    Some(Rc::new(MalType::Bool(true)))
                } else {
                    Some(Rc::new(MalType::Bool(false)))
                }
            })),
        ));

        builtin.push((
            "empty?",
            MalType::BuiltinFunc(Rc::new(|args| {
                if args.is_empty() {
                    Some(Rc::new(MalType::Nil))
                } else if let MalType::List(list) | MalType::Vector(list) = &*args[0] {
                    Some(Rc::new(MalType::Bool(list.is_empty())))
                } else {
                    Some(Rc::new(MalType::Nil))
                }
            })),
        ));

        builtin.push((
            "count",
            MalType::BuiltinFunc(Rc::new(|args| {
                if args.is_empty() {
                    Some(Rc::new(MalType::Nil))
                } else if let MalType::List(list) | MalType::Vector(list) = &*args[0] {
                    Some(Rc::new(MalType::Int(list.len() as i32)))
                } else {
                    Some(Rc::new(MalType::Int(0)))
                }
            })),
        ));

        builtin.push((
            "=",
            MalType::BuiltinFunc(Rc::new(|args| {
                if args.len() != 2 {
                    println!("Wrong amount of arguments for =");
                    return None;
                }
                Some(Rc::new(MalType::Bool(args[0] == args[1])))
            })),
        ));

        builtin.push((
            "<",
            MalType::BuiltinFunc(Rc::new(|args| {
                if args.len() != 2 {
                    println!("Wrong amount of arguments for <");
                    return None;
                }
                match (&*args[0], &*args[1]) {
                    (MalType::Int(a), MalType::Int(b)) => Some(Rc::new(MalType::Bool(a < b))),
                    _ => None,
                }
            })),
        ));

        builtin.push((
            "<=",
            MalType::BuiltinFunc(Rc::new(|args| {
                if args.len() != 2 {
                    println!("Wrong amount of arguments for <=");
                    return None;
                }
                match (&*args[0], &*args[1]) {
                    (MalType::Int(a), MalType::Int(b)) => Some(Rc::new(MalType::Bool(a <= b))),
                    _ => None,
                }
            })),
        ));

        builtin.push((
            ">",
            MalType::BuiltinFunc(Rc::new(|args| {
                if args.len() != 2 {
                    println!("Wrong amount of arguments for >");
                    return None;
                }
                match (&*args[0], &*args[1]) {
                    (MalType::Int(a), MalType::Int(b)) => Some(Rc::new(MalType::Bool(a > b))),
                    _ => None,
                }
            })),
        ));

        builtin.push((
            ">=",
            MalType::BuiltinFunc(Rc::new(|args| {
                if args.len() != 2 {
                    println!("Wrong amount of arguments for >=");
                    return None;
                }
                match (&*args[0], &*args[1]) {
                    (MalType::Int(a), MalType::Int(b)) => Some(Rc::new(MalType::Bool(a >= b))),
                    _ => None,
                }
            })),
        ));

        builtin.push((
            "read-string",
            MalType::BuiltinFunc(Rc::new(|args| {
                if !args.is_empty() {
                    if let MalType::Str(s) = &*args[0] {
                        if let Ok((_, mal)) = read_str(s) {
                            return Some(mal);
                        }
                    }
                }
                Some(Rc::new(MalType::Nil))
            })),
        ));

        builtin.push((
            "slurp",
            MalType::BuiltinFunc(Rc::new(|args| {
                if !args.is_empty() {
                    if let MalType::Str(s) = &*args[0] {
                        let path = Path::new(&s);
                        if let Ok(mut file) = File::open(&path) {
                            let mut content = String::new();
                            if file.read_to_string(&mut content).is_ok() {
                                return Some(Rc::new(MalType::Str(content)));
                            }
                        }
                    }
                }
                Some(Rc::new(MalType::Nil))
            })),
        ));

        builtin.push((
            "atom",
            MalType::BuiltinFunc(Rc::new(|args| {
                if args.is_empty() {
                    Some(Rc::new(MalType::Nil))
                } else {
                    Some(Rc::new(MalType::Atom(RefCell::new(args[0].clone()))))
                }
            })),
        ));

        builtin.push((
            "atom?",
            MalType::BuiltinFunc(Rc::new(|args| {
                if args.is_empty() {
                    Some(Rc::new(MalType::Nil))
                } else if let MalType::Atom(_) = &*args[0] {
                    Some(Rc::new(MalType::Bool(true)))
                } else {
                    Some(Rc::new(MalType::Bool(false)))
                }
            })),
        ));

        builtin.push((
            "deref",
            MalType::BuiltinFunc(Rc::new(|args| {
                if args.is_empty() {
                    Some(Rc::new(MalType::Nil))
                } else if let MalType::Atom(value) = &*args[0] {
                    Some(value.borrow().clone())
                } else {
                    Some(Rc::new(MalType::Nil))
                }
            })),
        ));

        builtin.push((
            "reset!",
            MalType::BuiltinFunc(Rc::new(|args| {
                if args.len() < 2 {
                    Some(Rc::new(MalType::Nil))
                } else {
                    if let MalType::Atom(value) = &*args[0] {
                        value.replace(args[1].clone());
                        Some(args[1].clone())
                    } else {
                        Some(Rc::new(MalType::Nil))
                    }
                }
            })),
        ));

        builtin.push((
            "swap!",
            MalType::BuiltinFunc(Rc::new(|args| {
                if args.len() < 2 {
                    Some(Rc::new(MalType::Nil))
                } else {
                    if let MalType::Atom(value) = &*args[0] {
                        let mut list = vec![value.borrow().clone()];
                        for parameter in args.iter().skip(2) {
                            list.push(parameter.clone());
                        }
                        let result;
                        match &*args[1] {
                            MalType::BuiltinFunc(func) => result = func(&list)?,
                            MalType::Func(closure) => result = (closure.func)(&list)?,
                            _ => {
                                return Some(Rc::new(MalType::Nil));
                            }
                        }
                        value.replace(result.clone());
                        Some(result)
                    } else {
                        Some(Rc::new(MalType::Nil))
                    }
                }
            })),
        ));

        builtin.push((
            "cons",
            MalType::BuiltinFunc(Rc::new(|args| {
                if args.len() < 2 {
                    Some(Rc::new(MalType::Nil))
                } else if let MalType::List(list) | MalType::Vector(list) = &*args[1] {
                    let mut result = list.clone();
                    result.insert(0, args[0].clone());
                    Some(Rc::new(MalType::List(result)))
                } else {
                    Some(Rc::new(MalType::Int(0)))
                }
            })),
        ));

        builtin.push((
            "concat",
            MalType::BuiltinFunc(Rc::new(|args| {
                let mut result = vec![];
                for arg in args {
                    if let MalType::List(list) | MalType::Vector(list) = &**arg {
                        for item in list {
                            result.push(item.clone());
                        }
                    }
                }
                Some(Rc::new(MalType::List(result)))
            })),
        ));

        builtin.push((
            "vec",
            MalType::BuiltinFunc(Rc::new(|args| {
                if args.is_empty() {
                    Some(Rc::new(MalType::Nil))
                } else if let MalType::List(list) | MalType::Vector(list) = &*args[0] {
                    Some(Rc::new(MalType::Vector(list.clone())))
                } else {
                    Some(Rc::new(MalType::Nil))
                }
            })),
        ));
        Self { builtin }
    }
}
