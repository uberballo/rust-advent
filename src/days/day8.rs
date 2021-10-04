use crate::helpers;
use std::collections::HashSet;

pub fn solve(input: String) {
    let data = create_tuple(helpers::file::to_list(&input));

    part1(&data);
    part2(&data);
}

fn create_tuple(input: Vec<&str>) -> Vec<(&str, i32)> {
    input
        .iter()
        .map(|x| {
            let split: Vec<&str> = x.split(' ').collect();
            let command: &str = split[0];
            let value: i32 = split[1].parse().unwrap();
            return (command, value);
        })
        .collect()
}
fn part1(input: &Vec<(&str, i32)>) {
    let (_, res) = solve_problem(input);
    println!("part1 {}", res);
}

fn solve_problem(input: &Vec<(&str, i32)>) -> (bool, i32) {
    let mut sum = 0;
    let mut index: i32 = 0;
    let mut visited: HashSet<i32> = HashSet::new();

    loop {
        let line = input[index as usize];
        let command: &str = line.0;
        let value: i32 = line.1;
        match command {
            "nop" => {
                index += 1;
            }
            "acc" => {
                sum += value;
                index += 1;
            }
            "jmp" => {
                index += value;
            }
            _ => {
                continue;
            }
        }
        if visited.contains(&index) {
            return (false, sum);
        }
        if index as usize == input.len() {
            return (true, sum);
        }
        visited.insert(index);
    }
}

fn part2(input: &Vec<(&str, i32)>) {
    for (pos, a) in input.iter().enumerate() {
        if a.0 == "nop" || a.0 == "jmp" {
            let mut temp = input.clone();
            match a.0 {
                "nop" => {
                    temp[pos].0 = "jmp";
                    let (finished, res) = solve_problem(&temp);
                    if finished {
                        println!("Finished: {}", res)
                    }
                }
                "jmp" => {
                    temp[pos].0 = "nop";
                    let (finished, res) = solve_problem(&temp);
                    if finished {
                        println!("Finished: {}", res)
                    }
                }
                _ => continue,
            }
        }
    }
}
