use std::{collections::HashSet, io::stdin};

#[derive(Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

pub fn main() {
    let mut points = HashSet::new();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    while !input.is_empty() {
        input = input.strip_suffix("\r\n").unwrap().to_string();
        let sections = input.split(",").collect::<Vec<&str>>();
        points.insert(Point {
            x: sections.get(0).unwrap().parse().unwrap(),
            y: sections.get(1).unwrap().parse().unwrap(),
            z: sections.get(2).unwrap().parse().unwrap(),
        });

        input.clear();
        stdin().read_line(&mut input).unwrap();
    }

    let mut surface_area = 0;
    const DIRECTION: [(i32, i32, i32); 6] = [
        (0, 0, 1),
        (0, 0, -1),
        (0, 1, 0),
        (0, -1, 0),
        (1, 0, 0),
        (-1, 0, 0),
    ];

    for point in points.iter() {
        for direction in DIRECTION {
            if !points.contains(&Point {
                x: point.x + direction.0,
                y: point.y + direction.1,
                z: point.z + direction.2,
            }) {
                surface_area += 1;
            }
        }
    }
    println!("{}", surface_area);
}
