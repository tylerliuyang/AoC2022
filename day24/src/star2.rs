use std::{
    cmp::max,
    collections::{HashMap, HashSet, VecDeque},
    io::stdin,
};

trait Item {
    fn get_pos(&self, columns: i64, rows: i64) -> (i64, i64);
    fn get_pos_raw(&self) -> (i64, i64);
    fn forward(&mut self);
    fn backtrack(&mut self);
    fn to_string(&self) -> char;
}

struct Blizzard {
    x: i64,
    y: i64,
    direction: Direction,
}

impl Blizzard {
    fn wait(&self) -> Blizzard {
        Blizzard {
            x: self.x + convert_enum(self.direction).0,
            y: self.y + convert_enum(self.direction).1,
            direction: self.direction,
        }
    }
}

impl Item for Blizzard {
    fn get_pos(&self, c: i64, r: i64) -> (i64, i64) {
        (
            (self.x - 1).rem_euclid(c - 1) + 1,
            (self.y - 1).rem_euclid(r - 1) + 1,
        )
    }
    fn get_pos_raw(&self) -> (i64, i64) {
        (self.x, self.y)
    }
    fn forward(&mut self) {
        self.x = self.wait().x;
        self.y = self.wait().y;
    }
    fn backtrack(&mut self) {
        self.x = self.x - convert_enum(self.direction).0;
        self.y = self.y - convert_enum(self.direction).1;
    }
    fn to_string(&self) -> char {
        match self.direction {
            Direction::LEFT => '<',
            Direction::RIGHT => '>',
            Direction::DOWN => 'v',
            Direction::UP => '^',
            Direction::NONE => {
                unreachable!()
            }
        }
    }
}

#[derive(Clone)]
struct Wall {
    x: i64,
    y: i64,
}

