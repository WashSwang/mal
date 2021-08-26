use crate::types::{MalType, KV};
use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, tag, take_till1, take_while1},
    character::complete::{char, digit1, none_of},
    combinator::{eof, map, map_res, opt, recognize, success, value},
    multi::many0,
    sequence::{delimited, pair, preceded, terminated},
    IResult,
};
use std::rc::Rc;
use std::str::FromStr;

// space and comma
fn spc(input: &str) -> IResult<&str, Vec<&str>> {
    let chars = " \t\n,";
    many0(alt((
        take_while1(move |c| chars.contains(c)),
        preceded(char(';'), take_while1(|c| c != '\n')),
    )))(input)
}

fn parse_boolean(input: &str) -> IResult<&str, bool> {
    let parse_true = value(true, tag("true"));
    let parse_false = value(false, tag("false"));
    alt((parse_true, parse_false))(input)
}

fn parse_nil(input: &str) -> IResult<&str, ()> {
    value((), tag("nil"))(input)
}

fn parse_i32(input: &str) -> IResult<&str, i32> {
    map_res(recognize(pair(opt(char('-')), digit1)), FromStr::from_str)(input)
}

fn parse_str(input: &str) -> IResult<&str, String> {
    delimited(
        char('\"'),
        alt((
            escaped_transform(
                none_of("\\\""),
                '\\',
                alt((
                    value("\\", tag("\\")),
                    value("\"", tag("\"")),
                    value("\n", tag("n")),
                )),
            ),
            success(String::from("")),
        )),
        char('\"'),
    )(input)
}

fn parse_list(input: &str) -> IResult<&str, Vec<Rc<MalType>>> {
    // unable to support statements like (()()) because the sep in separated_list should always consume sth
    // delimited(char('('), delimited(spc, separated_list0(spc1, parse_mal), spc), char(')'))(input)
    delimited(
        char('('),
        delimited(spc, many0(preceded(spc, parse_mal)), spc),
        char(')'),
    )(input)
}

fn parse_vec(input: &str) -> IResult<&str, Vec<Rc<MalType>>> {
    delimited(
        char('['),
        delimited(spc, many0(preceded(spc, parse_mal)), spc),
        char(']'),
    )(input)
}

fn parse_symbol(input: &str) -> IResult<&str, &str> {
    let chars = "{[()]} \t,;:\"\n";
    take_till1(move |c| chars.contains(c))(input)
}

fn parse_keyword(input: &str) -> IResult<&str, &str> {
    preceded(char(':'), parse_symbol)(input)
}

fn parse_hash_map_kv(input: &str) -> IResult<&str, KV> {
    pair(terminated(parse_mal, spc), parse_mal)(input)
}

fn parse_hash_map(input: &str) -> IResult<&str, Vec<KV>> {
    delimited(
        char('{'),
        delimited(spc, many0(preceded(spc, parse_hash_map_kv)), spc),
        char('}'),
    )(input)
}

fn parse_quote(input: &str) -> IResult<&str, Vec<Rc<MalType>>> {
    alt((
        map(
            pair(
                alt((tag("~@"), tag("\'"), tag("`"), tag("@"), tag("~"))),
                parse_mal,
            ),
            |x| match x.0 {
                "~@" => vec![
                    Rc::new(MalType::Symbol(String::from("splice-unquote"))),
                    x.1,
                ],
                "\'" => vec![Rc::new(MalType::Symbol(String::from("quote"))), x.1],
                "`" => vec![Rc::new(MalType::Symbol(String::from("quasiquote"))), x.1],
                "@" => vec![Rc::new(MalType::Symbol(String::from("deref"))), x.1],
                "~" => vec![Rc::new(MalType::Symbol(String::from("unquote"))), x.1],
                _ => vec![Rc::new(MalType::Nil)],
            },
        ),
        map(
            preceded(tag("^"), pair(parse_mal, preceded(spc, parse_mal))),
            |x| {
                vec![
                    Rc::new(MalType::Symbol(String::from("with-meta"))),
                    x.1,
                    x.0,
                ]
            },
        ),
    ))(input)
}

fn parse_mal(input: &str) -> IResult<&str, Rc<MalType>> {
    map(
        alt((
            map(parse_hash_map, MalType::HashMap),
            map(parse_str, MalType::Str),
            map(parse_vec, MalType::Vector),
            map(parse_i32, MalType::Int),
            map(parse_boolean, MalType::Bool),
            map(parse_nil, |_| MalType::Nil),
            map(parse_keyword, |s| MalType::Keyword(String::from(s))),
            map(parse_list, MalType::List),
            map(parse_quote, MalType::List),
            map(parse_symbol, |s| MalType::Symbol(String::from(s))),
        )),
        Rc::new,
    )(input)
}

// fn parse_comment(input: &str) -> IResult<&str, Option<char>> {
//     opt(terminated(char(';'), take_while(|_| true)))(input)
// }

pub fn read_str(input: &str) -> IResult<&str, Rc<MalType>> {
    terminated(delimited(spc, parse_mal, spc), eof)(input)
}
