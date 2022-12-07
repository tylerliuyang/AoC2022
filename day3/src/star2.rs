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
                let mut inventory = HashMap::<char, u32>::new();

                let _: Vec<()> = input.chars().map(|c| -> () {
                    if inventory.contains_key(&c) {
                        return;
                    }
                    inventory.insert(c, 1);
                }).collect();
                
                for _x in 0..2 {
                    let mut input = String::new();
                    stdin().read_line(&mut input).unwrap();
                    input = input.strip_suffix("\r\n").unwrap().to_string();
                    
                    let mut seen = HashSet::<char>::new();
                    let _: Vec<()> = input.chars().map(|c| -> () {
                        if seen.contains(&c) {
                            return;
                        }
                        seen.insert(c);
                        inventory.insert(c, *inventory.get(&c).unwrap_or(&0) + 1);
                    }).collect();
                }

                let tag = inventory.iter().find(|item| {
                    *item.1 == 3
                }).unwrap().0;
                println!("{}", tag);

                if tag.is_uppercase() {
                    priorities += tag.to_digit(36).unwrap() - 9 + 26;
                } else {
                    priorities += tag.to_digit(36).unwrap() - 9;
                }
            }
            Err(error) => {println!("error: {error}"); break;},
        } 
    }
    println!("the priorities sum to {}", priorities);
}

