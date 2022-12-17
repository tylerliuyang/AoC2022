use std::{cmp::max, collections::HashSet, io::stdin};

#[derive(Clone, Debug)]
struct Shape {
    shape: HashSet<Point>,
    height: usize,
}
const TRILLION: usize = 1000000000000;

impl Shape {
    fn flip(&self) -> Shape {
        let mut shape = HashSet::new();
        for point in &self.shape {
            shape.insert(Point {
                x: point.x,
                y: self.height - point.y,
            });
        }
        Shape {
            shape,
            height: self.height,
        }
    }

    fn move_to(&self, direction: Direction, amount: usize) -> Shape {
        let mut shape = HashSet::new();
        for point in &self.shape {
            shape.insert(point.move_to(direction.clone(), amount));
        }
        Shape {
            shape,
            height: self.height,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, PartialEq, Eq)]
enum Direction {
    RIGHT,
    LEFT,
    DOWN,
    UP,
}

impl Point {
    fn move_to(&self, direction: Direction, amount: usize) -> Point {
        match direction {
            Direction::RIGHT => Point {
                x: self.x + amount,
                y: self.y,
            },
            Direction::LEFT => Point {
                x: self.x - amount,
                y: self.y,
            },
            Direction::UP => Point {
                x: self.x,
                y: self.y + amount,
            },
            Direction::DOWN => Point {
                x: self.x,
                y: self.y - amount,
            },
        }
    }
}

fn try_move(
    shape: &mut Shape,
    direction: Direction,
    amount: usize,
    board: &HashSet<Point>,
) -> bool {
    if direction == Direction::LEFT {
        if shape.shape.iter().any(|point| point.x == 0) {
            return false;
        }
    }
    if direction == Direction::DOWN {
        if shape.shape.iter().any(|point| point.y == 0) {
            return false;
        }
    }
    if direction == Direction::RIGHT {
        if shape.shape.iter().any(|point| point.x == 6) {
            return false;
        }
    }
    if shape
        .move_to(direction.clone(), amount)
        .shape
        .iter()
        .all(|point| !board.contains(point))
    {
        *shape = shape.move_to(direction, amount);
        return true;
    }
    false
}

pub fn main() {
    let mut shapes = Vec::new();
    let mut shape = HashSet::new();
    let mut jet_stream = String::new();
    let mut y = 0;
    loop {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(2) => {
                let elt = Shape {
                    shape: shape.clone(),
                    height: y - 1,
                }
                .flip();
                shapes.push(elt);
                shape = HashSet::new();
                y = 0;
            }

            Ok(_) => {
                input = input.strip_suffix("\r\n").unwrap().to_string();
                for (i, c) in input.chars().into_iter().enumerate() {
                    if c == '<' || c == '>' {
                        jet_stream = input.clone();
                        break;
                    }
                    if c == '#' {
                        shape.insert(Point { x: i, y: y });
                    }
                }
                y += 1;
            }
            Err(_) => {
                panic!();
            }
        }
    }

    let mut board = HashSet::new();
    let mut max_50455 = 0;
    let mut steps = 0;

    for j in 0..166 {
        let shape = shapes.get(j % 5).unwrap();
        let mut normalized_shape = shape
            .clone()
            .move_to(Direction::RIGHT, 2)
            .move_to(Direction::UP, max_50455 + 3);

        loop {
            match jet_stream.chars().nth(steps % jet_stream.len()).unwrap() {
                '<' => {
                    try_move(&mut normalized_shape, Direction::LEFT, 1, &board);
                }
                '>' => {
                    try_move(&mut normalized_shape, Direction::RIGHT, 1, &board);
                }
                _ => {}
            }
            steps += 1;

            if !try_move(&mut normalized_shape, Direction::DOWN, 1, &board) {
                normalized_shape.shape.iter().for_each(|point| {
                    max_50455 = max(point.y + 1, max_50455);
                    board.insert(point.clone());
                });
                break;
            }
        }
    }
    println!("done");

    let mut board = HashSet::new();
    let mut max_2702 = 0;
    let mut steps = 0;

    for j in 0..(1891) {
        let shape = shapes.get(j % 5).unwrap();
        let mut normalized_shape = shape
            .clone()
            .move_to(Direction::RIGHT, 2)
            .move_to(Direction::UP, max_2702 + 3);

        loop {
            match jet_stream.chars().nth(steps % jet_stream.len()).unwrap() {
                '<' => {
                    try_move(&mut normalized_shape, Direction::LEFT, 1, &board);
                }
                '>' => {
                    try_move(&mut normalized_shape, Direction::RIGHT, 1, &board);
                }
                _ => {}
            }
            steps += 1;

            if !try_move(&mut normalized_shape, Direction::DOWN, 1, &board) {
                normalized_shape.shape.iter().for_each(|point| {
                    max_2702 = max(point.y + 1, max_2702);
                    board.insert(point.clone());
                });
                break;
            }
        }
    }
    println!("done");

    let mut board = HashSet::new();
    let mut max_remaining = 0;
    let mut steps = 0;
    const A: usize = (TRILLION - 166) % (1725);
    for j in 0..(166 + A) {
        let shape = shapes.get(j % 5).unwrap();
        let mut normalized_shape = shape
            .clone()
            .move_to(Direction::RIGHT, 2)
            .move_to(Direction::UP, max_remaining + 3);

        loop {
            match jet_stream.chars().nth(steps % jet_stream.len()).unwrap() {
                '<' => {
                    try_move(&mut normalized_shape, Direction::LEFT, 1, &board);
                }
                '>' => {
                    try_move(&mut normalized_shape, Direction::RIGHT, 1, &board);
                }
                _ => {}
            }
            steps += 1;

            if !try_move(&mut normalized_shape, Direction::DOWN, 1, &board) {
                normalized_shape.shape.iter().for_each(|point| {
                    max_remaining = max(point.y + 1, max_remaining);
                    board.insert(point.clone());
                });
                break;
            }
        }
    }
    println!(
        "{} {} {}",
        max_2702 - max_50455,
        max_50455,
        max_remaining - max_50455
    );
    println!(
        "{}",
        max_50455 + (max_2702 - max_50455) * ((TRILLION - 166) / 1725) + max_remaining - max_50455
    )
}
