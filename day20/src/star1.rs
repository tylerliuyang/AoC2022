use std::{io::stdin, ops::Rem};

pub fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut values: Vec<i32> = Vec::new();

    while !input.is_empty() {
        input = input.strip_suffix("\r\n").unwrap().to_string();

        values.push(input.parse().unwrap());
        input.clear();
        stdin().read_line(&mut input).unwrap();
    }

    let mut indicies: Vec<usize> = Vec::new();

    for i in 0..values.len() {
        indicies.push(i);
    }
    let new_values = values
        .iter()
        .map(|elt| {
            elt.rem_euclid(values.len().try_into().unwrap())
                .try_into()
                .unwrap()
        })
        .collect::<Vec<usize>>();

    for i in 0..values.len() {
        let to_move = new_values.get(i).unwrap();
        let place = indicies.iter().position(|elt| *elt == i).unwrap();
        indicies.remove(place);

        let mut new_index: usize = place + to_move;

        if new_index >= values.len() && *values.get(i).unwrap() >= 0 {
            new_index = (new_index + 1).rem_euclid(values.len());
        } else if *values.get(i).unwrap() < 0 {
            if new_index == 0 {
                new_index = new_index + values.len() - 1
            } else if new_index > place {
                new_index -= 1
            }
        }

        indicies.insert(new_index, i);
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
