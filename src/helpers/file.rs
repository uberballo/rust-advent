use std::fs;

pub fn read_file(name: &str) -> String {
    let path = format!("src/{}", name);
    fs::read_to_string(path).expect("Something went wrong reading file")
}

pub fn to_list(input: &str) -> Vec<&str> {
    input.split("\n").collect::<Vec<&str>>()
}
