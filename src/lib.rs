// #![allow(dead_code)]
/*

Passing iterator to each parser

Single Word Parser: fn(iter: Chars) -> Result<(iter, result),notfound>
Multi Word Parser: fn(iter: Chars) -> Result<(iter, result),notfound>


*/

pub trait Pattern: Fn(char) -> Option<char> {}
impl<T: Fn(char) -> Option<char>> Pattern for T {}

pub trait Parser: Fn(Chars) -> Option<String> {}
impl<T: Fn(Chars) -> Option<String>> Parser for T {}

use std::str::Chars;
#[test]
fn test() {
    let a = "_const iter = 5;";
    let b = "?-99 true { }";

    let parser = one_or_more(any_but('t'));

    let new_var = parser(a.chars()).unwrap();
    let condition = parser(b.chars()).unwrap();

    assert_eq!(new_var, "_cons".to_string());
    assert_eq!(condition, "?-99 ".to_string());

    let word_number = "more12and4 and then2";
    let iter = word_number.chars();

    let match_word_or_number = one_or_more(or(word, number));

    assert_eq!(match_word_or_number(iter), Some("more12and4".to_string()));
}

pub enum PatternType {
    Word(char),
    Number(char),
    Space(char),
    Operator(char),
    AnyLine(char),
}

pub fn resolve_remainder(input: &mut Chars) -> Option<String> {
    let collect = input.collect::<Vec<char>>();
    Some(collect.into_iter().collect())
}

pub fn word(ch: char) -> Option<char> {
    if ch.is_alphabetic() { Some(ch) } else { None }
}

pub fn number(ch: char) -> Option<char> {
    if ch.is_numeric() { Some(ch) } else { None }
}

pub fn space(ch: char) -> Option<char> {
    if ch.is_whitespace() { Some(ch) } else { None }
}

pub const OPERATOR: [char; 30] = [
    '`', '~', '!', '@', '#', '%', '^', '&', '*', '(', ')', '-', '=', '+', 
    '{', '}', '[', ']', '\\', '|', ';', ':', '\'', '"', ',', '<', '>', 
    '.', '/', '?'
];
pub fn operator(ch: char) -> Option<char> {
    if OPERATOR.iter().any(|e| e == &ch) { Some(ch) } else { None }
}

pub fn any_line(ch: char) -> Option<char> {
    if ch != '\n' { Some(ch) } else { None }
}

pub fn literal(target: char) -> impl Pattern {
    move |ch| { if ch == target { Some(ch) } else { None } }
}

pub fn any_but(target: char) -> impl Pattern {
    move |ch| { if ch != target { Some(ch) } else { None } }
}

pub fn any_but_pattern<T>(pattern: T) -> impl Pattern
where
    T: Pattern,
{
    move |ch| { if let None = pattern(ch) { Some(ch) } else { None } }
}


pub fn or<T, U>(pattern1: T, pattern2: U) -> impl Pattern
where
    T: Pattern,
    U: Pattern,
{
    move |ch| { if let Some(_) = pattern1(ch) { Some(ch) } else if let Some(_) = pattern2(ch) { Some(ch) } else { None } }
}


pub fn one<T>(pattern: T) -> impl Parser
where
    T: Pattern,
{
    move |mut input: Chars| {
        if let Some(ch) = input.next() {
            if let Some(_) = pattern(ch) {
                return Some(ch.to_string());
            }
        }
        None
    }
}

pub fn one_or_more<T>(pattern: T) -> impl Parser
where
    T: Pattern,
{
    move |mut input: Chars| {
        let mut state = String::new();

        while let Some(ch) = input.next() {
            match pattern(ch) {
                Some(_) => state.push(ch),
                None => break,
            }
        }

        if state.is_empty() { None } else { Some(state) }
    }
}
