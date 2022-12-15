use std::{io::stdin, collections::{HashMap}, sync::Mutex, cmp::{min, max}};

fn combine_line(v: Vec<(i32, i32)>) -> Option<i32> {
    let mut i = -1;
    while i < 4000000 {
        let mut contained_values = Vec::new();

        v.iter().map(|value| {
            if i >= value.0 && i <= value.1 {
                contained_values.push(value.1);
            }
        }).for_each(drop);
        if contained_values.is_empty() {
            return Some(i)
        }
        i = contained_values.iter().fold(i, |acc, i| {max(acc, *i)});
        i += 1;
    }
    None
}

pub fn main() {
    
    let mut input = String::new();
    let mut occupied_lines: HashMap<i32, Mutex<Vec<(i32, i32)>>> = HashMap::new();
    stdin().read_line(&mut input).unwrap();
    while !input.is_empty() {
        input = input.strip_suffix("\r\n").unwrap().to_string();
        let sections = input.split(" ").collect::<Vec<&str>>();
        let x: i32 = sections.get(2).unwrap()[2..sections.get(2).unwrap().len()-1].parse().unwrap();
        let y: i32 = sections.get(3).unwrap()[2..sections.get(3).unwrap().len()-1].parse().unwrap();

        let beacon_x: i32 = sections.get(8).unwrap()[2..sections.get(8).unwrap().len()-1].parse().unwrap();
        let beacon_y: i32 = sections.get(9).unwrap()[2..sections.get(9).unwrap().len()].parse().unwrap();
        


        let distance = (x-beacon_x).abs() + (y-beacon_y).abs();

        for (i, y) in (y-distance..=y+distance).enumerate() {
            let mut j: i32 = i.try_into().unwrap();
            if <usize as TryInto<i32>>::try_into(i).unwrap() > distance {
                j = 2*distance - <usize as TryInto<i32>>::try_into(i).unwrap();
            }
            if let Some(v) = occupied_lines.get(&y) {
                v.lock().unwrap().push((x-j, x+j));
            } else {
                let mut v = Vec::new();
                v.push((x-j, x+j));
                occupied_lines.insert(y, Mutex::new(v));
            }
        }

        input.clear();
        stdin().read_line(&mut input).unwrap();
    }

    println!("ready for computation");

    for i in 0..4000000 {
        let output = combine_line(occupied_lines.get(&i).unwrap().lock().unwrap().to_vec());
        match output {
            Some(value) => println!("x: {}, y: {}, total: {}", value, i, <i32 as TryInto<i64>>::try_into(i).unwrap() + <i32 as TryInto<i64>>::try_into(value).unwrap() * 4000000),
            None => {}
        }

        if i % 10000 == 0 {
            println!("{} {:?}", i, output);
        }

    }
    
}

