import os

# Part 1

# What would your total score be if everything goes exactly according to your strategy guide?

# Clarifying rules and points - opponent and what own choice beats what
guide = {'A': 'rock', 'B': 'paper', 'C': 'scissors',
         'Y': 'paper', 'Z': 'scissors', 'X': 'rock'}
points = {'X': 1, 'Y': 2, 'Z': 3}
points_outcome = {'L': 0, 'T': 3, 'W': 6}

# Win-loss-tie
wlt = {'A': {'X': 'T', 'Y': 'W', 'Z': 'L'},
       'B': {'X': 'L', 'Y': 'T', 'Z': 'W'},
       'C': {'X': 'W', 'Y': 'L', 'Z': 'T'}}

# Read data
input_path = os.path.join(os.path.dirname(__file__), '../input/02.txt')
with open(input_path) as f:
    data = f.read()

# Read to list
match_list = data.splitlines()

# Use created dict structure to find the winner and loser of each and add related points to sum.
score = 0
for x in match_list:
    home = x[0]
    away = x[2]
    score += points[away]
    outcome = wlt[home][away]
    score += points_outcome[outcome]

print(score)

# Part 2

# Following the Elf's [updated] instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?

# "Translate" new rules
rules = {'X': 'L', 'Y': 'T', 'Z': 'W'}

# Transform previous to reflect new outcome critera
oc = {}
for d in wlt:
    oc[d] = {v: k for k, v in wlt[d].items()}

# For each match
score_two = 0
for x in match_list:
    opponent_choice = x[0]
    required_outcome = x[2]
    # Check required outcome, add points for that
    outcome = rules[required_outcome]
    score_two += points_outcome[outcome]
    # Check which choice reflects that outcome, add points for that
    choice = oc[opponent_choice][outcome]
    score_two += points[choice]

print(score_two)