import sys

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
    current_lower_bound = sorted_list[0][0]
    # print("LB:", current_lower_bound)
    current_upper_bound = sorted_list[0][1]
    # print("UB:", current_upper_bound)
    sorted_list_length = len(sorted_list)
    # print("S_IDS:", safe_ids)
    for i in range(0, sorted_list_length):
        # print("i:", i)
        # Is this the issue? Does the last boundary not get examined, leading to too low?
        if i == sorted_list_length-1:
            if current_upper_bound < sorted_list[i][1]:
                current_upper_bound = sorted_list[i][1]
            safe_ids += (current_upper_bound-current_lower_bound+1)
            # print("S_IDS:", safe_ids)
            break
        if sorted_list[i][1] > sorted_list[i+1][0] & sorted_list[i+1][1] > sorted_list[i+1][1]:
            current_upper_bound = sorted_list[i+1][1]
            # print("LB:", current_lower_bound)
            # print("UB:", current_upper_bound)
            # print("S_IDS:", safe_ids)
        elif sorted_list[i][1] < sorted_list[i+1][0]:
            safe_ids += (current_upper_bound-current_lower_bound+1)
            current_lower_bound = sorted_list[i+1][0]
            current_upper_bound = sorted_list[i+1][1]
            # print("LB:", current_lower_bound)
            # print("UB:", current_upper_bound)
            # print("S_IDS:", safe_ids)
        # print("S_IDS:", safe_ids)
    print("Fresh IDs part two:", safe_ids)

main(data)