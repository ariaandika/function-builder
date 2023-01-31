#![allow(dead_code)]

use std::str::Chars;

fn main() {
    let input = "soasasd55as5ddas5d55asd lines
    next word";
    
    let parser = one_or_more( or(word,number) );
    let iter = input.chars();
    
    match parser(iter) {
        Some(result) => println!("{}",result),
        None => println!("Not Found"),
    };
    
}

#[test]
fn test() {
    let word_number = "more12and4 and then2";
    let iter = word_number.chars();
    
    let match_word_or_number = one_or_more( or(word, number) );
    
    assert_eq!( match_word_or_number(iter), Some("more12and4".to_string()) );
}

/*

Passing iterator to each parser

Single Word Parser: fn(iter: Chars) -> Result<(iter, result),notfound>
Multi Word Parser: fn(iter: Chars) -> Result<(iter, result),notfound>



*/

// trait Parser: FnOnce(char) -> Result<char,char> {}
// impl<T: FnOnce(char) -> Result<char,char>> Parser for T {}



fn resolve_remainder(input: &mut Chars) -> Option<String> {
    let collect = input.collect::<Vec<char>>();
    Some( collect.into_iter().collect() )
}

fn word(ch: char) -> Result<char,char> {
    if ch.is_alphabetic() { Ok(ch) } else { Err(ch) }
}

fn number(ch: char) -> Result<char,char> {
    if ch.is_numeric() { Ok(ch) } else { Err(ch) }
}

fn space(ch: char) -> Result<char,char> {
    if ch.is_whitespace() { Ok(ch) } else { Err(ch) }
}

fn any_line(ch: char) -> Result<char,char> {
    if ch != '\n' { Ok(ch) } else { Err(ch) }
}

fn any_but(target: char) -> impl Fn(char) -> Result<char,char> {
    move |ch| {
        if ch != target { Ok(ch) } else { Err(ch) }
    }
}

fn literal(target: char) -> impl Fn(char) -> Result<char,char> {
    move |ch| {
        if ch == target { Ok(ch) } else { Err(ch) }
    }
}


fn or<T,U>(pattern1: T, pattern2: U) -> impl Fn(char) -> Result<char,char>
    where 
    T: Fn(char) -> Result<char,char>,
    U: Fn(char) -> Result<char,char>
{
    move |ch| {
        if let Ok(_) = pattern1(ch) {
            return Ok(ch)
        }
        else if let Ok(_) = pattern2(ch) {
            return Ok(ch)
        }
        return Err(ch)
    }
}


fn one<T>(pattern: T) -> impl Fn(Chars) -> Option<String>
    where T: Fn(char) -> Result<char,char>
{
    move |mut input| {        
        // found a character
        if let Some(ch) = input.next() {
            if let Ok(_) = pattern(ch) {
                return Some(ch.to_string())
            }
        }
        None
    }
}


fn one_or_more<T>(pattern: T) -> impl Fn(Chars) -> Option<String>
    where T: Fn(char) -> Result<char,char>
{
    move |mut input| {
        let mut state = String::new();
        
        // found a character
        while let Some(ch) = input.next() {
            match pattern(ch) {
                Ok(_) => state.push(ch),
                Err(_) => break,
            }
        }
        
        if state.is_empty() { None } else { Some(state) }
    }
}
