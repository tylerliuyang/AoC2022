use std::sync::Mutex;
use crate::MONKEY_COUNT;

#[derive(Default, Debug, Clone)]
pub struct Monkey {
    pub items: Vec<i32>,
    pub operation: char,
    pub value: i32,
    pub test: i32,
    pub true_monkey: i32,
    pub false_monkey: i32,
    pub inspected: i32
}

impl Monkey {
    pub fn throw(&mut self, other: &[Mutex<Monkey>; MONKEY_COUNT]) {
        for (i, item) in (self.items).clone().iter().enumerate() {
            self.inspected += 1;
            match self.operation {
                '+' => {self.items[i] = (item + self.value)/3} // /3 for worry level decreasing
                '*' => {self.items[i] = (item * self.value)/3}
                '^' => {self.items[i] = (item * item)/3}
                a => {panic!("this is not the correct symbol, {}", a)}
            }
        }
        for item in &self.items {
            if item % self.test == 0 {
                other[<i32 as TryInto<usize>>::try_into(self.true_monkey).unwrap()].lock().unwrap().items.push(*item);
            } else {
                other[<i32 as TryInto<usize>>::try_into(self.false_monkey).unwrap()].lock().unwrap().items.push(*item);
            }
        }
        self.items.clear();
    }
    pub fn inspect(&self) -> i32 {
        return self.inspected;
    }
}