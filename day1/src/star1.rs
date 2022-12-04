use std::{io::stdin};

pub fn main() {
    let mut max_elf = 0;
    let mut local_elf = 0; 
    loop {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(0) => {
                break;
            }
            Ok(2) => {
                if local_elf > max_elf {max_elf = local_elf;}
                local_elf = 0;
            }
            Ok(_) => {
                input = input.strip_suffix("\r\n").unwrap().to_string();
                local_elf += input.parse::<i32>().unwrap();
            }
            Err(error) => {println!("error: {error}"); break;},
        } 
    }
    println!("the most calories is {}", max_elf);
}

