import re

f = open('Day07_input.txt', 'r')

rules = dict()

def parse_line(line):
    line = line.strip().replace(' no other ', ' 0 ').replace('bags','bag')
    line = line.replace(',','').replace('.','').replace(' contain','')
    tokens = re.split(r'\W*bag\W*', line)
    value = tokens[0]
    for i in range(1, len(tokens)):
        key = re.match('([0-9]) (.*)', tokens[i])
        if key is None:
            continue 
        # Part 1 uncomment this
        # insert_into_rules(key.groups()[1], value)

        # Part 2
        insert_into_rules(value, key.groups())

def insert_into_rules(key, value):
    if key not in rules:
        rules[key] = [value]
    else:
        rules[key].append(value)
    
def sum_topo(root):
    if root not in rules:
        return 1
    else:
        # + 1 for ownself bag
        return sum([int(x[0]) * int(sum_topo(x[1])) for x in rules[root]]) + 1

# Main function starts here

for line in f.readlines():
    parse_line(line)

# Part 1

# queue = ['shiny gold']
# count = 0
# idx = 0

# while len(queue) > idx:
#     curr = queue[idx]
#     idx += 1
# 
#     if curr not in rules:
#         continue
# 
#     queue += [x for x in rules[curr] if x not in queue]
# 
# print(len(set(queue)) - 1)
    

# Part 2    
print(sum_topo('shiny gold') - 1)

