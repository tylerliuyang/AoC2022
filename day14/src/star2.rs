use std::{collections::HashSet, io::stdin};

type Stone = (usize, usize);

fn add_inbetween(stone1: (usize, usize),
 stone2: (usize, usize),
 occupied: &mut HashSet<Stone>) {
   for i in stone1.0..=stone2.0 {
     for j in stone1.1..=stone2.1 {
       occupied.insert((i, j));
     }
   }
   for i in stone2.0..=stone1.0 {
    for j in stone2.1..=stone1.1 {
      occupied.insert((i, j));
    }
  }
 }

fn filter_down(occupied: & HashSet<Stone>, stone: Stone, max_depth: usize) -> Stone {
  if max_depth-1 == stone.1 {
    return stone;
  }
  if !occupied.contains(&(stone.0, stone.1+1)) {
    return filter_down(occupied, (stone.0, stone.1+1), max_depth);
  }
  if !occupied.contains(&(stone.0-1, stone.1+1)) {
    return filter_down(occupied, (stone.0-1, stone.1+1), max_depth);
  }
  if !occupied.contains(&(stone.0+1, stone.1+1)) {
    return filter_down(occupied, (stone.0+1, stone.1+1), max_depth);
  }
  return stone;
}

const SOURCE: (usize, usize) = (500, 0);
pub fn main() {
  let mut max_depth = 0;
  let mut occupied: HashSet<Stone> = HashSet::new();
  let mut input = String::new();
  stdin().read_line(&mut input).unwrap();
  while !input.is_empty() {
    input = input.strip_suffix("\r\n").unwrap().to_string();
    let sections: Vec<&str> = input.split(" ").collect();
    for i in (2..sections.len()).step_by(2) {
      let curr_pos: Vec<&str> = sections[i].split(",").collect();
      let other_pos: Vec<&str> = sections[i-2].split(",").collect();
      if curr_pos.get(1).unwrap().parse::<usize>().unwrap() > max_depth {
        max_depth = curr_pos.get(1).unwrap().parse().unwrap();
      }
      if other_pos.get(1).unwrap().parse::<usize>().unwrap() > max_depth {
        max_depth = other_pos.get(1).unwrap().parse().unwrap();
      }
      add_inbetween(
        (other_pos.get(0).unwrap().parse().unwrap(), other_pos.get(1).unwrap().parse().unwrap()),
        (curr_pos.get(0).unwrap().parse().unwrap(), curr_pos.get(1).unwrap().parse().unwrap()),
        &mut occupied);
    }
    input.clear();
    stdin().read_line(&mut input).unwrap();
  };

  let mut i = 0;
  while !occupied.contains(&SOURCE) {
    let stone = filter_down(&occupied, SOURCE, max_depth + 2);
    occupied.insert(stone);
    i += 1;
  }
  println!("{}", i);
}