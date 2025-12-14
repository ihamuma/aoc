use std::fs;

pub fn solve(file_path: &str) {
    let input_file: String = fs::read_to_string(file_path).unwrap();
    let battery_banks: Vec<Vec<u32>> = input_file
    .lines()
    .map(|bb|bb.chars()
        .map(|b: char|b.to_digit(10).unwrap()).collect())
    .collect();

    let part_one_joltage: u32 = solve_part_one(&battery_banks);
    println!("Part one total joltage: {part_one_joltage}");

    let part_two_joltage: u64 = solve_part_two(&battery_banks);
    println!("Part two total joltage: {part_two_joltage}");
}

fn solve_part_one(battery_banks: &[Vec<u32>]) -> u32 {
    let mut total_joltage: u32 = 0;
    let battery_bank_length: usize = battery_banks[0].len();
    for battery_bank in battery_banks {
        let max: &u32 = battery_bank.iter().max().unwrap();
        let low_bound: usize = battery_bank.iter().position(|b|b == max).unwrap();
        if low_bound == battery_bank_length-1 {
            let remaining_batteries: &[u32] = &battery_bank[..low_bound];
            let remaining_batteries_max: &u32 = remaining_batteries.iter().max().unwrap();  
            total_joltage += remaining_batteries_max * 10 + max;            
        } else {
            let second_slice_batteries: &[u32] = &battery_bank[low_bound+1..];
            let second_slice_max: &u32 = second_slice_batteries.iter().max().unwrap();
            total_joltage += max * 10 + second_slice_max;
        }
    }
    total_joltage
}

fn solve_part_two(battery_banks: &[Vec<u32>]) -> u64 {
    let mut all_joltages: Vec<u64> = vec![];
    let battery_bank_length: usize = battery_banks[0].len();
    for battery_bank in battery_banks {
        let mut joltages: Vec<u64> = vec![];
        let mut low_bound: usize = 0;
        let mut high_bound: usize = battery_bank_length-12+joltages.len();
        while joltages.len() < 12 {
            let battery_bank_slice: &[u32] = &battery_bank[low_bound..=high_bound];
            let max: &u32 = battery_bank_slice.iter().max().unwrap();
            joltages.push(*max as u64);
            let idx_max_in_slice: usize = battery_bank_slice.iter().position(|b: &u32|b == max).unwrap();
            low_bound += idx_max_in_slice + 1;
            high_bound = battery_bank_length-12+joltages.len();
        }
        let max_joltage: u64 = joltages.iter().fold(0, |acc: u64, elem: &u64| acc * 10 + elem);
        all_joltages.push(max_joltage);
    }
    let total_max_joltage: u64 = all_joltages.iter().sum();
    total_max_joltage
}