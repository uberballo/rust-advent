use regex::Regex;

const RE_LINE: &str = r"([a-z]{3}):(.*?)(?:\s|$)";
const RE_BYR: &str = r"(19[2-9][0-9]|200[0-2])";
const RE_IYR: &str = r"(20(1[0-9]|20))";
const RE_EYR: &str = r"(20(2[0-9]|30))";
const RE_HGT: &str = r"(1(([5-8][0-9])|(9[0-3]))cm)|((59|6[0-9]|7[0-6])in)";
const RE_HCL: &str = r"(#[0-9a-f]{6})";
const RE_ECL: &str = r"(amb|blu|brn|gry|grn|hzl|oth)";
const RE_PID: &str = r"^\d{9}$";
const REQ_FIELDS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

pub fn solve(input: Vec<&str>) {
    part1(&input);
    part2(&input);
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

fn part2(lines: &Vec<&str>) {
    let mut sum = 0;
    let mut count = 0;

    let re_line = Regex::new(RE_LINE).unwrap();
    let re_byr = Regex::new(RE_BYR).unwrap();
    let re_iyr = Regex::new(RE_IYR).unwrap();
    let re_eyr = Regex::new(RE_EYR).unwrap();
    let re_hgt = Regex::new(RE_HGT).unwrap();
    let re_hcl = Regex::new(RE_HCL).unwrap();
    let re_ecl = Regex::new(RE_ECL).unwrap();
    let re_pid = Regex::new(RE_PID).unwrap();

    for line in lines {
        if line.is_empty() {
            if count == 7 {
                sum += 1;
            }
            count = 0;
        }

        for capture in re_line.captures_iter(line) {
            let (key, value) = (
                capture.get(1).unwrap().as_str(),
                capture.get(2).unwrap().as_str(),
            );
            let re = match key {
                "byr" => &re_byr,
                "iyr" => &re_iyr,
                "eyr" => &re_eyr,
                "hgt" => &re_hgt,
                "hcl" => &re_hcl,
                "ecl" => &re_ecl,
                "pid" => &re_pid,
                "cid" => continue,
                _ => panic!("none"),
            };
            if re.is_match(value) {
                count += 1;
            }
        }
    }

    println!("summa: {}", sum);
}
