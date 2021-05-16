pub fn solve(input: Vec<&str>, right: i32, down: i32) {
    part1(&input, right, down);
    part2(&input);
}

fn get_character(line: &str, index: i32) -> String {
    line.chars()
        .nth(index as usize)
        .unwrap_or_default()
        .to_string()
}

fn part1(lines: &Vec<&str>, right: i32, down: i32) -> i32 {
    let max_x = lines[0].len() as i32;
    let max_y = lines.len() as i32;
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    while y < max_y {
        if get_character(lines[y as usize], x % max_x) == "#" {
            count += 1;
        }
        y += down;
        x = x + right;
    }
    println!("{}", count);
    count
}

fn part2(lines: &Vec<&str>) {
    let a = part1(&lines, 1, 1) as i64;
    let b = part1(&lines, 3, 1) as i64;
    let c = part1(&lines, 5, 1) as i64;
    let d = part1(&lines, 7, 1) as i64;
    let e = part1(&lines, 1, 2) as i64;
    println!("{}", (a * b * c * d * e));
}
