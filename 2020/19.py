import re

f = open('Day19_input.txt', 'r')
f = open('test19_part2.txt', 'r')

def parse_rule(rule):
    grps = re.match(r'([0-9]+):(.*)', rule).groups()
    rules[grps[0]] = grps[1].split('|')

def match(n, idx, line):
    def helper(n, idx, rule, line): 
        if idx >= len(line):
            print(line, rule, idx, "OOB")
            return -2
        #print(f"called with {n}, idx: {idx}, rule:{rule}, line:{line}")
        for token in rule.split(' '):
            if re.search('[0-9]+', token) is None:
                #print(line[idx:idx + len(token)], token)
                # base case
                if line[idx:idx + len(token)] == token:
                    #print("HERE")
                    return idx + 1
                else:
                    # curr character do not match.
                    return -1
            else:
                idx = match(token, idx, line)
                #print(f"result: {idx}")
                if idx < 0: # invalid
                    break
                    
        return idx


    max_idx = -1
    for rule in rules[n]:
        #print(f"applying rule: {rule}")
        rule = rule.strip().replace('"', '')
        tmp = helper(n, idx, rule, line)
        if tmp == len(line):
            return tmp
        max_idx = max(max_idx, tmp)

    # no rules match
    return max_idx

rules = {}

line = f.readline().strip()
while line != "":
    parse_rule(line)
    line = f.readline().strip()

rules['8'].extend(['42 42', '42 42 42', '42 42 42 42', '42 42 42 42 42'])
rules['11'].extend(
['42 42 31 31', '42 42 42 31 31 31', '42 42 42 42 31 31 31 31'])


count = 0
line = f.readline().strip()
#print(match('0', 0, 'bbabbbbaabaabba'))

while line:
    t = match('0', 0, line)
    print(t, len(line), line)
    if t >= len(line):
        count += 1
    line = f.readline().strip()

print(count)
