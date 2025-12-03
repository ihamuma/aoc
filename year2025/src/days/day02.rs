use std::fs;

pub fn solve(input_file: & str) {
    let input_file: String = fs::read_to_string(input_file).unwrap();
    let ranges: Vec<Vec<u64>> = input_file.split(',').map(|r| r.split('-').map(|s| s.parse::<u64>().unwrap()).collect()).collect();

    let part_one_total: u64 = solve_part_one(&ranges);
    println!("Part one total: {part_one_total}");

    let part_two_total: u64 = solve_part_two(&ranges);
    println!("Part two total: {part_two_total}");
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
    total
}

/* I'm fairly certain regexing for repeat digits and series would be faster and neater
    but this implementation only uses the standard library. Of course, far more efficient 
    ways than this are out there :D */ 
fn solve_part_two(id_ranges: &Vec<Vec<u64>>) -> u64 {
    let mut total: u64 = 0;
    for id_range in id_ranges {
        let first_id: u64 = id_range[0];
        let last_id: u64 = id_range[1];
        for id in first_id..=last_id {
            let id_string: Vec<char> = id.to_string().chars().collect();
            let id_length = id_string.len();
            let id_slice: &[char] = &id_string[..];
            for n in 1..=id_length/2 {
                if id_length.is_multiple_of(n) {
                    let mut id_chunks: Vec<&[char]> = id_slice.chunks(n).collect();
                    let first: &[char] = id_chunks.pop().unwrap();
                    let is_invalid = id_chunks.iter().all(|x: &&[char]|x == &first);
                    if is_invalid {
                        total += id;
                        break
                    }
                }
            }
        }    
    }
    total
}
