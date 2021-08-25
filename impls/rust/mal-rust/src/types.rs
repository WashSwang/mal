use std::rc::Rc;

pub type KV = (Rc<MalType>, Rc<MalType>);
pub type Closure = dyn Fn(&[Rc<MalType>]) -> Option<Rc<MalType>>;

#[derive(Clone)]
pub enum MalType {
    Int(i32),
    Symbol(String),
    List(Vec<Rc<MalType>>),
    HashMap(Vec<KV>),
    Keyword(String),
    Str(String),
    Vector(Vec<Rc<MalType>>),
    Bool(bool),
    BuiltinFunc(fn(&[Rc<MalType>]) -> Option<Rc<MalType>>),
    Func(Rc<Closure>),
    Nil,
}

impl PartialEq for MalType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MalType::Int(i1), MalType::Int(i2)) => i1 == i2,
            (MalType::Symbol(s1), MalType::Symbol(s2)) => s1 == s2,
            (MalType::Str(s1), MalType::Str(s2)) => s1 == s2,
            (MalType::List(l1) | MalType::Vector(l1), MalType::List(l2) | MalType::Vector(l2)) => {
                if l1.len() != l2.len() {
                    return false;
                }
                for (item1, item2) in l1.iter().zip(l2.iter()) {
                    if **item1 != **item2 {
                        return false;
                    }
                }
                true
            }
            (MalType::Keyword(k1), MalType::Keyword(k2)) => k1 == k2,
            (MalType::Bool(b1), MalType::Bool(b2)) => b1 == b2,
            (MalType::Nil, MalType::Nil) => true,
            (_, _) => false,
        }
    }
}