impl Item for Wall {
    fn get_pos(&self, _: i64, _: i64) -> (i64, i64) {
        (self.x, self.y)
    }
    fn get_pos_raw(&self) -> (i64, i64) {
        (self.x, self.y)
    }
    fn forward(&mut self) {}
    fn backtrack(&mut self) {}
    fn to_string(&self) -> char {
        '#'
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NONE,
}
fn convert_enum(d: Direction) -> (i64, i64) {
    match d {
        Direction::UP => (0, -1),
        Direction::DOWN => (0, 1),
        Direction::RIGHT => (1, 0),
        Direction::LEFT => (-1, 0),
        Direction::NONE => (0, 0),
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn forward_checked(&self, direction: Direction, columns: i64, rows: i64) -> Option<Position> {
        if self.forward(direction).x < 0 || self.forward(direction).y < 0 {
            return None;
        }
        if self.forward(direction).x > columns + 1 || self.forward(direction).y > rows + 1 {
            return None;
        }
        Some(self.forward(direction))
    }
    fn forward(&self, direction: Direction) -> Position {
        Position {
            x: self.x + convert_enum(direction).0,
            y: self.y + convert_enum(direction).1,
        }
    }

    fn get_pos(&self) -> (i64, i64) {
        (self.x, self.y)
    }
    fn distance(&self, other: &Position) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

pub fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut board: Vec<Box<dyn Item>> = Vec::new();

    let mut row = 0;
    while !input.is_empty() {
        input = input.strip_suffix("\r\n").unwrap().to_string();
        for (column, c) in input.chars().enumerate() {
            match c {
                '>' => board.push(Box::new(Blizzard {
                    x: column as i64,
                    y: row,
                    direction: Direction::RIGHT,
                })),
                'v' => board.push(Box::new(Blizzard {
                    x: column as i64,
                    y: row,
                    direction: Direction::DOWN,
                })),
                '<' => board.push(Box::new(Blizzard {
                    x: column as i64,
                    y: row,
                    direction: Direction::LEFT,
                })),
                '^' => board.push(Box::new(Blizzard {
                    x: column as i64,
                    y: row,
                    direction: Direction::UP,
                })),
                '#' => board.push(Box::new(Wall {
                    x: column as i64,
                    y: row,
                })),
                _ => {}
            }
        }

        row += 1;
        input.clear();
        stdin().read_line(&mut input).unwrap();
    }

    let columns = board
        .iter()
        .fold(0, |acc, elt| max(elt.get_pos_raw().0, acc));
    let rows = board
        .iter()
        .fold(0, |acc, elt| max(elt.get_pos_raw().1, acc));

    let pos: Position = Position { x: 1, y: 0 };
    let desired: Position = Position {
        x: columns - 1,
        y: rows,
    };
    let mut queue: VecDeque<(Position, usize)> = VecDeque::new();
    queue.push_back((pos, 0));
    let result1 = descent(&mut board, desired, queue, columns, rows);

    let mut queue: VecDeque<(Position, usize)> = VecDeque::new();
    queue.push_back((desired, 0));
    let result2 = descent(&mut board, pos, queue, columns, rows);

    let mut queue: VecDeque<(Position, usize)> = VecDeque::new();
    queue.push_back((pos, 0));
    let result3 = descent(&mut board, desired, queue, columns, rows);

    println!("{}", result1 + result2 + result3 + 3);
}

fn descent(
    board: &mut Vec<Box<dyn Item>>,
    desired: Position,
    mut queue: VecDeque<(Position, usize)>,
    columns: i64,
    rows: i64,
) -> usize {
    let mut visited_states = HashSet::new();

    let origin = queue.front().unwrap().0;

    let mut last_minute = usize::MAX;
    while let Some(pos_minute) = queue.pop_front() {
        // print_board(board, columns, rows, pos_minute.0);
        // println!("{:?} {:?}", pos_minute, queue);

        if pos_minute.1 != last_minute {
            print_board(board, columns, rows, pos_minute.0);
            println!("onto minute {} with {} states", pos_minute.1, queue.len());
            last_minute = pos_minute.1;

            for item in board.iter_mut() {
                item.forward();
            }
        }

        let mut area_of_interest = HashSet::new();
        area_of_interest.insert(Direction::RIGHT);
        area_of_interest.insert(Direction::LEFT);
        area_of_interest.insert(Direction::DOWN);
        area_of_interest.insert(Direction::UP);
        area_of_interest.insert(Direction::NONE);
        // removes illegal moves

        for item in board.iter() {
            if item.get_pos(columns, rows) == (pos_minute.0.forward(Direction::RIGHT).get_pos()) {
                area_of_interest.remove(&Direction::RIGHT);
            }
            if item.get_pos(columns, rows) == (pos_minute.0.forward(Direction::LEFT).get_pos()) {
                area_of_interest.remove(&Direction::LEFT);
            }
            if item.get_pos(columns, rows) == (pos_minute.0.forward(Direction::DOWN).get_pos()) {
                area_of_interest.remove(&Direction::DOWN);
            }
            if item.get_pos(columns, rows) == (pos_minute.0.forward(Direction::UP).get_pos()) {
                area_of_interest.remove(&Direction::UP);
            }
            if item.get_pos(columns, rows) == (pos_minute.0.forward(Direction::NONE).get_pos()) {
                area_of_interest.remove(&Direction::NONE);
            }
        }

        for direction in area_of_interest {
            if let Some(p) = pos_minute.0.forward_checked(direction, columns, rows) {
                if p == desired {
                    return pos_minute.1;
                }
                if p.distance(&origin) < (pos_minute.1 / 3) as i64 {
                    continue;
                }
                if visited_states.insert((p, pos_minute.1 + 1)) {
                    queue.push_back((p, pos_minute.1 + 1));
                }
            }
        }
    }

    0
}

fn print_board(board: &Vec<Box<dyn Item>>, columns: i64, rows: i64, pos: Position) {
    let mut b: HashMap<(i64, i64), (usize, usize)> = HashMap::new();

    for (i, item) in board.iter().enumerate() {
        if b.contains_key(&item.get_pos(columns, rows)) {
            b.insert(
                item.get_pos(columns, rows),
                (b.get(&item.get_pos(columns, rows)).unwrap().0 + 1, i),
            );
        } else {
            b.insert(item.get_pos(columns, rows), (1, i));
        }
    }

    for j in 0..rows + 1 {
        for i in 0..columns + 1 {
            if (i, j) == pos.get_pos() {
                print!("E");
                continue;
            }
            match b.get(&(i, j)) {
                Some((1, i)) => print!("{}", board.get(*i).unwrap().to_string()),
                Some((mins, _)) => print!("{}", mins),
                None => print!("."),
            }
        }
        println!();
    }
}
