import re

def is_valid(text):
    parsed = re.match('(\d+)-(\d+) ([a-z]): ([a-z]*)', text).groups()
    # return int(parsed[0]) <= parsed[3].count(parsed[2]) <= int(parsed[1]) 
    x = parsed[3][int(parsed[0]) - 1]
    y = parsed[3][int(parsed[1]) - 1]

    return x != y and (x == parsed[2] or y == parsed[2])

print(len([x for x in open('Day02_Input.txt', 'r') if is_valid(x)]))
