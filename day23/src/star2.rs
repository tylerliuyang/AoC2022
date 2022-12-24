use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    io::stdin,
};

struct MoveOrder {
    count: usize,
}

#[derive(Default, Debug)]
enum Direction {
    #[default]
    NORTH,
    SOUTH,
    WEST,
    EAST,
}

impl MoveOrder {
    fn new() -> MoveOrder {
        MoveOrder { count: 0 }
    }
    fn get_next(&self) -> [Direction; 4] {
        let mut output: [Direction; 4] = Default::default();
        for i in 0..4 {
            output[i] = match (self.count + i) % 4 {
                0 => Direction::NORTH,
                1 => Direction::SOUTH,
                2 => Direction::WEST,
                3 => Direction::EAST,
                _ => unreachable!(),
            };
        }
        output
    }

    fn next(&mut self) {
        self.count = (self.count + 1) % 4;
    }
}

fn alone(pos: (i32, i32), set: &HashSet<(i32, i32)>) -> bool {
    let directions = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, -1),
        (-1, 0),
        (1, -1),
        (-1, 1),
        (0, -1),
    ];
    for d in directions {
        if set.contains(&(pos.0 + d.0, pos.1 + d.1)) {
            return false;
        }
    }
    return true;
}

fn empty(pos: (i32, i32), set: &HashSet<(i32, i32)>, to_move: Vec<(i32, i32)>) -> bool {
    for (fst, snd) in to_move {
        if set.contains(&(pos.0 + fst, pos.1 + snd)) {
            return false;
        }
    }
    true
}

fn print_board(pos: &HashSet<(i32, i32)>) {
    let left = pos.iter().fold(i32::MAX, |acc, elt| min(acc, elt.0));
    let right = pos.iter().fold(i32::MIN, |acc, elt| max(acc, elt.0 + 1));
    let bottom = pos.iter().fold(i32::MAX, |acc, elt| min(acc, elt.1));
    let top = pos.iter().fold(i32::MIN, |acc, elt| max(acc, elt.1 + 1));

    for i in left..right {
        for j in bottom..top {
            if pos.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn main() {
    let mut positions = HashSet::new();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut row = 0;
    while !input.is_empty() {
        input = input.strip_suffix("\r\n").unwrap().to_string();
        for (i, c) in input.chars().enumerate() {
            if c == '#' {
                positions.insert((row, i as i32));
            }
        }
        row += 1;
        input.clear();
        stdin().read_line(&mut input).unwrap();
    }

    let mut move_order = MoveOrder::new();

    for i in 0..1000 {
        println!("start of turn {}", i);
        let mut turn: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        let mut invalidate: HashSet<(i32, i32)> = HashSet::new();

        for elf in positions.iter() {
            if alone(elf.clone(), &positions) {
                continue;
            }
            let order = move_order.get_next();

            for direction in order {
                match direction {
                    Direction::NORTH => {
                        if empty(elf.clone(), &positions, vec![(-1, 0), (-1, 1), (-1, -1)]) {
                            if let Some(_) = turn.insert((elf.0 - 1, elf.1), elf.clone()) {
                                invalidate.insert((elf.0 - 1, elf.1));
                            }
                            break;
                        }
                    }
                    Direction::SOUTH => {
                        if empty(elf.clone(), &positions, vec![(1, 0), (1, 1), (1, -1)]) {
                            if let Some(_) = turn.insert((elf.0 + 1, elf.1), elf.clone()) {
                                invalidate.insert((elf.0 + 1, elf.1));
                            }
                            break;
                        }
                    }
                    Direction::EAST => {
                        if empty(elf.clone(), &positions, vec![(-1, 1), (1, 1), (0, 1)]) {
                            if let Some(_) = turn.insert((elf.0, elf.1 + 1), elf.clone()) {
                                invalidate.insert((elf.0, elf.1 + 1));
                            }
                            break;
                        }
                    }
                    Direction::WEST => {
                        if empty(elf.clone(), &positions, vec![(0, -1), (1, -1), (-1, -1)]) {
                            if let Some(_) = turn.insert((elf.0, elf.1 - 1), elf.clone()) {
                                invalidate.insert((elf.0, elf.1 - 1));
                            }
                            break;
                        }
                    }
                }
            }
        }
        if turn.is_empty() {
            break;
        }
        for (dest, source) in turn.iter() {
            if invalidate.contains(dest) {
                continue;
            }
            positions.insert(dest.clone());
            positions.remove(source);
        }
        move_order.next();
    }
    print_board(&positions);
    let left = positions.iter().fold(i32::MAX, |acc, elt| min(acc, elt.0));
    let right = positions
        .iter()
        .fold(i32::MIN, |acc, elt| max(acc, elt.0 + 1));
    let bottom = positions.iter().fold(i32::MAX, |acc, elt| min(acc, elt.1));
    let top = positions
        .iter()
        .fold(i32::MIN, |acc, elt| max(acc, elt.1 + 1));

    println!("{} {} {} {}", left, right, bottom, top);
    let mut empty = 0;
    for i in left..right {
        for j in bottom..top {
            if !positions.contains(&(i, j)) {
                empty += 1;
            }
        }
    }

    println!("{}", empty);
}
