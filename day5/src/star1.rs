use std::{io::stdin, sync::Mutex};

pub fn main() {
    let crates: [Mutex<Vec<char>>; 10] = Default::default();

    for _rows in 0..8 {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        for j in 1..10 {
            input.remove(0);
            match input.remove(0) {
                ' ' => {}
                _value => {crates.get(j).unwrap().lock().unwrap().push(_value);}
            };
            input.remove(0);
            input.remove(0);
        }
    }
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    stdin().read_line(&mut input).unwrap();

    loop {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                input = input.strip_suffix("\r\n").unwrap().to_string();
                let parts: Vec<&str> = input.split(' ').collect();
                println!("{:?}", parts);
                let amount: usize = parts.get(1).unwrap().parse().unwrap();
                let source: usize = parts.get(3).unwrap().parse().unwrap();
                let destination: usize = parts.get(5).unwrap().parse().unwrap();
                println!("{} {} {}", amount, source, destination);
                for _ in 0..amount {
                    let to_move = crates.get(source).unwrap().lock().unwrap().remove(0);
                    crates.get(destination).unwrap().lock().unwrap().insert(0, to_move);
                }
                
            }
            Err(error) => {println!("error: {error}"); break;},
        } 
    }

    println!("the order of crates is");
    crates.map(|m| {
        print!("{}", m.lock().unwrap().get(0).unwrap_or(&'_'));
    });
}

