use std::fs;

pub fn solve(input_file: &str) {
    let input: String = fs::read_to_string(input_file).unwrap();
    let by_line: Vec<(&str, i16)> = input
        .lines()
        .map(|s: &str| s.split_at(1))
        .map(|t: (&str, &str)| (t.0, t.1.parse::<i16>().unwrap()))
        .collect();

    let part_one_combination: u16 = solve_day_one(by_line);
    println!("Part one combination: {}", part_one_combination);
}

fn solve_day_one(lines: Vec<(&str, i16)>) -> u16 {

    let mut combination: u16 = 0;
    let mut current_position: i16 = 50;

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

fn solve_day_two(lines: Vec<(&str, i16) -> u16 {
    println("{lines}");
    
    // TODO: For each rotation, do % 100
    // Reduce the remainder from the rotation and / 100 = full rotations over 0 for numbers >100
    // Add / reduce remainder from current position - how to check if passes 0?
    
    return 16
}