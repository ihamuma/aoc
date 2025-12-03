use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

pub fn solve(input_file: &str) {
    let by_line: Vec<String> = fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let (rules, updates) = extract_rules_and_updates(&by_line);

    let mapped_rules = tuples_to_hashmap(&rules);

    let mut valid_mid_elem_sum = 0;
    let mut invalid_updates = Vec::new();
    for update in updates {
        if update_is_valid(&update, &mapped_rules) {
            valid_mid_elem_sum += middle_element(&update);
        } else {
            invalid_updates.push(update);
        }
    }
    println!("Sum of valid middle elements: {valid_mid_elem_sum}");

    let mut fixed_mid_elem_sum = 0;
    for mut inv in invalid_updates {
        let fixed = fix_invalid_update(&mut inv, &mapped_rules);
        fixed_mid_elem_sum += middle_element(&fixed);
    }
    println!("Sum of fixed middle elements: {fixed_mid_elem_sum}");
}

fn extract_rules_and_updates(r_and_u: &Vec<String>) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut rules = Vec::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();
    for line in r_and_u {
        if line.contains('|') {
            if let Some((p1, p2)) = line.split_once('|') {
                rules.push((p1.parse::<u32>().unwrap(), p2.parse::<u32>().unwrap()));
            } else {
                println!("{line} didn't compile to a rule");
            }
        }
        if line.contains(',') {
            let split: Vec<u32> = line.split(',').map(|x| x.parse::<u32>().unwrap()).collect();
            updates.push(split);
        }
    }
    (rules, updates)
}

fn tuples_to_hashmap(tup_vec: &Vec<(u32, u32)>) -> HashMap<u32, HashSet<u32>> {
    let mut map = HashMap::new();
    for tup in tup_vec {
        map.entry(tup.0).or_insert_with(HashSet::new).insert(tup.1);
    }
    map
}

fn update_is_valid(update: &[u32], rules: &HashMap<u32, HashSet<u32>>) -> bool {
    for i in 1..update.len() {
        let Some(rule_set) = rules.get(&update[i]) else { continue };
        for j in update.iter().take(i) { 
            if rule_set.contains(j) {
                return false;
            }
        }
    }
    true
}

fn middle_element(vec: &[u32]) -> u32 {
    vec[vec.len() / 2]
}

fn fix_invalid_update(update: &mut [u32], rules: &HashMap<u32, HashSet<u32>>) -> Vec<u32> {
    while !update_is_valid(update, rules) {
        for i in 1..update.len() {
            let Some(rule_set) = rules.get(&update[i]) else { continue };
            for j in 0..i {
                if rule_set.contains(&update[j]) {
                    update.swap(i, j);
                }
            }
        }
    }
    update.to_vec()
}
