use std::{io::stdin};


pub fn main() {
    let mut sum = 0;
    let mut register = 1;
    let mut counter = 0;
    loop {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(0) => {break;}
            Ok(_) => {
                counter += 1;
                if (counter-20) % 40 == 0 { sum += register * counter; }

                input = input.strip_suffix("\r\n").unwrap().to_string();
                let values = input.split(" ").collect::<Vec<&str>>();
                if values.get(0).unwrap() == &"noop" {
                    continue;
                }
                let add: i32 = values.get(1).unwrap().parse().unwrap();
                counter += 1;
                if (counter-20) % 40 == 0 { sum += register * counter; }
                register += add;
            }
            Err(error) => {println!("error: {error}"); break;},
        } 
    }
    println!("the sum is: {}", sum);
}

