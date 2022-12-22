use std::io::stdin;

pub fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut values: Vec<i64> = Vec::new();

    while !input.is_empty() {
        input = input.strip_suffix("\r\n").unwrap().to_string();

        values.push(input.parse::<i64>().unwrap() * 811589153);
        input.clear();
        stdin().read_line(&mut input).unwrap();
    }

    let mut indicies: Vec<usize> = Vec::new();

    for i in 0..values.len() {
        indicies.push(i);
    }

    for _ in 0..10 {
        for i in 0..values.len() {
            let to_move = values.get(i).unwrap();
            let place = indicies.iter().position(|elt| *elt == i).unwrap();
            indicies.remove(place);

            let mut new_index: i64 = <usize as TryInto<i64>>::try_into(place).unwrap() + to_move;
            new_index = new_index.rem_euclid((values.len() - 1) as i64);

            indicies.insert(new_index.try_into().unwrap(), i);
        }
    }

    let zero_i = values.iter().position(|elt| *elt == 0).unwrap();
    let position = indicies.iter().position(|elt| *elt == zero_i).unwrap();
    println!(
        "{}",
        values
            .get(*indicies.get((1000 + position) % values.len()).unwrap())
            .unwrap()
            + values
                .get(*indicies.get((2000 + position) % values.len()).unwrap())
                .unwrap()
            + values
                .get(*indicies.get((3000 + position) % values.len()).unwrap())
                .unwrap()
    );
}
