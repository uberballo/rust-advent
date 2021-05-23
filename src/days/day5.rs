pub fn solve(input: Vec<&str>) {
    part1(&input);
    part2();
}

fn left_right(c: char, low: i32, high: i32) -> i32 {
    match c {
        'B' => low,
        'L' => low,
        _ => high,
    }
}

fn solve_line(mut line: Vec<char>, low: i32, high: i32) -> i32 {
    let c = line.pop().unwrap();
    let new = ((low as f64 + high as f64) / 2 as f64).floor() as i32;
    if line.is_empty() {
        return low;
    }
    match c {
        'F' => solve_line(line, low, new),
        'L' => solve_line(line, low, new),
        _ => solve_line(line, new + 1, high),
    }
}

fn part1(lines: &Vec<&str>) {
    let mut values: Vec<i32> = Vec::new();
    for line in lines {
        let first: &str = &line[..7];
        let last = &line[7..];

        let list_first: Vec<char> = first.chars().rev().collect();
        let list_last: Vec<char> = last.chars().rev().collect();
        let row = solve_line(list_first, 0, 127);
        let seat = solve_line(list_last, 0, 7);
        let res = (row * 8) + (seat + 1);
        values.push(res);
    }
    let max_values = values.iter().max().unwrap();
    println!("{}", max_values);
}

fn part2() {
    println!("Not implemented");
}
