pub fn solve(input: Vec<&str>, down: i32, right: i32) {
    part1(&input, down, right);
    part2();
}

fn get_character(line: &str, index: i32) -> String {
    line.chars()
        .nth(index as usize)
        .unwrap_or_default()
        .to_string()
}

fn part1(lines: &Vec<&str>, down: i32, right: i32) {
    let max_x = lines[0].len();
    let max_y = lines.len();
    let mut x = 0;
    let mut y = 0;
    while y < max_y {}
    println!("Not implemented");
}

fn part2() {
    println!("Not implemented");
}
