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

const CUBESIZE: i64 = 50;

fn get_cube_pos(place: (i64, i64), direction: Direction) -> ((i64, i64), Direction) {
    let (i, j) = (place.0 / 50, place.1 / 50);
    match (i, j, direction) {
        (0, 1, Direction::RIGHT) => ((place.0, 100), Direction::RIGHT),
        (0, 1, Direction::DOWN) => ((50, place.1), Direction::DOWN),
        (0, 1, Direction::UP) => ((place.1 + 100, 0), Direction::RIGHT),
        (0, 1, Direction::LEFT) => ((149 - place.0, 0), Direction::RIGHT),

        (0, 2, Direction::RIGHT) => ((149 - place.0, 99), Direction::LEFT),
        (0, 2, Direction::DOWN) => ((place.1 - 50, 99), Direction::LEFT),
        (0, 2, Direction::UP) => ((199, place.1 - 100), Direction::UP),
        (0, 2, Direction::LEFT) => ((place.0, 99), Direction::LEFT),

        (1, 1, Direction::RIGHT) => ((49, 50 + place.0), Direction::UP),
        (1, 1, Direction::DOWN) => ((100, place.1), Direction::DOWN),
        (1, 1, Direction::UP) => ((49, place.1), Direction::UP),
        (1, 1, Direction::LEFT) => ((100, place.0 - 50), Direction::DOWN),

        (2, 0, Direction::RIGHT) => ((place.0, 50), Direction::RIGHT),
        (2, 0, Direction::DOWN) => ((150, place.1), Direction::DOWN),
        (2, 0, Direction::UP) => ((50 + place.1, 50), Direction::RIGHT),
        (2, 0, Direction::LEFT) => ((149 - place.0, 50), Direction::RIGHT),

        (2, 1, Direction::RIGHT) => ((149 - place.0, 149), Direction::LEFT),
        (2, 1, Direction::DOWN) => ((50 + place.1, 49), Direction::LEFT),
        (2, 1, Direction::UP) => ((99, place.1), Direction::UP),
        (2, 1, Direction::LEFT) => ((place.0, 49), Direction::LEFT),

        (3, 0, Direction::RIGHT) => ((149, place.0 - 100), Direction::UP),
        (3, 0, Direction::DOWN) => ((0, place.1 + 100), Direction::DOWN),
        (3, 0, Direction::UP) => ((149, place.1), Direction::UP),
        (3, 0, Direction::LEFT) => ((0, place.0 - 100), Direction::DOWN),

        (_, _, _) => {
            panic!("YOU'RE NOT SUPPOSED TO BE HERE!");
        }
    }
}

fn get_next(
    board: &HashMap<(i64, i64), char>,
    place: (i64, i64),
    mut direction: Direction,
) -> Option<((i64, i64), Direction)> {
    match direction {
        Direction::RIGHT => {
            let place_to_check = &mut (place.0, place.1 + 1);
            if (place.1 + 50) / CUBESIZE != (place_to_check.1 + 50) / CUBESIZE {
                (*place_to_check, direction) = get_cube_pos(place, Direction::RIGHT);
            }

            if *board.get(place_to_check).unwrap() == '#' {
                return None;
            }
            Some((*place_to_check, direction))
        }
        Direction::LEFT => {
            let place_to_check = &mut (place.0, place.1 - 1);
            if (place.1 + 50) / CUBESIZE != (place_to_check.1 + 50) / CUBESIZE {
                (*place_to_check, direction) = get_cube_pos(place, Direction::LEFT);
            }

            if *board.get(place_to_check).unwrap() == '#' {
                return None;
            }
            Some((*place_to_check, direction))
        }
        Direction::UP => {
            let place_to_check = &mut (place.0 - 1, place.1);
            if (place.0 + 50) / CUBESIZE != (place_to_check.0 + 50) / CUBESIZE {
                (*place_to_check, direction) = get_cube_pos(place, Direction::UP);
            }

            if *board.get(place_to_check).unwrap() == '#' {
                return None;
            }
            Some((*place_to_check, direction))
        }
        Direction::DOWN => {
            let place_to_check = &mut (place.0 + 1, place.1);
            if (place.0 + 50) / CUBESIZE != (place_to_check.0 + 50) / CUBESIZE {
                (*place_to_check, direction) = get_cube_pos(place, Direction::DOWN);
            }

            if *board.get(place_to_check).unwrap() == '#' {
                return None;
            }
            Some((*place_to_check, direction))
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

    for command in a {
        match command.as_str() {
            "R" => direction = direction.right(),
            "L" => direction = direction.left(),
            amount => {
                for _ in 0..amount.parse().unwrap() {
                    if let Some((next_pos, new_dir)) = get_next(&board, pos, direction.clone()) {
                        pos = next_pos;
                        direction = new_dir;
                    }
                }
            }
        }
    }
    println!("{:?} {:?}", pos, direction);
    println!("{}", 1000 * (pos.0 + 1) + 4 * (pos.1 + 1));
}
