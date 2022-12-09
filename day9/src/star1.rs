use std::{io::stdin, collections::HashSet};

enum Direction { 
    LEFT,
    RIGHT,
    UP,
    DOWN
}
type Location = (i32, i32);
fn new_loc(head: Location, tail: Location, old_head: Location) -> Location {
    if (head.0 - tail.0).abs() > 1 {
        return old_head;
    }
    if (head.1 - tail.1).abs() > 1 {
        return old_head;
    }
    tail
}

pub fn main() {
    let mut visited = HashSet::new();
    let mut head: Location = (0, 4);
    let mut tail: Location = (0, 4);
    visited.insert(tail);
    loop {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(0) => {break;}
            Ok(_) => {
                input = input.strip_suffix("\r\n").unwrap().to_string();
                let values = input.split(" ").collect::<Vec<&str>>();
                let direction = match values.get(0).unwrap().chars().nth(0) {
                    Some('L') => {Direction::LEFT}
                    Some('R') => {Direction::RIGHT}
                    Some('U') => {Direction::UP}
                    Some('D') => {Direction::DOWN}
                    _ => {panic!("ahhhh");}
                };
                let amount: i32 = values.get(1).unwrap().parse().unwrap();

                for _ in 0..amount {
                    let old_head = head;
                    match direction {
                        Direction::LEFT => {head = (head.0-1, head.1)}
                        Direction::RIGHT => {head = (head.0+1, head.1)}
                        Direction::DOWN => {head = (head.0, head.1+1)}
                        Direction::UP => {head = (head.0, head.1-1)}
                    }
                    tail = new_loc(head, tail, old_head);
                    visited.insert(tail);
                }

            }
            Err(error) => {println!("error: {error}"); break;},
        } 
    }
    println!("{:?}", visited);
    println!("the amount of unique spaces the tail visited is: {}", visited.len());
}

