import os
import queue

# Part 1

# After the rearrangement procedure completes, what crate ends up on top of each stack?

# Define rows of data structure
n = 9

# Read data to list
input_path = os.path.join(os.path.dirname(__file__), '../input/05.txt')
with open(input_path) as f:
    data = f.read().splitlines()

# Separate structure to individual characters
data_separated = [[y for y in x] for x in data[:n]]

# Transpose to get list of stack
transposed_tuples = list(zip(*data_separated))
transposed = [list(reversed(t)) for t in transposed_tuples]

# Create dict crate stacks. 
#   Key: number of stack. 
#   Value: LifoQueue of crates.
stacks = {}
for x in transposed:
    # Check first element of sublist isn't undesirable
    if x[0] in {' ', '[', ']'}:
        continue
    else:
        # Create LifoQueue for each stack of crates
        stacks[int(x[0])] = queue.LifoQueue()
        for y in x[1:len(x)]:
            # If char isn't undesirable, put in queue for stack
            if y != ' ':
                stacks[int(x[0])].put(y)

# List moving instructions
instructions = [x.split(' ') for x in data[n + 1:]]

# Move crates according to instructions
for x in instructions:
    for i in range(0, int(x[1])):
        stacks[int(x[5])].put(stacks[int(x[3])].get())

# Identify top crate
top_crates = [stacks[x].get() for x in stacks]

print('Top crates: ', ''.join(top_crates))

# Part 2

# After the [updated] rearrangement procedure completes, what crate ends up on top of each stack?

# Create dict crate stacks. 
#   Key: number of stack. 
#   Value: LifoQueue of crates.
stacks = {}
for x in transposed:
    # Check first element of sublist isn't undesirable
    if x[0] in {' ', '[', ']'}:
        continue
    else:
        # Create LifoQueue for each stack of crates
        stacks[int(x[0])] = queue.LifoQueue()
        for y in x[1:len(x)]:
            # If char isn't undesirable, put in queue for stack
            if y != ' ':
                stacks[int(x[0])].put(y)

# Parse a row of instructions for amount of crates to move from stack to stack
def parseinstruction(instruction_row):
    amount = int(instruction_row[1])
    fromstack = int(instruction_row[3])
    tostack = int(instruction_row[5])
    return amount, fromstack, tostack

# Moce crates according to instructions (part2 scenario)
def movecrates(amount, fromstack, tostack):
    # Move amount of crates to temporary stack
    tempstack = queue.LifoQueue()
    for i in range(0, amount):
        tempstack.put(stacks[fromstack].get())
    # Move amount of crates from tempstack to correct stack
    for i in range(0, amount):
        stacks[tostack].put(tempstack.get())

# Move crates according to each row of instructions
for row in instructions:
    amount, fromstack, tostack = parseinstruction(row)
    movecrates(amount, fromstack, tostack)

# Identify top crate of each stack
top_crates = [stacks[stack].get() for stack in stacks]

# Print result
print('Top crates 2: ', ''.join(top_crates))