use std::sync::Mutex;
use crate::MONKEY_COUNT;
const MOD: i64 = 2 * 7 * 13 * 19 * 11 * 5 * 3 * 17;
#[derive(Default, Debug, Clone)]
pub struct Monkey {
    pub items: Vec<i64>,
    pub operation: char,
    pub value: i64,
    pub test: i64,
    pub true_monkey: i64,
    pub false_monkey: i64,
    pub inspected: i64
}

impl Monkey {
    pub fn throw(&mut self, other: &[Mutex<Monkey>; MONKEY_COUNT]) {
        for (i, item) in (self.items).clone().iter().enumerate() {
            self.inspected += 1;
            match self.operation {
                '+' => {self.items[i] = (item + self.value) % MOD}
                '*' => {self.items[i] = (item % MOD * self.value % MOD) % MOD}
                '^' => {self.items[i] = (item % MOD * item % MOD) % MOD}
                a => {panic!("this is not the correct symbol, {}", a)}
            }
        }
        for item in &self.items {
            if item % self.test == 0 {
                other[<i64 as TryInto<usize>>::try_into(self.true_monkey).unwrap()].lock().unwrap().items.push(*item);
            } else {
                other[<i64 as TryInto<usize>>::try_into(self.false_monkey).unwrap()].lock().unwrap().items.push(*item);
            }
        }
        self.items.clear();
    }
    pub fn inspect(&self) -> i64 {
        return self.inspected;
    }
}