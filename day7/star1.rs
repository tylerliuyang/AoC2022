use std::{io::stdin, collections::HashSet, process::exit};

pub fn main() {
    loop {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                let mut four_chars = String::new();
                input = input.strip_suffix("\r\n").unwrap().to_string();
                let mut i = 0;
                while !input.is_empty() {
                    i += 1;
                    four_chars.push(input.remove(0));
                    if four_chars.len() > 4 {
                        four_chars.remove(0);
                    } else if four_chars.len() < 4 {
                        continue;
                    }
                    let mut count = HashSet::new();
                    if four_chars.chars().any(|c| {!count.insert(c)}) {
                        continue;
                    }
                    println!("index: {} contains an acceptable buffer", i);
                    exit(0);

                }    
            }
            Err(error) => {println!("error: {error}"); break;},
        } 
    }
}

