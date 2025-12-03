use std::fs;

pub fn solve(file_path: &str) {
    let input_file = fs::read_to_string(file_path).unwrap();
    println!("{input_file}");
}