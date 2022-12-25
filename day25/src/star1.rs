use std::{cmp::max, io::stdin};

const LOOKUP: [i64; 20] = [
    2,
    12,
    62,
    312,
    1562,
    7812,
    39062,
    195312,
    976562,
    4882812,
    24414062,
    122070312,
    610351562,
    3051757812,
    15258789062,
    76293945312,
    381469726562,
    1907348632812,
    9536743164062,
    47683715820312,
];
fn parse_snafu(s: &str) -> i64 {
    let mut total = 0;
    for (i, c) in (s.chars().rev()).enumerate() {
        total += parse_char(c) * 5_i64.pow(i as u32);
    }
    total
}

fn parse_char(c: char) -> i64 {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => unreachable!(),
    }
}

fn encode_num(n: i64) -> char {
    match n {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        _ => unreachable!(),
    }
}

fn to_snafu(n: i64) -> Vec<(char, u32)> {
    let mut output = Vec::new();
    if n.abs() <= 2 {
        output.push((encode_num(n), 0));
        return output;
    }

    let iter = 1..u32::MAX;
    let mut new_iter = iter
        .map(|elt| vec![(elt, 1), (elt, 2), (elt, 0), (elt, -1), (elt, -2)])
        .flatten();

    let first_char = new_iter
        .find(|(elt, amount)| (n - (5_i64.pow(*elt) * amount)).abs() <= LOOKUP[*elt as usize - 1])
        .unwrap();

    output = to_snafu(n - (5_i64.pow(first_char.0) * first_char.1));

    output.push((encode_num(first_char.1), first_char.0));
    output
}

pub fn main() {
    let mut snafu = Vec::new();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    while !input.is_empty() {
        input = input.strip_suffix("\r\n").unwrap().to_string();
        snafu.push(input.clone());

        input.clear();
        stdin().read_line(&mut input).unwrap();
    }

    let mut total = 0;
    for elt in snafu {
        total += parse_snafu(&elt);
    }
    println!("{}", total);
    let output = to_snafu(total);
    let mut s = String::new();
    for i in 0..output
        .iter()
        .fold(0, |acc, elt| max(acc, elt.1 + 1))
        .clone()
    {
        if let Some((c, _)) = output.iter().find(|elt| elt.1 == i) {
            s.insert(0, *c);
        } else {
            s.insert(0, '0');
        }
    }
    println!("{}", s);
    println!("{}", parse_snafu(&s));
    // for i in 0..20 {
    //     println!("{}", to_snafu(total));
    // }
}
