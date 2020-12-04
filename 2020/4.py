import re

f = open('Day04_input.txt','r')
# f = open('invalid.txt','r')

fields = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']

def parse_line(line, data):
    d = line.split(" ")
    for x in d:
        data.append(x)

count = 0
while True:
    line = f.readline().strip()
    data = []

    if line == "":
        break

    while line != "":
        parse_line(line, data)
        line = f.readline().strip()

    num_attrib = 0
    
    for attrib in data:

        if re.match('byr:(19[2-9][0-9]|200[0-2])$', attrib):
            num_attrib += 1
        elif re.match('iyr:(201[0-9]|2020)$', attrib):
            num_attrib += 1
        elif re.match('eyr:(202[0-9]|2030)$', attrib):
            num_attrib += 1
        elif re.match('hgt:(((1[5-8][0-9]|19[0-3])cm)|((59|6[0-9]|7[0-6])in))$',
                attrib):
            num_attrib += 1
        elif re.match('hcl:#[0-9a-f]{6}$', attrib):
            num_attrib += 1
        elif re.match('ecl:(amb|blu|brn|gry|grn|hzl|oth)$', attrib):
            num_attrib += 1
        elif re.match('pid:[0-9]{9}$', attrib):
            num_attrib += 1

    if num_attrib == len(fields):
        count += 1

print(count)

