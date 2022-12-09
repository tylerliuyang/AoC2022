use std::{io::{stdin}, sync::Mutex};

enum Direction { 
    LEFT,
    RIGHT,
    UP,
    DOWN
}

fn look(forest: &Mutex<Vec<Mutex<Vec<usize>>>>, i: usize, j: usize, direction: Direction) -> usize{
    let height = forest.lock().unwrap().get(i).unwrap().lock().unwrap().get(j).unwrap().clone();
    let mut trees_seen = 1;
    let mut k = 0;
    let mut l = 0;
    let rows = forest.lock().unwrap().len().clone();
    let columns = forest.lock().unwrap().get(0).unwrap().lock().unwrap().len().clone();
    match direction {
        Direction::LEFT => {if i == 0 {return 0}; if i == 1 {return 1}; k = i - 1; l = j;}
        Direction::RIGHT => {if i == columns-1 {return 0}; if i == columns-2 {return 1}; k = i + 1; l = j;}
        Direction::UP => {if j == 0 {return 0}; if j == 1 {return 1};  k = i; l = j -1;} 
        Direction::DOWN => {if j == columns-1 {return 0}; if j == columns-2 {return 1}; k = i; l = j + 1;}
    }
    let mut curr_height = forest.lock().unwrap().get(k).unwrap().lock().unwrap().get(l).unwrap().clone();
    // println!("before");
    while curr_height < height {
        // println!("after {} {}", k, l);
        match direction {
            Direction::LEFT => {k = k - 1;}
            Direction::RIGHT => {k = k + 1;}
            Direction::UP => {l = l - 1;} 
            Direction::DOWN => {l = l + 1;}
        }
        if k == 0 || k >= columns-1 || l == 0 || l >= rows-1 {return trees_seen + 1}
        trees_seen += 1;
        curr_height = forest.lock().unwrap().get(k).unwrap().lock().unwrap().get(l).unwrap().clone();
    }
    trees_seen
}

pub fn main() {
    let forest: Mutex<Vec<Mutex<Vec<usize>>>> = Mutex::new(Vec::new());

    let mut buffer = String::new();
    while stdin().read_line(&mut buffer).unwrap() > 0 {};

    let mut lines = 0;
    buffer.chars().map(|c| {if c == '\n' {lines += 1;}}).for_each(drop);
    
    for i in 0..lines {
        forest.lock().unwrap().push(Mutex::new(Vec::new()));
    }

    let mut row = 0;
    for c in buffer.chars() {
        if c == '\r' || c == '\n' {row = 0; continue; }
        forest.lock().unwrap().get(row)
            .unwrap().lock().unwrap()
            .push(char::to_digit(c, 10).unwrap().try_into().unwrap());
        row += 1;
    }


    

    // check top
    let rows = forest.lock().unwrap().len().clone();
    let columns = forest.lock().unwrap().get(0).unwrap().lock().unwrap().len().clone() ;
    let mut max_scenic = 0;
    for i in 0..rows {
        for j in 0..columns {
            let left = look(&forest, j, i, Direction::LEFT);
            let right = look(&forest, j, i, Direction::RIGHT);
            let up = look(&forest, j, i, Direction::UP);
            let down = look(&forest, j, i, Direction::DOWN);
            if left * right * up * down > max_scenic {
                max_scenic = left * right * up * down;
            }
        }
    }

            
    for i in 0..5 {
        println!("{:?}", forest.lock().unwrap().get(i).unwrap().lock().unwrap());
    }
    println!("{}", max_scenic);
}

