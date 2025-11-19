import os

# Part 1

# Find the item type that appears in both compartments of each rucksack.
# What is the sum of the priorities of those item types?s

# Read data to list
input_path = os.path.join(os.path.dirname(__file__), '../input/03.txt')
with open(input_path) as f:
    data = f.read().splitlines()

# Split into list of tuples
split_data = [([y for y in x[0:len(x)//2]], 
               [z for z in x[len(x)//2:len(x)]])
               for x in data]

# Calculate priority for each
split_values = [(([ord(y)-38 if y.isupper() else ord(y)-96 for y in x[0]]),
                 ([ord(z)-38 if z.isupper() else ord(z)-96 for z in x[1]]))
                 for x in split_data]

# Find matching items and add to sum of priorities
priorities_sum = 0
for pair in split_values:
    priorities_sum += (set(pair[0]) & set(pair[1])).pop()

print(priorities_sum)

# Could be optimised by making sets in split_values and getting the match from those,
# and only then checking if isupper() calculating priority with ord(x)-38 or 96

# Part 2

# Find the item type that corresponds to the badges of each three-Elf group. 
# What is the sum of the priorities of those item types?

# Split list to list of lists of size 3
n = 3
data_grouped = [data[i * n : (i + 1) * n] for i in range((len(data) + n - 1) // n)]

# List of matching objects
badges = [(set(x[0]) & set(x[1]) & set(x[2])).pop() for x in data_grouped]

# Priorities of values
badge_values = [ord(x)-38 if x.isupper() else ord(x)-96 for x in badges]

print(sum(badge_values))