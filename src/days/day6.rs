use crate::helpers;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn solve(input: String) {
    let data = helpers::file::to_combined_list(&input);
    part1(data);
    part2();
}

fn combine_to_one(input: &str) -> String {
    input.replace("\n", "")
}

fn part1(input: Vec<&str>) {
    let res = input
        .iter()
        .map(|x| HashSet::from_iter(combine_to_one(x).chars()))
        .collect::<Vec<HashSet<char>>>();

    let sum: usize = res.iter().map(|x| x.len()).sum();
    println!("{}", sum);
}

fn part2() {
    println!("Not implemented");
}
