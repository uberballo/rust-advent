mod days;
mod helpers;

fn main() {
    let data = helpers::file::read_file("data.txt");
    let input = helpers::file::to_list(&data);
    days::day3::solve(input, 1, 3);
}
