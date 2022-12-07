use std::{io::{stdin, Read}, collections::{HashSet, LinkedList, HashMap}, process::exit, sync::Mutex, ops::Add};

use crate::stdinpushback::Pushback;


fn add_subdirectories(sizes: &mut HashMap<String, i32>, links: &mut HashMap<String, Mutex<Vec<String>>>, curr: &mut String) -> i32 {
    // println!("{:?} {} {}", links.get(curr), curr, sizes.get(curr).unwrap());
    if let Some(next) = links.remove(curr) {
        let values = next.lock().unwrap();
        if values.len() == 0 {
            return *sizes.get(curr).unwrap();
        }
        print!("{:?} {}", values.iter(), curr);
        for subdir in values.iter() {
            let output = add_subdirectories(sizes, links, &mut subdir.to_string());
            println!("{} {}", subdir, output);
            sizes.insert(curr.to_string(), sizes.get(curr).unwrap() + output);
        };
    }
    *sizes.get(curr).unwrap()
}

pub fn main() {
    let mut helper = Pushback {value: None};
    let mut sizes: HashMap<String, Mutex<i32>> = HashMap::new();
    let mut links: HashMap<String, Mutex<Vec<String>>> = HashMap::new();
    let mut list = LinkedList::new();

    loop {
        let mut input = String::new();
        match helper.get(&mut input) {
            0 => {
                break;
            }
            _ => {
                input = input.strip_suffix("\r\n").unwrap().to_string();
                if input.starts_with("$ cd ..") {
                    list.pop_back();
                } else if input.starts_with("$ cd") {
                    let output: Vec<&str> = input.split(" ").collect();  
                    list.push_back(output.get(2).unwrap().to_string());
                    sizes.insert(output.get(2).unwrap().to_string(), Mutex::new(0));
                    links.insert(output.get(2).unwrap().to_string(), Mutex::new(Vec::new()));
                } else if input.starts_with("$ ls"){
                    loop {
                        let mut buf = String::new();
                        stdin().read_line(&mut buf).unwrap();
                        
                        if buf.starts_with("$") | buf.is_empty() { 
                            helper.set(buf);
                            break;
                        } else if buf.starts_with("dir") { 
                            buf = buf.strip_suffix("\r\n").unwrap().to_string();
                            links.get(list.back().unwrap()) // gets vec 
                                .unwrap()
                                .lock() // gets lock for mutex
                                .unwrap()
                                .push(buf.split(" ") // gets the dir
                                    .collect::<Vec<&str>>()
                                    .get(1).unwrap()
                                    .to_string());
                            continue;
                        } else {
                            let output: Vec<&str> = buf.split(" ").collect();
                            let old_size = sizes.get(list.back().unwrap()).unwrap();
                            sizes.get(&list.back().unwrap().to_string()).replace(&Some(sizes.get(&list.back().unwrap().to_string()).unwrap().unwrap() + (output.get(0).unwrap().parse::<i32>().unwrap())));
                            // sizes.insert(list.back().unwrap().to_string(), old_size + output.get(0).unwrap().parse::<i32>().unwrap());
                        }
                    }


                }

            }
        } 
    }
    println!("{:?}", sizes);
    println!("{:?}", links);
    // add_subdirectories(&mut sizes, &mut links, &mut "/".to_string());
    // let mut sizes_under = 0;
    // sizes.iter().map(|c| {
    //     add_subdirectories(&mut sizes.clone(), &mut links, &mut c.0.to_string());
    // });
    // for elt in sizes.iter() {
    //     println!("{:?}", elt);
    //     if *elt.1 <= 100000 {
    //         sizes_under += elt.1;
    //     }
    // }
    // println!("the total size under 100000 is: {}", sizes_under);
}

