use std::{io::stdin, default::default, sync::Mutex};

use crate::monkey2::Monkey;
use crate::MONKEY_COUNT;

pub fn main() {
    let mut monkeys: [Mutex<Monkey>; MONKEY_COUNT] = default();
    for i in 0..MONKEY_COUNT {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.strip_suffix("\r\n").unwrap().to_string();
        let items = input.split(" ").collect::<Vec<&str>>()
            .iter().skip(4).map(|s| -> i64 {s.strip_suffix(',').unwrap_or(s).parse().unwrap()})
            .collect::<Vec<i64>>();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.strip_suffix("\r\n").unwrap().to_string();

        let arr = input.split(" ").collect::<Vec<&str>>();
        let mut op = arr.get(6).unwrap().chars().nth(0).unwrap();
        let other = arr.get(7).unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.strip_suffix("\r\n").unwrap().to_string();
        let test: i64 = input.split(" ").collect::<Vec<&str>>().get(5).unwrap().parse().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.strip_suffix("\r\n").unwrap().to_string();
        let true_monkey: i64 = input.split(" ").collect::<Vec<&str>>().get(9).unwrap().parse().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.strip_suffix("\r\n").unwrap().to_string();
        let false_monkey: i64 = input.split(" ").collect::<Vec<&str>>().get(9).unwrap().parse().unwrap();

        stdin().read_line(&mut input).unwrap();

        let mut value = -1;
        match *other {
            "old" => {op = '^'}
            _ => {value = other.parse().unwrap()}
        }
        monkeys[i] = Mutex::new(Monkey {items: items, operation: op, false_monkey: false_monkey, true_monkey: true_monkey, test: test, value: value, inspected: 0});
    }
    for i in 0..10000 {
        for j in 0..MONKEY_COUNT {
            monkeys[j].lock().unwrap().throw(&monkeys);
        }
    }
    for i in 0..MONKEY_COUNT {
        println!("{}", monkeys[i].lock().unwrap().inspect());
    }
}

