use std::fs;

pub fn solve(input_file: &str) {
    let by_line: Vec<(&str, i16)> = fs::read_to_string(input_file)
        .unwrap()   
        .lines()
        .map(|l| String::from(l))
        .map(|s| s.split_at(1))
        .map(|t| (t.0, t.1.parse::<i16>().unwrap()))
        .collect();

    let day_one_combination = solve_day_one(by_line);
    println!("Day one combination: {}", day_one_combination);

    ()
}

fn solve_day_one(lines: Vec<(&str, i16)>) -> u16 {

    let mut combination = 0;
    let mut current_position = 50;

    for line in lines {
        if line.0 == "L" {
            current_position += -line.1;
        } else {
            current_position += line.1;
        }        
        if current_position % 100 == 0 {
            combination += 1;
        }
    }

    return combination;
}