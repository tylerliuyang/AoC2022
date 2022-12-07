use std::{io::stdin};

pub fn main() {
    let mut score = 0;
    loop {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                input = input.strip_suffix("\r\n").unwrap().to_string();
                input.remove(1);
                match input.pop().unwrap() {
                    'X' => {
                        score += 0;
                        match input.pop().unwrap() {
                            'A' => {
                                score += 3;
                            }
                            'B' => {
                                score += 1;
                            }
                            'C' => {
                                score += 2;
                            }
                            _ => {}
                        }
                    }
                    'Y' => {
                        score += 3;
                        match input.pop().unwrap() {
                            'A' => {
                                score += 1;
                            }
                            'B' => {
                                score += 2;
                            }
                            'C' => {
                                score += 3;
                            }
                            _ => {}
                        }
                    }
                    'Z' => {
                        score += 6;
                        match input.pop().unwrap() {
                            'A' => {
                                score += 2;
                            }
                            'B' => {
                                score += 3;
                            }
                            'C' => {
                                score += 1;
                            }
                            _ => {}
                        }
                    }
                    _ => {
                        panic!("you shouldn't be here!");
                    }
                }
            }
            Err(error) => {println!("error: {error}"); break;},
        } 
    }
    println!("the score is {}", score);
}

