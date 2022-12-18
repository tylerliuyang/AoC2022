use std::{
    cmp::{max, min},
    collections::HashSet,
    io::stdin,
};

#[derive(Eq, PartialEq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

const DIRECTION: [Point; 6] = [
    Point { x: 0, y: 0, z: 1 },
    Point { x: 0, y: 0, z: -1 },
    Point { x: 0, y: 1, z: 0 },
    Point { x: 0, y: -1, z: 0 },
    Point { x: 1, y: 0, z: 0 },
    Point { x: -1, y: 0, z: 0 },
];

fn bfs(
    bounding_box: &[(i32, i32, i32); 2],
    checked: &mut HashSet<Point>,
    point: Point,
    points: &HashSet<Point>,
) {
    let mut queue = Vec::new();
    queue.push(point);
    while let Some(point) = queue.pop() {
        for direction in DIRECTION {
            let new_point = point.add(&direction);

            if new_point.x > bounding_box[1].0
                || new_point.x < bounding_box[0].0
                || new_point.y > bounding_box[1].1
                || new_point.y < bounding_box[0].1
                || new_point.z > bounding_box[1].2
                || new_point.z < bounding_box[0].2
            {
                continue;
            }

            if points.contains(&new_point) {
                continue;
            }

            if !checked.insert(new_point.clone()) {
                continue;
            }

            queue.push(new_point);
        }
    }
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

    let mut min_x = i32::MAX;
    points
        .iter()
        .for_each(|point| min_x = min(min_x, point.x - 1));
    let mut max_x = i32::MIN;
    points
        .iter()
        .for_each(|point| max_x = max(max_x, point.x + 1));
    let mut min_y = i32::MAX;
    points
        .iter()
        .for_each(|point| min_y = min(min_y, point.y - 1));
    let mut max_y = i32::MIN;
    points
        .iter()
        .for_each(|point| max_y = max(max_y, point.y + 1));
    let mut min_z = i32::MAX;
    points
        .iter()
        .for_each(|point| min_z = min(min_z, point.z - 1));
    let mut max_z = i32::MIN;
    points
        .iter()
        .for_each(|point| max_z = max(max_z, point.z + 1));

    let bounding_box: [(i32, i32, i32); 2] = [(min_x, min_y, min_z), (max_x, max_y, max_z)];

    let mut checked = HashSet::new();

    bfs(
        &bounding_box,
        &mut checked,
        Point {
            x: min_x,
            y: min_y,
            z: min_z,
        },
        &points,
    );

    let mut surface_area = 0;
    for item in checked.iter() {
        for direction in DIRECTION {
            if points.contains(&item.add(&direction)) {
                surface_area += 1;
            }
        }
    }

    let mut other_area = 0;
    for point in points.iter() {
        for direction in DIRECTION {
            if checked.contains(&point.add(&direction)) {
                other_area += 1;
            }
        }
    }

    println!("{} {}", surface_area, other_area);
}
