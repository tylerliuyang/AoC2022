use std::{io::stdin, collections::{HashMap, HashSet}};

pub fn main() {
    let mut priorities = 0;
    loop {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                input = input.strip_suffix("\r\n").unwrap().to_string();
                let len = input.len();
                let (first, last) = input.split_at(len/2);
                let mut inventory = HashSet::<char>::new();

                let _: Vec<()> = first.chars().map(|c| -> () {
                    inventory.insert(c);
                }).collect();

                let repeat = last.chars().find(|c| -> bool {
                    inventory.contains(&c)
                }).unwrap();

                if repeat.is_uppercase() {
                    priorities += repeat.to_digit(36).unwrap() - 9 + 26;
                } else {
                    priorities += repeat.to_digit(36).unwrap() - 9;
                }

            }
            Err(error) => {println!("error: {error}"); break;},
        } 
    }
    println!("the priorities sum to {}", priorities);
}

