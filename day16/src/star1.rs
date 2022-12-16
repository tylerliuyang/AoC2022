use std::{collections::{HashMap, HashSet}, cmp::{min, max}, io::stdin};

use priority_queue::DoublePriorityQueue;

#[derive(Eq, Hash, PartialEq, Debug)]
struct Node {
    name: String,
    value: i32,
    connected: Vec<String>
}
  
fn dijkstra<'a>(node: &'a Node, node_map: &'a HashMap<String, Node>) -> HashMap<&'a Node, i32> { 
let mut q: DoublePriorityQueue<&Node, i32> = DoublePriorityQueue::new();
let mut best: HashMap<&Node, i32> = HashMap::new();
let mut found: HashSet<&Node> = HashSet::new();

q.push(&node, 0);
best.insert(node, 0);

while let Some((node, _priority)) = q.pop_min() {
    if found.contains(node) {
    continue;
    }
    found.insert(node);

    let curr = *best.get(node).unwrap();

    for out in &node.connected {
    let other = node_map.get(out).unwrap();

    q.push(other, curr+1);

    if let Some(value) = best.get(other) {
        best.insert(other, min(*value, curr+1));
    } else {
        best.insert(other, curr+1);
    }
    }
}
best
}
  
fn find_max<'a>(node: &Node, minutes: i32, nodes: &mut Vec<&Node>, distances: &HashMap<(&Node, &Node), i32>) -> i32 {
    let mut values = vec![0];
    
    for other in nodes.clone() {
        if other.value == 0 {continue}
        let distance = distances.get(&(node, other)).unwrap();
        if minutes - distance < 1 {continue}
        nodes.retain(|x| *x != other);
        values.push((minutes - distance - 1) * other.value + find_max(other, minutes - distance - 1, nodes, distances));
        nodes.push(other);
    };

    return values.iter().fold(0, |acc, elt| {max(acc, *elt)});
}
  
pub fn main() {
    let mut nodes = HashMap::new();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    while !input.is_empty() {
        input = input.strip_suffix("\r\n").unwrap().to_string();
        
        let sections: Vec<&str> = input.split(" ").collect();
        let name = sections.get(1).unwrap().to_string();
        let value = sections.get(4).unwrap()[5..sections.get(4).unwrap().len()-1].parse().unwrap();

        let mut connected = Vec::new();
        for i in 9..sections.len() {
            connected.push(sections.get(i).unwrap()
                .strip_suffix(",").unwrap_or(sections.get(i).unwrap()).to_string()
            );
        }

        nodes.insert(name.clone(), Node {name, value, connected});

        input.clear();
        stdin().read_line(&mut input).unwrap();
    }

    let mut map: HashMap<(&Node, &Node), i32> = HashMap::new();

    for node in nodes.values() {
        for (other, best) in dijkstra(node, &nodes).iter() {
            map.insert((node, other), *best);
        }
    }

    let mut a = Vec::new();
    nodes.values().into_iter().for_each(|elt| {a.push(elt)});
    println!("searching...");
    println!("{}", find_max(nodes.get("AA").unwrap(), 30, &mut a, &map));

}