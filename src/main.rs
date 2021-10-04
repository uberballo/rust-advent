#![allow(dead_code)]
mod days;
mod helpers;

fn main() {
    let data = helpers::file::read_file("data.txt");
    days::day6::solve(data);
}
