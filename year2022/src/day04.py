import os

# Part 1

# In how many assignment pairs does one range fully contain the other?

# Read data to list
input_path = os.path.join(os.path.dirname(__file__), '../input/04.txt')
with open(input_path) as f:
    data = f.read().splitlines()

# Split by ',' to separate pairs, '-' for individual values and transform to int
data_pairs = [[[int(z) for z in y.split('-', 1)] for y in x.split(',', 1)] for x in data]

# Identify completely overlapping ranges
contained_ranges = 0
for x in data_pairs:
    if ((x[0][0] <= x[1][0]) and (x[0][1] >= x[1][1])) or ((x[1][0] <= x[0][0]) and (x[1][1] >= x[0][1])):
        contained_ranges += 1

print(contained_ranges)

# Part 2

# In how many assignment pairs do the ranges overlap [at all]?

# Identify all overlapping ranges
overlap_ranges = 0
for x in data_pairs:
    if not ((x[0][1] < x[1][0]) or (x[1][1] < x[0][0])):
        overlap_ranges += 1

print(overlap_ranges)