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

    let mut indicies: Vec<i32> = Vec::new();

    for i in 0..values.len().try_into().unwrap() {
        indicies.push(i);
    }

    for i in 0..values.len().try_into().unwrap() {
        let mut position = indicies.iter().position(|elt| *elt == i).unwrap();
        let to_bubble = *values.get::<usize>(i.try_into().unwrap()).unwrap();

        for _ in 0..to_bubble.abs() {
            if to_bubble >= 0 {
                if position == values.len() - 1 {
                    indicies.remove(values.len() - 1);
                    position = 0;
                    indicies.insert(position, i);
                }
                indicies.swap(position, position + 1);
                position += 1;

                if position == values.len() - 1 {
                    indicies.remove(values.len() - 1);
                    position = 0;
                    indicies.insert(position, i);
                }
            } else if to_bubble < 0 {
                if position == 0 {
                    indicies.remove(0);
                    position = values.len() - 1;
                    indicies.insert(values.len() - 1, i);
                }
                indicies.swap(position, position - 1);
                position -= 1;
                if position == 0 {
                    indicies.remove(0);
                    position = values.len() - 1;
                    indicies.insert(values.len() - 1, i);
                }
            }
        }
    }
    let zero_i = values.iter().position(|elt| *elt == 0).unwrap();
    let position = indicies
        .iter()
        .position(|elt| *elt == zero_i.try_into().unwrap())
        .unwrap();
    println!(
        "{} {} {}",
        values
            .get(
                <i32 as TryInto<usize>>::try_into(
                    *indicies.get((1000 + position) % values.len()).unwrap()
                )
                .unwrap()
            )
            .unwrap(),
        values
            .get(
                <i32 as TryInto<usize>>::try_into(
                    *indicies.get((2000 + position) % values.len()).unwrap()
                )
                .unwrap()
            )
            .unwrap(),
        values
            .get(
                <i32 as TryInto<usize>>::try_into(
                    *indicies.get((3000 + position) % values.len()).unwrap()
                )
                .unwrap()
            )
            .unwrap()
    );
}
