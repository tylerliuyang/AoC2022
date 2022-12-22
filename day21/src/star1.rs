use std::{collections::HashMap, io::stdin, sync::Mutex};

#[derive(Clone)]
struct Node {
    parents: (String, String),
    op: char,
}

pub fn main() {
    let mut link: HashMap<String, Mutex<Vec<String>>> = HashMap::new();
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut values: HashMap<String, i64> = HashMap::new();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    while !input.is_empty() {
        input = input.strip_suffix("\r\n").unwrap().to_string();
        let sections = input.split(" ").collect::<Vec<&str>>();
        let name = sections[0].strip_suffix(":").unwrap().to_string();
        match sections.len() {
            2 => {
                values.insert(name, sections[1].parse().unwrap());
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

    println!("{}", values.get("root").unwrap());
}

fn eval(
    name: String,
    link: &HashMap<String, Mutex<Vec<String>>>,
    values: &mut HashMap<String, i64>,
    nodes: &HashMap<String, Node>,
) {
    let (parent1, parent2) = nodes.get(&name).unwrap().parents.clone();
    if !values.contains_key(&parent1) || !values.contains_key(&parent2) {
        return;
    }
    let val1 = values.get(&parent1).unwrap();
    let val2 = values.get(&parent2).unwrap();
    match nodes.get(&name).unwrap().op {
        '+' => {
            values.insert(name.clone(), val1 + val2);
        }
        '-' => {
            values.insert(name.clone(), val1 - val2);
        }
        '*' => {
            values.insert(name.clone(), val1 * val2);
        }
        '/' => {
            values.insert(name.clone(), val1 / val2);
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
