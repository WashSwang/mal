use crate::printer::print_str;
use crate::types::MalType;
use std::rc::Rc;

pub struct NameSpace {
    pub builtin: Vec<(&'static str, MalType)>,
}

impl NameSpace {
    pub fn new() -> Self {
        let mut builtin = vec![];

        builtin.push((
            "+",
            MalType::BuiltinFunc(|args| {
                if args.len() != 2 {
                    println!("Wrong amount of arguments for +");
                    return None;
                }
                match (&*args[0], &*args[1]) {
                    (MalType::Int(a), MalType::Int(b)) => Some(Rc::new(MalType::Int(a + b))),
                    _ => None,
                }
            }),
        ));

        builtin.push((
            "-",
            MalType::BuiltinFunc(|args| {
                if args.len() != 2 {
                    println!("Wrong amount of arguments for -");
                    return None;
                }
                match (&*args[0], &*args[1]) {
                    (MalType::Int(a), MalType::Int(b)) => Some(Rc::new(MalType::Int(a - b))),
                    _ => None,
                }
            }),
        ));

        builtin.push((
            "*",
            MalType::BuiltinFunc(|args| {
                if args.len() != 2 {
                    println!("Wrong amount of arguments for *");
                    return None;
                }
                match (&*args[0], &*args[1]) {
                    (MalType::Int(a), MalType::Int(b)) => Some(Rc::new(MalType::Int(a * b))),
                    _ => None,
                }
            }),
        ));

        builtin.push((
            "/",
            MalType::BuiltinFunc(|args| {
                if args.len() != 2 {
                    println!("Wrong amount of arguments for /");
                    return None;
                }
                match (&*args[0], &*args[1]) {
                    (MalType::Int(a), MalType::Int(b)) => Some(Rc::new(MalType::Int(a / b))),
                    _ => None,
                }
            }),
        ));

        builtin.push((
            "prn",
            MalType::BuiltinFunc(|args| {
                let mut result = String::from("");
                for i in 0..args.len() {
                    result += &print_str(args[i].clone(), false, true);
                    if i != args.len() - 1 {
                        result.push(' ');
                    }
                }
                println!("{}", result);
                Some(Rc::new(MalType::Nil))
            }),
        ));

        builtin.push((
            "pr-str",
            MalType::BuiltinFunc(|args| {
                let mut result = String::from("");
                for i in 0..args.len() {
                    result += &print_str(args[i].clone(), false, true);
                    if i != args.len() - 1 {
                        result.push(' ');
                    }
                }
                Some(Rc::new(MalType::Str(result)))
            }),
        ));

        builtin.push((
            "str",
            MalType::BuiltinFunc(|args| {
                let mut result = String::from("");
                for arg in args.iter() {
                    result += &print_str(arg.clone(), false, false);
                }
                Some(Rc::new(MalType::Str(result)))
            }),
        ));

        builtin.push((
            "println",
            MalType::BuiltinFunc(|args| {
                let mut result = String::from("");
                for i in 0..args.len() {
                    result += &print_str(args[i].clone(), false, false);
                    if i != args.len() - 1 {
                        result.push(' ');
                    }
                }
                println!("{}", result);
                Some(Rc::new(MalType::Nil))
            }),
        ));

        builtin.push((
            "list",
            MalType::BuiltinFunc(|args| {
                let mut list = vec![];
                for item in args.iter() {
                    list.push(item.clone());
                }
                Some(Rc::new(MalType::List(list)))
            }),
        ));

        builtin.push((
            "list?",
            MalType::BuiltinFunc(|args| {
                if args.is_empty() {
                    Some(Rc::new(MalType::Nil))
                } else if let MalType::List(_) = &*args[0] {
                    Some(Rc::new(MalType::Bool(true)))
                } else {
                    Some(Rc::new(MalType::Bool(false)))
                }
            }),
        ));

        builtin.push((
            "empty?",
            MalType::BuiltinFunc(|args| {
                if args.is_empty() {
                    Some(Rc::new(MalType::Nil))
                } else if let MalType::List(list) | MalType::Vector(list) = &*args[0] {
                    Some(Rc::new(MalType::Bool(list.is_empty())))
                } else {
                    Some(Rc::new(MalType::Nil))
                }
            }),
        ));

        builtin.push((
            "count",
            MalType::BuiltinFunc(|args| {
                if args.is_empty() {
                    Some(Rc::new(MalType::Nil))
                } else if let MalType::List(list) | MalType::Vector(list) = &*args[0] {
                    Some(Rc::new(MalType::Int(list.len() as i32)))
                } else {
                    Some(Rc::new(MalType::Int(0)))
                }
            }),
        ));

        builtin.push((
            "=",
            MalType::BuiltinFunc(|args| {
                if args.len() != 2 {
                    println!("Wrong amount of arguments for =");
                    return None;
                }
                Some(Rc::new(MalType::Bool(args[0] == args[1])))
            }),
        ));

        builtin.push((
            "<",
            MalType::BuiltinFunc(|args| {
                if args.len() != 2 {
                    println!("Wrong amount of arguments for <");
                    return None;
                }
                match (&*args[0], &*args[1]) {
                    (MalType::Int(a), MalType::Int(b)) => Some(Rc::new(MalType::Bool(a < b))),
                    _ => None,
                }
            }),
        ));

        builtin.push((
            "<=",
            MalType::BuiltinFunc(|args| {
                if args.len() != 2 {
                    println!("Wrong amount of arguments for <=");
                    return None;
                }
                match (&*args[0], &*args[1]) {
                    (MalType::Int(a), MalType::Int(b)) => Some(Rc::new(MalType::Bool(a <= b))),
                    _ => None,
                }
            }),
        ));

        builtin.push((
            ">",
            MalType::BuiltinFunc(|args| {
                if args.len() != 2 {
                    println!("Wrong amount of arguments for >");
                    return None;
                }
                match (&*args[0], &*args[1]) {
                    (MalType::Int(a), MalType::Int(b)) => Some(Rc::new(MalType::Bool(a > b))),
                    _ => None,
                }
            }),
        ));

        builtin.push((
            ">=",
            MalType::BuiltinFunc(|args| {
                if args.len() != 2 {
                    println!("Wrong amount of arguments for >=");
                    return None;
                }
                match (&*args[0], &*args[1]) {
                    (MalType::Int(a), MalType::Int(b)) => Some(Rc::new(MalType::Bool(a >= b))),
                    _ => None,
                }
            }),
        ));

        Self { builtin }
    }
}
