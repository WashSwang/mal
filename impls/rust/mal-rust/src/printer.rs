use crate::types::MalType;

fn dump_hash_map(kvs: &[(MalType, MalType)]) -> String {
    let mut output = String::from("{");
    for (i, (k, v)) in kvs.iter().enumerate() {
        output += &dump_mal(k);
        output.push(' ');
        output += &dump_mal(v);
        if i != kvs.len() - 1 {
            output.push(' ');
        }
    }
    output.push('}');
    output
}

fn dump_str(string: &str) -> String {
    let mut output = String::from('\"');
    for c in string.chars() {
        match c {
            '\\' => output.push_str("\\\\"),
            '\"' => output.push_str("\\\""),
            '\n' => output.push_str("\\n"),
            _ => output.push(c),
        }
    }
    output.push('\"');
    output
}

fn dump_vec(items: &[MalType]) -> String {
    let mut output = String::from('[');
    for (i, item) in items.iter().enumerate() {
        output += &dump_mal(item);
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

fn dump_list(items: &[MalType]) -> String {
    let mut output = String::from('(');
    for (i, item) in items.iter().enumerate() {
        output += &dump_mal(item);
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

fn dump_nil() -> String {
    String::from("nil")
}

fn dump_mal_debug(mal: &MalType) -> String {
    match mal {
        MalType::HashMap(kvs) => String::from("Hash:") + &dump_hash_map(kvs),
        MalType::Str(string) => String::from("Str:") + &dump_str(string),
        MalType::Vector(items) => String::from("Vec:") + &dump_vec(items),
        MalType::Int(value) => String::from("Int:") + &dump_i32(value),
        MalType::Bool(value) => String::from("Bool:") + &dump_boolean(value),
        MalType::Nil => String::from("Nil:") + &dump_nil(),
        MalType::Keyword(keyword) => String::from("Key:") + &dump_keyword(keyword),
        MalType::List(items) => String::from("List:") + &dump_list(items),
        MalType::Symbol(symbol) => String::from("Sym:") + &dump_symbol(symbol),
    }
}

fn dump_mal(mal: &MalType) -> String {
    match mal {
        MalType::HashMap(kvs) => dump_hash_map(kvs),
        MalType::Str(string) => dump_str(string),
        MalType::Vector(items) => dump_vec(items),
        MalType::Int(value) => dump_i32(value),
        MalType::Bool(value) => dump_boolean(value),
        MalType::Nil => dump_nil(),
        MalType::Keyword(keyword) => dump_keyword(keyword),
        MalType::List(items) => dump_list(items),
        MalType::Symbol(symbol) => dump_symbol(symbol),
    }
}

pub fn print_str(mal: &MalType, debug: bool) -> String {
    if debug {
        dump_mal_debug(mal)
    } else {
        dump_mal(mal)
    }
}
