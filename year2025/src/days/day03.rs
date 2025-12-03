use std::fs;

pub fn solve(file_path: &str) {
    let input_file: String = fs::read_to_string(file_path).unwrap();
    let mut battery_banks: Vec<Vec<u32>> = input_file
    .lines()
    .map(|bb|bb.chars()
        .map(|b|b.to_digit(10).unwrap()).collect())
    .collect();
    println!("{battery_banks:?}");

    let part_one_joltage: u32 = solve_part_one(&mut battery_banks);
    println!("Part one total joltage: {part_one_joltage}");
}

fn solve_part_one(battery_banks: &mut [Vec<u32>]) -> u32 {
    let mut total_joltage: u32 = 0;
    for battery_bank in battery_banks {
        let (idx_max,_max) = battery_bank.iter().enumerate().max_by_key(|(_idx, &val)| val).unwrap();
        let first_max_battery: u32 = battery_bank.remove(idx_max);
        let (idx_second_max_battery,second_max_battery) = battery_bank.iter().enumerate().max_by_key(|(_idx, &val)| val).unwrap();
        if idx_max <= idx_second_max_battery {
            total_joltage += first_max_battery * 10 + second_max_battery;
            println!("{}", first_max_battery * 10 + second_max_battery);
        } else {
            total_joltage += second_max_battery * 10 + first_max_battery;
            println!("{}", second_max_battery * 10 + first_max_battery);
        }
    }
    total_joltage
}