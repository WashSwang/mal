#[derive(Debug)]
pub enum MalType {
    Int(i32),
    Symbol(String),
    List(Vec<MalType>),
    HashMap(Vec<(MalType, MalType)>),
    Keyword(String),
    Str(String),
    Vector(Vec<MalType>),
    Bool(bool),
    Nil,
}

impl PartialEq for MalType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MalType::Int(i1), MalType::Int(i2)) => i1 == i2,
            (MalType::Symbol(s1), MalType::Symbol(s2)) => s1 == s2,
            (MalType::List(l1), MalType::List(l2)) => {
                let mut x = true;
                if l1.len() != l2.len() {
                    return false;
                } else {
                    for i in 0..l1.len() {
                        if !l1[i].eq(&l2[i]) {
                            x = false;
                        }
                    }
                }
                x
            }
            _ => false,
        }
    }
}
