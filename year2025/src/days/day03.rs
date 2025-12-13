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

    let part_two_joltage: u64 = solve_part_two(&battery_banks);
    println!("Part two total joltage: {part_two_joltage}");
}

fn solve_part_one(battery_banks: &[Vec<u32>]) -> u32 {
    let mut total_joltage: u32 = 0;
    let battery_bank_length: usize = battery_banks[0].len();
    for battery_bank in battery_banks {
        let max = battery_bank.iter().max().unwrap();
        let idx_max = battery_bank.iter().position(|b|b == max).unwrap();
        if idx_max == battery_bank_length-1 {
            let remaining_batteries: &[u32] = &battery_bank[..idx_max];
            let remaining_batteries_max: &u32 = remaining_batteries.iter().max().unwrap();  
            total_joltage += remaining_batteries_max * 10 + max;            
        } else {
            let second_slice_batteries: &[u32] = &battery_bank[idx_max+1..];
            let second_slice_max: &u32 = second_slice_batteries.iter().max().unwrap();
            total_joltage += max * 10 + second_slice_max;
        }
    }
    total_joltage
}

fn solve_part_two(battery_banks: &[Vec<u32>]) -> u64 {
    let mut total_joltage: u64 = 0;
    for battery_bank in battery_banks {
        let mut joltages: Vec<u32> = vec![];
        let mut idx_max: usize = 0;
        let mut battery_bank_slice = &battery_bank[..];
        while joltages.len() < 12 {
            println!("Initially: {joltages:?} \n {idx_max} \n {battery_bank_slice:?}");
            let max: &u32 = battery_bank_slice.iter().max().unwrap();
            idx_max += battery_bank.iter().position(|b|b == max).unwrap();
            if battery_bank_slice.len()-idx_max == 12-joltages.len() {
                battery_bank_slice.iter().for_each(|b: &u32|joltages.push(*b));
            } else {
                joltages.push(*max);
                battery_bank_slice = &battery_bank[idx_max + 1..];
            }
        }
        total_joltage += 1;
    }
    total_joltage
}