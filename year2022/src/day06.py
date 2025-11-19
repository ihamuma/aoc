import os
import queue

# How many characters need to be processed before the first start-of-packet marker (four different characters() is detected?

# Read data to list
input_path = os.path.join(os.path.dirname(__file__), '../input/06.txt')
with open(input_path) as f:
    data = f.read()

# Create list of individual chars
letters = [letter for letter in data]

# Initialise list, append first three letters to list
marker = letters[0:3]

# Counter for current letter
index = 3

# Pass through remaining letters
for letter in letters[3:]:
    # Add one to index
    index += 1
    # Append latest char to list
    marker.append(letter)
    # Create set from list
    marker_set = set(marker)
    # If length the same, no duplicates and end of marker found
    if len(marker) == len(marker_set):
        print('First package marker at: ', index)
        break
    # If not the same, reset marker as last 3 of current marker
    else:
        marker = marker[-3:]

# How many characters need to be processed before the first start-of-message marker is detected?

# Initialise list, append first three letters to list
messagemarker = letters[0:13]

# Counter for current letter
messageindex = 13

# Pass through remaining letters
for letter in letters[13:]:
    # Add one to index
    messageindex += 1
    # Append latest char to list
    messagemarker.append(letter)
    # Create set from list
    message_set = set(messagemarker)
    # If length the same, no duplicates and end of marker found
    if len(messagemarker) == len(message_set):
        print('First message marker at: ', messageindex)
        break
    # If not the same, reset marker as last 3 of current marker
    else:
        messagemarker = messagemarker[-13:]