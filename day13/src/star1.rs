use std::{io::stdin, sync::Mutex, collections::{HashMap}};
use itertools::EitherOrBoth::{Both, Right, Left};
use itertools::Itertools;

#[derive(Debug, PartialEq)]
enum Compare {
    LESS,
    EQUAL,
    GREATER
}

fn split<'a>(s: &'a String) -> Vec<&'a str> {
    let mut counter = 0;
    let mut last_elt = 0;
    let mut out = Vec::new();
    for (i, c) in s.chars().enumerate() {
        if c == '[' {counter += 1;}
        if c == ']' {counter -= 1;}
        if c == ',' {
            if counter != 0 {continue;}
            out.push(&s[last_elt..i]);
            last_elt = i+1;
        }
    };
    if last_elt != s.len() {
        out.push(&s[last_elt..s.len()]);
    }
    println!("{:?}", out);
    return out;
}

fn compare_nested(str1: String, str2: String, depth: i32) -> Compare {
    let d = depth.to_string();
    let split1 = split(&str1);
    let split2 = split(&str2);
    // let split1 = str1.split(",").collect::<Vec<&str>>();
    // let split2 = str2.split(",").collect::<Vec<&str>>();
    for pair in split1.iter().zip_longest(split2) {
        let (value1, value2) = match pair {
            Both(a, b) => {(*a, b)}
            Right(b) => {(d.as_str(), b)}
            Left(a) => {(*a, d.as_str())}
        };
        println!("{:?} {:?}", value1, value2);

        if value1.chars().nth(0).unwrap() == '[' {
            let mut input2 = value2;
            if value2.chars().nth(0).unwrap() == '[' {
                input2 = &value2[1..value2.len()-1];
            }
            match compare_nested(value1[1..value1.len()-1].to_string(), input2.to_string(), depth + 1) {
                Compare::LESS => {return Compare::LESS}
                Compare::EQUAL => {continue}
                Compare::GREATER => {return Compare::GREATER}
            }
        } else if value2.chars().nth(0).unwrap() == '[' {
            let mut input1 = value1;
            if value1.chars().nth(0).unwrap() == '[' {
                input1 = &value1[1..value1.len()-1];
            }
            match compare_nested(input1.to_string(), value2[1..value2.len()-1].to_string(), depth + 1) {
                Compare::LESS => {return Compare::LESS}
                Compare::EQUAL => {continue}
                Compare::GREATER => {return Compare::GREATER}
            }
        } 
        else {
            if value1.parse::<i32>().unwrap() < value2.parse().unwrap() {
                return Compare::LESS;
            } else if value1.parse::<i32>().unwrap() > value2.parse().unwrap() {
                return Compare::GREATER;
            } else {
                continue;
            }
        }
    };
    return Compare::EQUAL;
}

pub fn main() {
    let mut indexes: Vec<usize> = Vec::new();
    let mut i = 0;
    loop {
        let mut input = String::new();
        let mut comparison = String::new();
        match stdin().read_line(&mut input) {
            Ok(0) => {
                break;
            }
            Ok(2) => {
                continue;
            }
            Ok(_) => {
                i += 1;
                stdin().read_line(&mut comparison).unwrap();
                input = input.strip_suffix("\r\n").unwrap().to_string();
                comparison = comparison.strip_suffix("\r\n").unwrap().to_string();
                let out = compare_nested(input[1..input.len()-1].to_string(), comparison[1..comparison.len()-1].to_string(), -100);
                println!("{:?}", out);
                if out == Compare::LESS {
                    indexes.push(i);
                }
            }
            Err(error) => {println!("error: {error}"); break;},
        } 
    }
    println!("{}", indexes.iter().fold(0, |a, elt| {a + elt}));
}