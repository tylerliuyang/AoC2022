use std::{collections::HashMap, io::stdin};

#[derive(Clone, Debug)]
enum Direction {
    LEFT,
    UP,
    RIGHT,
    DOWN,
}
impl Direction {
    fn right(self) -> Direction {
        match self {
            Direction::LEFT => Self::UP,
            Direction::UP => Self::RIGHT,
            Direction::RIGHT => Self::DOWN,
            Direction::DOWN => Self::LEFT,
        }
    }
    fn left(self) -> Direction {
        match self {
            Direction::LEFT => Self::DOWN,
            Direction::UP => Self::LEFT,
            Direction::RIGHT => Self::UP,
            Direction::DOWN => Self::RIGHT,
        }
    }
}

fn get_next(
    board: &HashMap<(i64, i64), char>,
    place: (i64, i64),
    direction: Direction,
) -> Option<(i64, i64)> {
    match direction {
        Direction::RIGHT => {
            let mut place_to_check = &(place.0, place.1 + 1);
            if !board.contains_key(&(place.0, place.1 + 1)) {
                place_to_check = *board
                    .keys()
                    .filter(|elt| elt.0 == place.0)
                    .collect::<Vec<&(i64, i64)>>()
                    .iter()
                    .min()
                    .unwrap();
            }
            if *board.get(place_to_check).unwrap() == '#' {
                return None;
            }
            Some(*place_to_check)
        }
        Direction::LEFT => {
            let mut place_to_check = &(place.0, place.1 - 1);
            if !board.contains_key(&(place.0, place.1 - 1)) {
                place_to_check = *board
                    .keys()
                    .filter(|elt| elt.0 == place.0)
                    .collect::<Vec<&(i64, i64)>>()
                    .iter()
                    .max()
                    .unwrap();
            }
            if *board.get(place_to_check).unwrap() == '#' {
                return None;
            }
            Some(*place_to_check)
        }
        Direction::UP => {
            let mut place_to_check = &(place.0 - 1, place.1);
            if !board.contains_key(&(place.0 - 1, place.1)) {
                place_to_check = *board
                    .keys()
                    .filter(|elt| elt.1 == place.1)
                    .collect::<Vec<&(i64, i64)>>()
                    .iter()
                    .max()
                    .unwrap();
            }
            if *board.get(place_to_check).unwrap() == '#' {
                return None;
            }
            Some(*place_to_check)
        }
        Direction::DOWN => {
            let mut place_to_check = &(place.0 + 1, place.1);
            if !board.contains_key(&(place.0 + 1, place.1)) {
                place_to_check = *board
                    .keys()
                    .filter(|elt| elt.1 == place.1)
                    .collect::<Vec<&(i64, i64)>>()
                    .iter()
                    .min()
                    .unwrap();
            }
            if *board.get(place_to_check).unwrap() == '#' {
                return None;
            }
            Some(*place_to_check)
        }
    }
}

pub fn main() {
    let mut board: HashMap<(i64, i64), char> = HashMap::new();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut row = 0;
    let mut pos = (-1, -1);
    let mut direction = Direction::RIGHT;

    while !input.is_empty() {
        for (column, c) in input.chars().enumerate() {
            if board.is_empty() {
                pos = (row, column as i64);
            }
            if c == '.' || c == '#' {
                board.insert((row, column as i64), c);
            }
        }

        row += 1;
        input.clear();
        stdin().read_line(&mut input).unwrap();
        input = input.strip_suffix("\r\n").unwrap().to_string();
    }

    let mut instructions = String::new();
    stdin().read_line(&mut instructions).unwrap();
    instructions = instructions.strip_suffix("\r\n").unwrap().to_string();

    let mut a = Vec::new();

    for part in instructions
        .split_inclusive("R")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.split_inclusive("L"))
        .flatten()
        .collect::<Vec<&str>>()
    {
        if part.chars().last().unwrap() == 'R' || part.chars().last().unwrap() == 'L' {
            a.push(part[0..part.len() - 1].to_string());
            a.push(part[part.len() - 1..part.len()].to_string());
        } else {
            a.push(part.to_string());
        }
    }
    println!("{:?}", a);

    for command in a {
        match command.as_str() {
            "R" => direction = direction.right(),
            "L" => direction = direction.left(),
            amount => {
                for _ in 0..amount.parse().unwrap() {
                    if let Some(next_pos) = get_next(&board, pos, direction.clone()) {
                        pos = next_pos;
                    }
                }
            }
        }
    }
    println!("{:?} {:?}", pos, direction);
    println!("{}", 1000 * (pos.0 + 1) + 4 * (pos.1 + 1));
}
