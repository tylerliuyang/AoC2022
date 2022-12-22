use std::io::{stdin, Read};

mod other;
mod star1;
mod star2;
fn main() {
    let mut input = String::new();
    loop {
        match stdin().read_line(&mut input) {
            Ok(0) => break,
            _ => continue,
        }
    }
    input = input.strip_suffix("\r\n").unwrap().to_string();
    println!("{}", other::part_2(&input));
    star2::main();
}
