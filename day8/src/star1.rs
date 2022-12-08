use std::{io::{stdin}, sync::Mutex};


pub fn main() {
    let forest: Mutex<Vec<Mutex<Vec<usize>>>> = Mutex::new(Vec::new());
    let dup_forest: Mutex<Vec<Mutex<Vec<bool>>>> = Mutex::new(Vec::new());

    let mut buffer = String::new();
    while stdin().read_line(&mut buffer).unwrap() > 0 {};

    let mut lines = 0;
    buffer.chars().map(|c| {if c == '\n' {lines += 1;}}).for_each(drop);
    
    for i in 0..lines {
        forest.lock().unwrap().push(Mutex::new(Vec::new()));
        dup_forest.lock().unwrap().push(Mutex::new(Vec::new()));
    }

    let mut row = 0;
    for c in buffer.chars() {
        if c == '\r' || c == '\n' {row = 0; continue; }
        forest.lock().unwrap().get(row)
            .unwrap().lock().unwrap()
            .push(char::to_digit(c, 10).unwrap().try_into().unwrap());
        dup_forest.lock().unwrap().get(row)
            .unwrap().lock().unwrap()
            .push(false);
        row += 1;
    }
    

    // check top
    let rows = forest.lock().unwrap().len().clone();
    let columns = forest.lock().unwrap().get(0).unwrap().lock().unwrap().len().clone() ;
    for i in 0..rows {
        let mut max_height: i32 = -1;
        for j in 0..columns {

            let v = forest.lock().unwrap()
                .get(j).unwrap().lock().unwrap()
                .get(i).unwrap().clone(); 
            if <usize as TryInto<i32>>::try_into(v).unwrap() <= max_height {continue;}
            max_height = v.try_into().unwrap();
            dup_forest.lock().unwrap()
                .get(j).unwrap().lock().unwrap()[i] = true;
        }
    }

    for i in 0..columns {
        let mut max_height: i32 = -1;

        for j in 0..rows {
            let v = forest.lock().unwrap()
                .get(i).unwrap().lock().unwrap()
                .get(j).unwrap().clone(); 
            if <usize as TryInto<i32>>::try_into(v).unwrap() <= max_height {continue;}
            max_height = v.try_into().unwrap();
            dup_forest.lock().unwrap()
                .get(i).unwrap().lock().unwrap()[j] = true;
        }
    }

    for i in (0..rows).rev() {
        let mut max_height: i32 = -1;
        for j in (0..columns).rev() {

            let v = forest.lock().unwrap()
                .get(j).unwrap().lock().unwrap()
                .get(i).unwrap().clone(); 
            if <usize as TryInto<i32>>::try_into(v).unwrap() <= max_height {continue;}
            max_height = v.try_into().unwrap();
            dup_forest.lock().unwrap()
                .get(j).unwrap().lock().unwrap()[i] = true;
        }
    }

    for i in (0..columns).rev() {
        let mut max_height: i32 = -1;
        for j in (0..rows).rev() {
            let v = forest.lock().unwrap()
                .get(i).unwrap().lock().unwrap()
                .get(j).unwrap().clone(); 
            if <usize as TryInto<i32>>::try_into(v).unwrap() <= max_height {continue;}
            max_height = v.try_into().unwrap();
            dup_forest.lock().unwrap()
                .get(i).unwrap().lock().unwrap()[j] = true;
        }
    }
            
    for i in 0..5 {
        println!("{:?}", forest.lock().unwrap().get(i).unwrap().lock().unwrap());
    }
    for i in 0..5 {
        println!("{:?}", dup_forest.lock().unwrap().get(i).unwrap().lock().unwrap());
    }
    let mut counter = 0;
    dup_forest.lock().unwrap().iter().map(|vec| {
        vec.lock().unwrap().iter().map(|b| if *b {counter += 1}).for_each(drop);
    }).for_each(drop);
    println!("{}", counter);
}

