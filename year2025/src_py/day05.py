import sys
import time

# Read input file path from command line argument
input_path = sys.argv[1]

with open(input_path) as f:
    data = f.read().strip()

def main(data):
    split_lines = data.splitlines()
    idx_to_split = split_lines.index('')
    safe_id_rs = split_lines[:idx_to_split]
    ingredient_ids = split_lines[idx_to_split+1:]
    safe_id_r_tuples = []
    freshcount = 0
    for r in safe_id_rs:
        r_str_tup = tuple(r.split('-'))
        r_int_tup = tuple((int(r_str_tup[0]), int(r_str_tup[1])))
        safe_id_r_tuples.append(r_int_tup)
    sorted_list = sorted(safe_id_r_tuples, key=lambda x: x[0])
    for id in ingredient_ids:
        int_id = int(id)
        for r in sorted_list:
            if int_id >= r[0] and int_id <= r[1]:
                freshcount += 1
                break
    print("Fresh IDs part one: ", freshcount)
    
    safe_ids = 0
    for safe_id_r in sorted_list:
        for id in range(int(safe_id_r[0]), int(safe_id_r[1])+1):
            safe_ids.append(id)
    fucking_set = set(safe_ids)
    print("Fresh IDs part two:", len(fucking_set))

main(data)