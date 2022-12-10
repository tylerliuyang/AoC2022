use std::{io::stdin};

fn draw(counter: i32, register: i32) {
    if counter % 40 >= register && counter % 40 <= register + 2 {
        print!("#");
    } else {
        print!(".");
    }
    if counter % 40 == 0 {
        print!("\n");
    }
}


pub fn main() {
    let mut register = 1;
    let mut counter = 0;
    loop {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(0) => {break;}
            Ok(_) => {
                counter += 1;
                draw(counter, register);

                input = input.strip_suffix("\r\n").unwrap().to_string();
                let values = input.split(" ").collect::<Vec<&str>>();
                if values.get(0).unwrap() == &"noop" {
                    continue;
                }
                let add: i32 = values.get(1).unwrap().parse().unwrap();
                counter += 1;
                draw(counter, register);

                register += add;
            }
            Err(error) => {println!("error: {error}"); break;},
        } 
    }
}

