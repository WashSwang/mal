use crate::types::MalType;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Env<'a> {
    pub map: HashMap<String, Rc<MalType>>,
    pub outer: Option<&'a Env<'a>>,
}

impl<'a> Env<'a> {
    pub fn new_root() -> Self {
        let mut env = Env {
            map: HashMap::new(),
            outer: None,
        };
        env.load_builtin();
        env
    }

    pub fn new(outer: &'a Env<'a>) -> Self {
        Self {
            map: HashMap::new(),
            outer: Some(outer),
        }
    }

    pub fn get(&self, symbol: &str) -> Option<Rc<MalType>> {
        let result = self.map.get(symbol).cloned();
        if result.is_none() {
            self.outer?.get(symbol)
        } else {
            result
        }
    }

    pub fn set(&mut self, symbol: &str, mal: Rc<MalType>) {
        self.map.insert(String::from(symbol), mal);
    }

    pub fn load_builtin(&mut self) {
        self.load_add();
        self.load_sub();
        self.load_mul();
        self.load_div();
    }

    fn load_add(&mut self) {
        self.map.insert(
            String::from("+"),
            Rc::new(MalType::BuiltinFunc(|args| {
                if args.len() != 2 {
                    return None;
                }
                match (&*args[0], &*args[1]) {
                    (MalType::Int(a), MalType::Int(b)) => Some(Rc::new(MalType::Int(a + b))),
                    _ => None,
                }
            })),
        );
    }

    fn load_sub(&mut self) {
        self.map.insert(
            String::from("-"),
            Rc::new(MalType::BuiltinFunc(|args| {
                if args.len() != 2 {
                    return None;
                }
                match (&*args[0], &*args[1]) {
                    (MalType::Int(a), MalType::Int(b)) => Some(Rc::new(MalType::Int(a - b))),
                    _ => None,
                }
            })),
        );
    }

    fn load_mul(&mut self) {
        self.map.insert(
            String::from("*"),
            Rc::new(MalType::BuiltinFunc(|args| {
                if args.len() != 2 {
                    return None;
                }
                match (&*args[0], &*args[1]) {
                    (MalType::Int(a), MalType::Int(b)) => Some(Rc::new(MalType::Int(a * b))),
                    _ => None,
                }
            })),
        );
    }

    fn load_div(&mut self) {
        self.map.insert(
            String::from("/"),
            Rc::new(MalType::BuiltinFunc(|args| {
                if args.len() != 2 {
                    return None;
                }
                match (&*args[0], &*args[1]) {
                    (MalType::Int(a), MalType::Int(b)) => Some(Rc::new(MalType::Int(a / b))),
                    _ => None,
                }
            })),
        );
    }
}
