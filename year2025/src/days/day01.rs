use std::fs;

pub fn solve(input_file: &str) {
    let input: String = fs::read_to_string(input_file).unwrap();
    let by_line: Vec<(&str, i16)> = input
        .lines()
        .map(|s: &str| s.split_at(1))
        .map(|t: (&str, &str)| (t.0, t.1.parse::<i16>().unwrap()))
        .collect();
    let by_line_copy: Vec<(&str, i16)> = by_line.clone();

    let part_one_combination: u16 = solve_part_one(by_line);
    println!("Part one combination: {part_one_combination}");

    let part_two_combination: u16 = solve_part_two(by_line_copy);
    println!("Part two combination: {part_two_combination}");
}

fn solve_part_one(lines: Vec<(&str, i16)>) -> u16 {
    let mut combination: u16 = 0;
    let mut current_position: i16 = 50;

    for line in lines {
        if line.0 == "L" {
            current_position -= line.1;
        } else {
            current_position += line.1;
        }        
        if current_position % 100 == 0 {
            combination += 1;
        }
    }
    combination
}

fn solve_part_two(lines: Vec<(&str, i16)>) -> u16 {    
    let mut combination: u16 = 0;
    let mut current_position: i16 = 50;
    let mut new_position: i16 = 50;

    for line in lines {
        let remainder: i16 = line.1 % 100;
        let full_rotations: u16 = ((line.1 - remainder) / 100).unsigned_abs();
        combination += full_rotations;
        if line.0 == "L" {
            new_position -= remainder;
        } else {
            new_position += remainder;
        }
        if new_position == 0 {
            combination += 1;
            current_position = 0;
        } else if current_position < 100 && new_position >= 100 { 
            combination += 1;
            new_position -= 100;
            current_position = new_position;
        } else if current_position > 0 && new_position < 0 {
            combination += 1;
            new_position += 100;
            current_position = new_position; 
        } else if current_position == 0 && new_position < 0 {
            new_position += 100;
            current_position = new_position; 
        } else {
            current_position = new_position;
        }
    }
    combination
}