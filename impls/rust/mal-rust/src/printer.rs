use crate::types::{MalType, KV};
use std::cell::RefCell;
use std::rc::Rc;

fn dump_hash_map(kvs: &[KV], print_readably: bool) -> String {
    let mut output = String::from("{");
    for (i, (k, v)) in kvs.iter().enumerate() {
        output += &dump_mal(k.clone(), print_readably);
        output.push(' ');
        output += &dump_mal(v.clone(), print_readably);
        if i != kvs.len() - 1 {
            output.push(' ');
        }
    }
    output.push('}');
    output
}

fn dump_str(string: &str, print_readably: bool) -> String {
    let mut output = String::from("");
    if print_readably {
        output.push('\"');
        for c in string.chars() {
            match c {
                '\\' => output.push_str("\\\\"),
                '\"' => output.push_str("\\\""),
                '\n' => output.push_str("\\n"),
                _ => output.push(c),
            }
        }
        output.push('\"');
    } else {
        output.push_str(string);
    }
    output
}

fn dump_vec(items: &[Rc<MalType>], print_readably: bool) -> String {
    let mut output = String::from('[');
    for (i, item) in items.iter().enumerate() {
        output += &dump_mal(item.clone(), print_readably);
        if i != items.len() - 1 {
            output.push(' ');
        }
    }
    output.push(']');
    output
}

fn dump_i32(value: &i32) -> String {
    value.to_string()
}

fn dump_boolean(value: &bool) -> String {
    value.to_string()
}

fn dump_keyword(keyword: &str) -> String {
    format!(":{}", keyword)
}

fn dump_list(items: &[Rc<MalType>], print_readably: bool) -> String {
    let mut output = String::from('(');
    for (i, item) in items.iter().enumerate() {
        output += &dump_mal(item.clone(), print_readably);
        if i != items.len() - 1 {
            output.push(' ');
        }
    }
    output.push(')');
    output
}

fn dump_symbol(symbol: &str) -> String {
    String::from(symbol)
}

fn dump_atom(value: &RefCell<Rc<MalType>>, print_readably: bool) -> String {
    format!(
        "(atom {})",
        dump_mal(value.borrow().clone(), print_readably)
    )
}

fn dump_mal_debug(mal: Rc<MalType>, print_readably: bool) -> String {
    match &*mal {
        MalType::HashMap(kvs) => String::from("Hash:") + &dump_hash_map(kvs, print_readably),
        MalType::Str(string) => String::from("Str:") + &dump_str(string, print_readably),
        MalType::Vector(items) => String::from("Vec:") + &dump_vec(items, print_readably),
        MalType::Int(value) => String::from("Int:") + &dump_i32(value),
        MalType::Bool(value) => String::from("Bool:") + &dump_boolean(value),
        MalType::Nil => String::from("nil"),
        MalType::Keyword(keyword) => String::from("Key:") + &dump_keyword(keyword),
        MalType::List(items) => String::from("List:") + &dump_list(items, print_readably),
        MalType::Symbol(symbol) => String::from("Sym:") + &dump_symbol(symbol),
        MalType::Atom(value) => dump_atom(value, print_readably),
        MalType::Func(_) | MalType::BuiltinFunc(_) => String::from("#<function>"),
    }
}

fn dump_mal(mal: Rc<MalType>, print_readably: bool) -> String {
    match &*mal {
        MalType::HashMap(kvs) => dump_hash_map(kvs, print_readably),
        MalType::Str(string) => dump_str(string, print_readably),
        MalType::Vector(items) => dump_vec(items, print_readably),
        MalType::Int(value) => dump_i32(value),
        MalType::Bool(value) => dump_boolean(value),
        MalType::Nil => String::from("nil"),
        MalType::Keyword(keyword) => dump_keyword(keyword),
        MalType::List(items) => dump_list(items, print_readably),
        MalType::Symbol(symbol) => dump_symbol(symbol),
        MalType::Atom(value) => dump_atom(value, print_readably),
        MalType::Func(_) | MalType::BuiltinFunc(_) => String::from("#<function>"),
    }
}

pub fn print_str(mal: Rc<MalType>, debug: bool, print_readably: bool) -> String {
    if debug {
        dump_mal_debug(mal, print_readably)
    } else {
        dump_mal(mal, print_readably)
    }
}
