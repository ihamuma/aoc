use std::fs;

pub fn solve(input_file: & str) {
    let input_file: String = fs::read_to_string(input_file).unwrap();
    let ranges: Vec<Vec<u64>> = input_file.split(',').map(|r| r.split('-').map(|s| s.parse::<u64>().unwrap()).collect()).collect();

    let part_one_total: u64 = solve_part_one(&ranges);
    println!("Part one total: {}", part_one_total);

    let part_two_total: u64 = solve_part_two(&ranges);
    println!("Part two total: {}", part_one_total);
}

fn solve_part_one(id_ranges: &Vec<Vec<u64>>) -> u64 {
    let mut total: u64 = 0;
    for id_range in id_ranges {
        let first_id: u64 = id_range[0];
        let last_id: u64 = id_range[1];
        for id in first_id..=last_id {
            let string_id: String = id.to_string();
            let (id1, id2) = string_id.split_at(string_id.len()/2);
            if id1 == id2 {
                total += id;
            }
        }
    }
    return total;
}

fn solve_part_two(id_ranges: &Vec<Vec<u64>>) -> u64 {
    /* TODO options:
    1. for each id in each range, get length. For n in 2..=length/2, check if length % n = 0.
        if true, split id into n equal chunks. if all equal, total += id and break
        NB: would .windows(length/n) work here?
    2. make some crazy regex that checks for repeating digits and series of digits until id.len()/2 for each id in each range.
        if repeating digits -> total += id and break   */
    return 64;
}
