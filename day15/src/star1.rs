use std::{io::stdin, cmp::{min, max}, collections::{HashMap, HashSet}};

const Y: i32 = 2000000;

pub fn main() {
    let mut min_distance = i32::MAX;
    let mut max_distance = i32::MIN;
    
    let mut input = String::new();
    let mut signals_and_beacons = HashSet::new();
    let mut map = HashMap::new();
    stdin().read_line(&mut input).unwrap();
    while !input.is_empty() {
        input = input.strip_suffix("\r\n").unwrap().to_string();
        let sections = input.split(" ").collect::<Vec<&str>>();
        let x: i32 = sections.get(2).unwrap()[2..sections.get(2).unwrap().len()-1].parse().unwrap();
        let y: i32 = sections.get(3).unwrap()[2..sections.get(3).unwrap().len()-1].parse().unwrap();

        let beacon_x: i32 = sections.get(8).unwrap()[2..sections.get(8).unwrap().len()-1].parse().unwrap();
        let beacon_y: i32 = sections.get(9).unwrap()[2..sections.get(9).unwrap().len()].parse().unwrap();

        signals_and_beacons.insert((x, y));
        signals_and_beacons.insert((beacon_x, beacon_y));

        let distance = (x-beacon_x).abs() + (y-beacon_y).abs();
        map.insert((x,y), distance);
        min_distance = min(min_distance, x-distance);
        max_distance = max(max_distance, x+distance);

        input.clear();
        stdin().read_line(&mut input).unwrap();
    }

    let mut total = 0;
    for i in min_distance..max_distance {
        if signals_and_beacons.contains(&(i, Y)) {
            continue;
        }
        for (key, value) in map.iter() {
            if (i-key.0).abs() + (Y-key.1).abs() <= *value {
                total += 1;
                break;
            }
        }
    }
    println!("{}", total);
}

