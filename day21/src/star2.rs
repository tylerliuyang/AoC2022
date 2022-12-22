use std::{collections::HashMap, io::stdin, sync::Mutex};

#[derive(Clone)]
struct Node {
    parents: (String, String),
    op: char,
}

pub fn main() {
    let mut link: HashMap<String, Mutex<Vec<String>>> = HashMap::new();
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut values: HashMap<String, (i64, i64, i64, bool)> = HashMap::new();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    while !input.is_empty() {
        input = input.strip_suffix("\r\n").unwrap().to_string();
        let sections = input.split(" ").collect::<Vec<&str>>();
        let name = sections[0].strip_suffix(":").unwrap().to_string();
        match sections.len() {
            2 => {
                if name == "humn" {
                    values.insert(name, (sections[1].parse().unwrap(), 0, 0, true));
                } else {
                    values.insert(name, (sections[1].parse().unwrap(), 0, 0, false));
                }
            }
            _ => {
                let parent1 = sections[1].to_string();
                let parent2 = sections[3].to_string();
                let op = sections[2].chars().nth(0).unwrap();

                let mut v = Vec::new();
                v.push(name.clone());
                if let Some(entry) = link.get(&parent1) {
                    entry.lock().unwrap().push(parent1.clone());
                } else {
                    link.insert(parent1.clone(), Mutex::new(v.clone()));
                }
                if let Some(entry) = link.get(&parent2) {
                    entry.lock().unwrap().push(parent2.clone());
                } else {
                    link.insert(parent2.clone(), Mutex::new(v.clone()));
                }

                nodes.insert(
                    name,
                    Node {
                        parents: (parent1, parent2),
                        op,
                    },
                );
            }
        }

        input.clear();
        stdin().read_line(&mut input).unwrap();
    }

    for name in values
        .keys()
        .map(|elt| elt.clone())
        .collect::<Vec<String>>()
    {
        for other in link.get(&name).unwrap().lock().unwrap().clone() {
            eval(other.clone(), &link, &mut values, &nodes);
        }
    }
    let (parent1, parent2) = nodes.get("root").unwrap().parents.clone();
    let mut desired_parent = String::new();
    if values.get(&parent1).unwrap().3 {
        desired_parent = parent2;
    } else {
        desired_parent = parent1;
    }
    println!(
        "{}",
        descent(
            "root".to_string(),
            &values,
            &nodes,
            values.get(&desired_parent).unwrap().0 * 2
        )
    );

    println!("{:?}", values.get("root").unwrap());
}

fn eval(
    name: String,
    link: &HashMap<String, Mutex<Vec<String>>>,
    values: &mut HashMap<String, (i64, i64, i64, bool)>,
    nodes: &HashMap<String, Node>,
) {
    let (parent1, parent2) = nodes.get(&name).unwrap().parents.clone();
    if !values.contains_key(&parent1) || !values.contains_key(&parent2) {
        return;
    }
    let val1 = values.get(&parent1).unwrap().0;
    let humn1 = values.get(&parent1).unwrap().3;
    let val2 = values.get(&parent2).unwrap().0;
    let humn2 = values.get(&parent2).unwrap().3;

    match nodes.get(&name).unwrap().op {
        '+' => {
            values.insert(name.clone(), (val1 + val2, val1, val2, humn1 || humn2));
        }
        '-' => {
            values.insert(name.clone(), (val1 - val2, val1, val2, humn1 || humn2));
        }
        '*' => {
            values.insert(name.clone(), (val1 * val2, val1, val2, humn1 || humn2));
        }
        '/' => {
            values.insert(name.clone(), (val1 / val2, val1, val2, humn1 || humn2));
        }
        _ => {
            panic!()
        }
    }
    for other in link
        .get(&name)
        .unwrap_or(&Mutex::new(Vec::new()))
        .lock()
        .unwrap()
        .clone()
    {
        eval(other.clone(), link, values, nodes);
    }
}

fn descent(
    name: String,
    values: &HashMap<String, (i64, i64, i64, bool)>,
    nodes: &HashMap<String, Node>,
    target: i64,
) -> i64 {
    if !nodes.contains_key(&name) {
        return target;
    }
    let (mut parent1, mut parent2) = nodes.get(&name).unwrap().parents.clone();
    let mut swapped = false;
    if !values.get(&parent1).unwrap().3 {
        swapped = true;
        let temp = parent1;
        parent1 = parent2;
        parent2 = temp;
    }
    let other_value = values.get(&parent2).unwrap().0;
    match nodes.get(&name).unwrap().op {
        '+' => {
            // a + b = c, c - b = a
            // b + a = c, c - b = a
            println!("{}", target - other_value);
            return descent(parent1, values, nodes, target - other_value);
        }
        '-' => {
            // a - b = c, c + b = a
            // b - a = c, b - c = a
            let desire;
            if swapped {
                desire = other_value - target;
            } else {
                desire = target + other_value;
            }
            println!("{}", desire);
            return descent(parent1, values, nodes, desire);
        }
        '*' => {
            // a * b = c, c / b = a
            // b * a = c, c / b = a
            println!("{}", target / other_value);
            return descent(parent1, values, nodes, target / other_value);
        }
        '/' => {
            // a / b = c, c * b = a
            // b / a = c, b / c = a
            let desire;
            if swapped {
                desire = other_value / target;
            } else {
                desire = target * other_value;
            }
            println!("{}", desire);
            return descent(parent1, values, nodes, desire);
        }
        _ => {
            panic!()
        }
    }

    // parent 1 should be the one with humn
}
