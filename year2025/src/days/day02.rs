use std::fs;

pub fn solve(input_file: & str) {
    let input: String = fs::read_to_string(input_file).unwrap();
    let ranges: Vec<&str> = input.split(',').collect(); //.map(|s| s.split('-'))
    println!("{:?}", ranges)
}