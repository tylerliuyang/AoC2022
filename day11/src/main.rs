#![feature(default_free_fn)]
mod star1;
mod star2;
mod monkey1;
mod monkey2;
pub const MONKEY_COUNT: usize = 8;

fn main() {
    star2::main();
}
