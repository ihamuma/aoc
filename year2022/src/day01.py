import os

# Part 1

# Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?

# Read the input data
input_path = os.path.join(os.path.dirname(__file__), '../input/01.txt')
with open(input_path) as f:
    lines = f.read()

# Create list of calories
lines_list = lines.splitlines()

# Sum calories of each elf
all_calories = []
calories = []
for x in lines_list:
    if x == '':
        all_calories.append(sum(calories))
        calories = []
    else:
        calories.append(int(x))
# Append final line
all_calories.append(sum(calories))

# Find maximum caloriesax
max_cal = max(all_calories)

# Print results
print('Max: ', max_cal)

# Part 2

# Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?

# Pass three times, recognise max and drop from list.
# Would it be faster to sort and take last three from list?
sum = 0
for i in range(3):
    max_cal = max(all_calories)
    sum += max_cal
    all_calories.remove(max_cal)

# Print results
print('Max 3: ', sum)