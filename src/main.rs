mod days;
mod helpers;

fn main() {
    let data = helpers::file::read_file("data.txt");
    let input = helpers::file::to_list(&data);
    let result = 2020;
    days::day2::solve(input);
}
