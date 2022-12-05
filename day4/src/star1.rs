use std::{io::stdin};

pub fn main() {
    let mut overlaps = 0;
    loop {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                input = input.strip_suffix("\r\n").unwrap().to_string();
                let parts: Vec<&str> = input.split(',').collect();
                let indicies: Vec<&str> = parts.iter().map(|item| {
                    item.split('-')
                }).flatten().collect();
                let values: Vec<u32> = indicies.iter().map(|item| -> u32 {
                    item.parse().unwrap()
                }).collect();
                
                if values.get(0) <= values.get(2) && values.get(1) >= values.get(3) {
                    println!("{:?}", values);
                    overlaps += 1;
                } else if values.get(0) >= values.get(2) && values.get(1) <= values.get(3) {
                    println!("{:?}", values);

                    overlaps += 1;
                };
            }
            Err(error) => {println!("error: {error}"); break;},
        } 
    }
    println!("the amount of complete overlaps are: {}", overlaps);
}

