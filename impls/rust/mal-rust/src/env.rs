use crate::types::MalType;
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

pub struct Env {
    pub map: HashMap<String, Rc<MalType>>,
    pub outer: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn new_root() -> Self {
        Self {
            map: HashMap::new(),
            outer: None,
        }
        //env.load_builtin();
    }

    pub fn new(outer: Rc<RefCell<Env>>) -> Self {
        Self {
            map: HashMap::new(),
            outer: Some(outer),
        }
    }

    pub fn new_bind(outer: Rc<RefCell<Env>>, binds: &[&str], exprs: &[Rc<MalType>]) -> Self {
        let mut env = Env::new(outer);
        for (bind, expr) in binds.iter().zip(exprs.iter()) {
            env.map.insert(String::from(*bind), expr.clone());
        }
        env
    }

    pub fn get(&self, symbol: &str) -> Option<Rc<MalType>> {
        let result = self.map.get(symbol).cloned();
        if result.is_none() {
            self.outer.as_ref()?.borrow().get(symbol)
        } else {
            result
        }
    }

    pub fn set(&mut self, symbol: &str, mal: Rc<MalType>) {
        self.map.insert(String::from(symbol), mal);
    }
}
