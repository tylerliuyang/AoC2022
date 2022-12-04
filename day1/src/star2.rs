use std::{io::stdin};

pub fn main() {
    let mut elf_1 = 0;
    let mut elf_2 = 0;
    let mut elf_3 = 0;
    let mut local_elf = 0; 
    loop {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(0) => {
                break;
            }
            Ok(2) => {
                if local_elf > elf_1 {
                    elf_3 = elf_2;
                    elf_2 = elf_1;
                    elf_1 = local_elf;
                }
                else if local_elf > elf_2 {
                    elf_3 = elf_2;
                    elf_2 = local_elf;
                }
                else if local_elf > elf_3 {
                    elf_3 = local_elf;
                }
                local_elf = 0;
            }
            Ok(_) => {
                input = input.strip_suffix("\r\n").unwrap().to_string();
                local_elf += input.parse::<i32>().unwrap();
            }
            Err(error) => {println!("error: {error}"); break;},
        } 
    }
    println!("the top 3 elves have {}", elf_1 + elf_2 + elf_3);
}

