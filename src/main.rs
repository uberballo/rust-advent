#![allow(dead_code)]
mod days;
mod helpers;

fn main() {
    let data = helpers::file::read_file("data.txt");
    let input = helpers::file::to_list(&data);
    days::day4::solve(input);
}
