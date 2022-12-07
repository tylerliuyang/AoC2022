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
                        score += 1;
                        match input.pop().unwrap() {
                            'A' => {
                                score += 3;
                            }
                            'C' => {
                                score += 6;
                            }
                            _ => {
                                score += 0;
                            }
                        }
                    }
                    'Y' => {
                        score += 2;
                        match input.pop().unwrap() {
                            'A' => {
                                score += 6;
                            }
                            'B' => {
                                score += 3;
                            }
                            _ => {
                                score += 0;
                            }
                        }
                    }
                    'Z' => {
                        score += 3;
                        match input.pop().unwrap() {
                            'C' => {
                                score += 3;
                            }
                            'B' => {
                                score += 6;
                            }
                            _ => {
                                score += 0;
                            }
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

