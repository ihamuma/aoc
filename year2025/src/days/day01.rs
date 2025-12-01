use std::fs;

pub fn solve(input_file: &str) {
    let by_line: Vec<String> = fs::read_to_string(input_file)
        .unwrap()   
        .lines()
        .map(|l| String::from(l))
        .collect();

    let day_one_combination = solve_day_one(by_line);
    println!("Day one combination: {}", day_one_combination);

    ()
}

fn solve_day_one(lines: Vec<String>) -> u16 {

    let mut combination = 0;
    let mut current_position = 50;

    for line in lines {
        let interpreted_line = interpret_line(&line);

        current_position += interpreted_line;
        
        if current_position % 100 == 0 {
            combination += 1;
        }
    }

    return combination;
}

fn interpret_line(line: &str) -> i16 {
    let (direction, to_turn): (&str, &str) = line.split_at(1);
    if direction == "L" {
        return -to_turn.parse::<i16>().unwrap();
    } else {
        return to_turn.parse::<i16>().unwrap()
    }
}