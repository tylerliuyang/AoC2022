use std::{io::{stdin, Read}, collections::{HashSet, LinkedList, HashMap}, process::exit, sync::Mutex, ops::Add};

use crate::stdinpushback::Pushback;

fn get_curr_directory(list: &LinkedList<String>) -> String {
    let mut fold_buf = String::new();

    list.iter().map(|s: &String| {
        fold_buf.push_str(&s.to_string());
        fold_buf.push('/');
    }).for_each(drop);

    return fold_buf;
}

fn add_subdirectories(sizes: &HashMap<String, Mutex<i32>>, links: &mut HashMap<String, Mutex<Vec<String>>>, curr: &mut String) -> i32 {
    // println!("{:?} {} {}", links.get(curr), curr, sizes.get(curr).unwrap());
    if let Some(next) = links.remove(curr) {
        let values = next.lock().unwrap();
        if values.len() == 0 {
            return *sizes.get(curr).unwrap().lock().unwrap();
        }
        print!("{:?} {} ", values.iter(), curr);
        for subdir in values.iter() {
            let output = add_subdirectories(sizes, links, &mut subdir.to_string());
            println!("{} {}", subdir, output);
            let value = *sizes.get(curr).unwrap().lock().unwrap();
            *sizes.get(curr)
                .unwrap()
                .lock()
                .unwrap() = value.add(output);
        };
    }
    *sizes.get(curr).unwrap().lock().unwrap()
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
                    sizes.insert(get_curr_directory(&list), Mutex::new(0));
                    links.insert(get_curr_directory(&list), Mutex::new(Vec::new()));
                } else if input.starts_with("$ ls"){
                    loop {
                        let mut buf = String::new();
                        stdin().read_line(&mut buf).unwrap();
                        
                        if buf.starts_with("$") | buf.is_empty() { 
                            helper.set(buf);
                            break;
                        } else if buf.starts_with("dir") { 

                            buf = buf.strip_suffix("\r\n").unwrap().to_string();
                            let mut folder_name = buf.split(" ") // gets the dir
                                .collect::<Vec<&str>>()
                                .get(1).unwrap()
                                .to_string();
                            list.push_back(folder_name);
                            folder_name = get_curr_directory(&list);
                            list.pop_back();

                            links.get(&get_curr_directory(&list)) // gets vec 
                                .unwrap()
                                .lock() // gets lock for mutex
                                .unwrap()
                                .push(folder_name);

                            continue;

                        } else {
                            let output: Vec<&str> = buf.split(" ").collect();
                            let old_size = *sizes.get(&get_curr_directory(&list)).unwrap().lock().unwrap();
                            *sizes.get(&get_curr_directory(&list))
                                .unwrap()
                                .lock()
                                .unwrap() = old_size + output.get(0).unwrap().parse::<i32>().unwrap();
                        }
                    }


                }

            }
        } 
    }
    println!("{:?}", sizes);
    println!("{:?}", links);
    let mut sizes_under = 0;
    add_subdirectories(&sizes, &mut links, &mut "//".to_string());

    // sizes.iter().map(|c| {
    //     add_subdirectories(&sizes, &mut links, &mut c.0.to_string());
    // }).for_each(drop);

    for elt in sizes.iter() {
        println!("{:?}", elt);
        if *elt.1.lock().unwrap() <= 100000 {
            sizes_under = *elt.1.lock().unwrap() + sizes_under;
        }
    }
    println!("the total size under 100000 is: {}", sizes_under);
}

