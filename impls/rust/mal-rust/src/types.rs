use std::rc::Rc;

pub type KV = (Rc<MalType>, Rc<MalType>);

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
    Nil,
}
