pub fn solve(input: Vec<&str>) {
    part1(&input);
    part2(&input);
}

fn strip_values(line: &str) -> (&str, &str, &str) {
    let values = line.split(" ").collect::<Vec<&str>>();
    (values[0], values[1], values[2])
}

fn strip_min_max(line: &str) -> (i32, i32) {
    let values = line
        .split("-")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();
    (values[0], values[1])
}

fn trim_character_string(line: &str) -> String {
    line.replace(":", "")
}

fn check_line(line: &str, min: i32, max: i32, chara: &str) -> bool {
    let mut count = 0;
    for c in line.chars() {
        if c.to_string() == chara {
            count += 1;
        }
    }
    min <= count && max >= count
}

fn part1(list: &Vec<&str>) {
    let mut count = 0;
    for i in list.iter() {
        let tup = strip_values(i);
        let (min, max) = strip_min_max(tup.0);
        let chara = trim_character_string(tup.1);
        let line = tup.2;

        if check_line(line, min, max, &chara) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn check_line_2(line: &str, index: i32, val: &str) -> bool {
    line.chars()
        .nth(index as usize)
        .unwrap_or_default()
        .to_string()
        == val
}

fn checker(i: &str) -> bool {
    let tup = strip_values(i);
    let (min, max) = strip_min_max(tup.0);
    let chara = trim_character_string(tup.1);
    let line = tup.2;

    let pos1 = check_line_2(line, min - 1, &chara);
    let pos2 = check_line_2(line, max - 1, &chara);

    pos1 ^ pos2
}

fn part2(list: &Vec<&str>) {
    let sum = list
        .iter()
        .fold(0, |sum, i| if checker(i) { sum + 1 } else { sum });

    println!("{}", sum);
}
