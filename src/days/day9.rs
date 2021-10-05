use crate::helpers;
use std::collections::HashMap;

pub fn solve(input: String) {
    let mut lines: Vec<i64> = helpers::file::to_list(&input)
        .iter()
        .map(|x| x.parse().unwrap())
        .collect();
    part1(lines);
    part2();
}

fn count_combinations(input: Vec<i64>) -> HashMap<i64, i64> {
    let mut res: HashMap<i64, i64> = HashMap::new();
    for i in input.clone() {
        for j in input.clone() {
            let index = i + j;
            res.insert(index, 1);
        }
    }

    return res;
}

fn part1(input: Vec<i64>) {
    let mut queue: Vec<i64> = Vec::with_capacity(25);
    let max_len = input.len();

    for i in 0..26 {
        queue.push(*input.get(i).unwrap());
    }
    for i in 26..max_len {
        let value = input.get(i).unwrap();
        queue.push(*value);
        let combinations = count_combinations(queue.clone());
        if !combinations.contains_key(value) {
            println!("{} {}", i, value);
            break;
        }
        queue.drain(0..1);
    }
}

fn part2() {
    println!("Not implemented");
}
