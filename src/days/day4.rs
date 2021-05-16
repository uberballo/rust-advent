use regex::Regex;

const RE_BYR: &str = r"";
const RE_LINE: &str = r"([a-z]{3}):(.*?)(?:\s|$)";
const REQ_FIELDS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

pub fn solve(input: Vec<&str>) {
    part1(&input);
    part2();
}

fn check_fields(line: &str) -> bool {
    let n = REQ_FIELDS
        .iter()
        .filter(|field| line.contains(field as &str))
        .count();
    n == 7
}

fn part1(lines: &Vec<&str>) {
    let mut sum = 0;
    let mut mut_line: String = "".to_string();

    for line in lines {
        if line.is_empty() {
            if check_fields(&mut_line) {
                sum += 1;
            }
            mut_line = "".to_string();
        }

        mut_line.push_str(line);
    }

    if check_fields(&mut_line) {
        sum += 1;
    }

    println!("summa: {}", sum);
}

fn part2() {
    println!("Not implemented");
}
