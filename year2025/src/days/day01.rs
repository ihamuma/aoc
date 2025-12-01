use std::fs;

pub fn solve(input_file: &str) {
    let by_line: Vec<String> = fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(|l| String::from(l))
        .collect();

    println!("Combination: {}", by_line);
}


