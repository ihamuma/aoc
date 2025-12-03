use std::fs;

pub fn solve(input_file: & str) {
    let input_file: String = fs::read_to_string(input_file).unwrap();
    let ranges: Vec<Vec<u64>> = input_file.split(',').map(|r| r.split('-').map(|s| s.parse::<u64>().unwrap()).collect()).collect();

    let part_one_total = solve_part_one(&ranges);
    println!("Part one total: {}", part_one_total)
}

fn solve_part_one(id_ranges: &Vec<Vec<u64>>) -> u64 {
    let mut total = 0;
    for id_range in id_ranges {
        let first_id = id_range[0];
        let last_id = id_range[1];
        for id in first_id..=last_id {
            let string_id = id.to_string();
            let (id1, id2) = string_id.split_at(string_id.len()/2);
            if id1 == id2 {
                total += id;
            }
        }
    }
    return total;
}