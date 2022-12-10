use std::{io::stdin, collections::HashSet};

enum Direction { 
    LEFT,
    RIGHT,
    UP,
    DOWN
}
type Location = (i32, i32);
fn new_loc(head: Location, tail: Location) -> Location {

    if (head.0 - tail.0).abs() > 1 && (head.1 - tail.1).abs() > 1 {
        return (tail.0 + (head.0 - tail.0)/2, tail.1 + (head.1 - tail.1)/2);
    }
    else if (head.1 - tail.1).abs() > 1 {
        return (tail.0 + (head.0 - tail.0), tail.1 + (head.1 - tail.1)/2);
    }
    else if (head.0 - tail.0).abs() > 1 {
        return (tail.0 + (head.0 - tail.0)/2, tail.1 + (head.1 - tail.1));
    }
    tail
}

pub fn main() {
    let mut visited = HashSet::new();
    let mut locations = [(100, 100); 10];
    visited.insert(locations[9]);
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
                    match direction {
                        Direction::LEFT => {locations[0] = (locations[0].0-1, locations[0].1)}
                        Direction::RIGHT => {locations[0] = (locations[0].0+1, locations[0].1)}
                        Direction::DOWN => {locations[0] = (locations[0].0, locations[0].1+1)}
                        Direction::UP => {locations[0] = (locations[0].0, locations[0].1-1)}
                    }

                    for i in 1..10 {
                        locations[i] = new_loc(locations[i-1], locations[i]);
                    } 
                    visited.insert(locations[9]);

                }

            }
            Err(error) => {println!("error: {error}"); break;},
        } 
    }
    println!("{:?}", visited);
    println!("the amount of unique spaces the tail visited is: {}", visited.len());
}

