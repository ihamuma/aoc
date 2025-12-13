use std::fs;

pub fn solve(file_path: &str) {
    let input_file: String = fs::read_to_string(file_path).unwrap();
    let battery_banks: Vec<Vec<u32>> = input_file
    .lines()
    .map(|bb|bb.chars()
        .map(|b|b.to_digit(10).unwrap()).collect())
    .collect();

    let part_one_joltage: u32 = solve_part_one(&battery_banks);
    println!("Part one total joltage: {part_one_joltage}");
}

fn solve_part_one(battery_banks: &[Vec<u32>]) -> u32 {
    let mut total_joltage: u32 = 0;
    let battery_bank_length: usize = battery_banks[0].len();
    for battery_bank in battery_banks {
        let (idx_max, max) = battery_bank.iter().enumerate().max_by_key(|(_idx, &val)| val).unwrap();
        if idx_max == battery_bank_length-1 {
            let remaining_batteries: &[u32] = &battery_bank[..idx_max];
            let remaining_batteries_max: &u32 = remaining_batteries.iter().max().unwrap();  
            let max_joltage: u32 = remaining_batteries_max * 10 + max;
            total_joltage += max_joltage;
        } else if idx_max == 0 {
            let remaining_batteries: &[u32] = &battery_bank[idx_max+1..];
            let remaining_batteries_max: &u32 = remaining_batteries.iter().max().unwrap();  
            let max_joltage = max * 10 + remaining_batteries_max;
            total_joltage += max_joltage;
        } else {
            let first_slice_batteries: &[u32] = &battery_bank[..idx_max];
            let first_slice_max: &u32 = first_slice_batteries.iter().max().unwrap();
            let second_slice_batteries: &[u32] = &battery_bank[idx_max+1..];
            let second_slice_max: &u32 = second_slice_batteries.iter().max().unwrap();
            if first_slice_max == max {
                total_joltage += first_slice_max * 10 + max;
            } else {
                total_joltage += max * 10 + second_slice_max;
            }
        }
    }
    total_joltage
}