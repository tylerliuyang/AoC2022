use std::{io::stdin, sync::Mutex, collections::{HashMap}};

#[derive(PartialEq)]
enum Direction {
    LEFT,
    RIGHT,
    DOWN,
    UP
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
struct Coordinate {
    x: usize,
    y: usize
}
impl Coordinate {
    pub fn get_coord(&self, direction: Direction) -> Coordinate {
        if self.x == 0 && direction == Direction::LEFT {
            return Coordinate {x: 1000, y: 1000}
        } if self.y == 0 && direction == Direction::UP {
            return Coordinate {x: 1000, y: 1000}
        }
        match direction {
            Direction::LEFT => {return Coordinate {x: self.x - 1, y:self.y }}
            Direction::RIGHT => {return Coordinate {x: self.x + 1, y:self.y }}
            Direction::DOWN => {return Coordinate {x: self.x, y:self.y + 1 }}
            Direction::UP => {return Coordinate {x: self.x, y: self.y - 1}}
        }
    }
}


fn backtrack(map: HashMap<Coordinate, u32>,
    checked: &Mutex<HashMap<Coordinate, u32>>,
    mut queue: Vec<Coordinate>)
    {
        while !queue.is_empty() {
        let coords = queue.remove(0);

        let steps = checked.lock().unwrap().get(&coords).unwrap().clone();

        let height: i32  = map.get(&coords).unwrap_or(&28).clone().try_into().unwrap();
        let right: i32 = map.get(&coords.get_coord(Direction::RIGHT)).unwrap_or(&28).clone().try_into().unwrap();
        let left: i32 = map.get(&coords.get_coord(Direction::LEFT)).unwrap_or(&28).clone().try_into().unwrap();
        let down: i32 = map.get(&coords.get_coord(Direction::DOWN)).unwrap_or(&28).clone().try_into().unwrap();
        let up: i32 = map.get(&coords.get_coord(Direction::UP)).unwrap_or(&28).clone().try_into().unwrap();
        
        if right - height <= 1 && !checked.lock().unwrap().contains_key(&coords.get_coord(Direction::RIGHT)) {
            queue.push(coords.get_coord(Direction::RIGHT));
            checked.lock().unwrap().insert(coords.get_coord(Direction::RIGHT), steps + 1);
        };
        if left - height <= 1 && !checked.lock().unwrap().contains_key(&coords.get_coord(Direction::LEFT)) {
            queue.push(coords.get_coord(Direction::LEFT));
            checked.lock().unwrap().insert(coords.get_coord(Direction::LEFT), steps + 1);
        };
        if up - height <= 1 && !checked.lock().unwrap().contains_key(&coords.get_coord(Direction::UP)) {
            queue.push(coords.get_coord(Direction::UP));
            checked.lock().unwrap().insert(coords.get_coord(Direction::UP), steps + 1);
        };
        if down - height <= 1 && !checked.lock().unwrap().contains_key(&coords.get_coord(Direction::DOWN)) {
            queue.push(coords.get_coord(Direction::DOWN));
            checked.lock().unwrap().insert(coords.get_coord(Direction::DOWN), steps + 1);
        };
    }
}


pub fn main() {
    let checked: Mutex<HashMap<Coordinate, u32>> = Mutex::new(HashMap::new());
    let mut to_check: Vec<Coordinate> = Vec::new();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut thingy: Vec<Mutex<Vec<u32>>> = Vec::new();
    let mut map: HashMap<Coordinate, u32> = HashMap::new();
    let mut target = Coordinate {x: 0, y: 0}; 
    let mut start = Coordinate {x: 0, y: 0};
    while !input.is_empty() {
        input = input.strip_suffix("\r\n").unwrap().to_string();

        thingy.push(Mutex::new(Vec::new()));
        for mut c in input.chars() {
            if c == 'E' {c = 'z'; target = Coordinate {x: thingy.len()-1, y: thingy.last().unwrap().lock().unwrap().len()}}
            if c == 'S' {c = 'a'; start = Coordinate {x: thingy.len()-1, y: thingy.last().unwrap().lock().unwrap().len()}}
            map.insert(Coordinate {x: thingy.len()-1, y: thingy.last().unwrap().lock().unwrap().len()}, char::to_digit(c, 36).unwrap()-10);
            thingy.last().unwrap().lock().unwrap().push(char::to_digit(c, 36).unwrap()-10);
        }
        input.clear();
        stdin().read_line(&mut input).unwrap();
    }
    
    let mut min = u32::MAX;
    for item in map.clone() {
        if item.1 == 0 {
            let checked: Mutex<HashMap<Coordinate, u32>> = Mutex::new(HashMap::new());
            let mut to_check: Vec<Coordinate> = Vec::new();
            checked.lock().unwrap().insert(item.0, 0);
            to_check.push(item.0);
            backtrack(map.clone(), &checked, to_check);
            let output = checked.lock().unwrap().get(&target).unwrap_or(&1000).clone();
            if output < min {
                min = output;
            }
        }
    }

    println!("{}", min);
}