use std::collections::BTreeMap;

pub fn solve(input: Vec<&str>, result: i32) {
    let number: Vec<i32> = input.iter().map(|x| x.parse().unwrap()).collect();
    part1(&number, result);
    part2(&number, result);
}

fn part1(list: &Vec<i32>, result: i32) {
    for i in list.iter() {
        for j in list.iter() {
            if (i + j) == result {
                println!("{} + {} = {}", i, j, result);
                return;
            }
        }
    }
}

fn part2(list: &Vec<i32>, result: i32) {
    let mut map: BTreeMap<i32, i32> = BTreeMap::new();

    for i in list.iter() {
        map.insert(*i, 1);
        for j in list.iter() {
            let sum = i + j;
            let key = 2020 - sum;
            if map.contains_key(&key) {
                println!("{} + {} + {} = {}", i, j, key, result);
                return;
            }
        }
    }
}
