use std::error::Error;
use std::fs;
use std::str::FromStr;
mod test;

#[derive(Debug)]
struct Num {
    pub index: usize,
    pub value: i64,
    pub gen: i64,
}

fn main() -> Result<(), Box<dyn Error>> {
    test::main();
    // Read the contents of the file into a string
    let contents = fs::read_to_string("numbers.txt")?;

    // Split the string into a vector of lines
    let lines: Vec<&str> = contents.split('\n').collect();

    // Create a new instance of the Num struct for each line and collect them into a new vector
    let mut nums: Vec<Num> = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        let value = i64::from_str(line)?;
        let num = Num {
            index,
            value,
            gen: 0,
        };
        nums.push(num);
    }
    let length = nums.len();
    let mut gen: Vec<i64> = vec![0; length];

    // Set the index of each num to the index plus the value modulo the length of the nums array
    for num in nums.iter_mut() {
        num.index = (num.index as i64 + num.value) as usize % (length - 1);
    }

    nums.sort_by(|a, b| (a.index as i64 + a.gen).cmp(&(b.index as i64 + b.gen)));
    println!("Nums: {:?}", nums);

    Ok(())
}
