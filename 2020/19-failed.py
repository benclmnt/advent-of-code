import re

f = open('Day19_input.txt', 'r')
#f = open('test19_part2.txt', 'r')
#f = open('test19.txt', 'r')

def parse_rule(rule):
    grps = re.match(r'([0-9]+):(.*)', rule).groups()
    raw_rules[grps[0]] = grps[1].split('|')

def create_rules(n):
    rules[n] = []
    for rrule in raw_rules[n]:
        num_patterns = re.findall("([0-9]+)", rrule)

        if len(num_patterns) == 0:
            rules[n].append(rrule.strip().replace('"', ''))
            continue

        tmp = [rrule]
        for j in num_patterns:
            if j not in rules:
                ptrns = create_rules(j)
            else:
                ptrns = rules[j]

            while j in tmp[0].split():
                curr = tmp.pop(0)
                for ptrn in ptrns:
                    tmp.append(' '.join(list(map(lambda x: ptrn if x == j else x,
                        curr.split()))))
                    

        rules[n].extend(tmp)
    #print(n, rules[n])
    return rules[n]

raw_rules = {}
rules = {}

line = f.readline().strip()

while line != "":
    parse_rule(line)
    line = f.readline().strip()

create_rules('0')

for key in rules.keys():
    rules[key] = list(map(lambda x: x.replace(" ",""), rules[key]))

print(rules['42'], rules['31'])
# print(rules)

count = 0

#line = f.readline().strip()
#while line:
#    line = f.readline().strip()
#    for ptrn in rules['0']:
#        if re.match(ptrn, line):
#            count += 1
#            break
#
#print(count)


