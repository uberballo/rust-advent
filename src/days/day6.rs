use crate::helpers;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn solve(input: String) {
    let data = helpers::file::to_combined_list(&input);
    part1(&data);
    part2(&input);
}

fn combine_to_one(input: &str) -> String {
    input.replace("\n", "")
}

fn group_data(input: &String) -> Vec<Vec<HashSet<char>>> {
    input
        .split("\n\n")
        .map(|x| x.lines().map(|i| HashSet::from_iter(i.chars())).collect())
        .collect::<Vec<Vec<HashSet<char>>>>()
}

fn count_votes_unique(input: &Vec<&str>) -> Vec<HashSet<char>> {
    input
        .iter()
        .map(|x| HashSet::from_iter(combine_to_one(x).chars()))
        .collect::<Vec<HashSet<char>>>()
}

fn part1(input: &Vec<&str>) {
    let res = count_votes_unique(input);
    let sum: usize = res.iter().map(|x| x.len()).sum();
    println!("{}", sum);
}
fn intersect_sets(sets: &[HashSet<char>]) -> HashSet<char> {
    sets.iter().fold(sets[0].clone(), |s1, s2| {
        s1.intersection(&s2).copied().collect()
    })
}

fn part2(input: &String) {
    let res = group_data(input);
    let sum: usize = res.iter().map(|x| intersect_sets(x).len()).sum();
    println!("{}", sum)
}
