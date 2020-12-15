import re

f = open('Day14_input.txt', 'r')
# f = open('test2.txt', 'r')

# part 1
#def parse_mask(line):
#    mask_or = line[7:].replace('X', '0')
#    mask_and = line[7:].replace('X', '1')
#    return int(mask_or, 2), int(mask_and, 2)
#
#def parse_mem(line):
#    loc, val = re.match(r'mem\[([0-9]+)\] = ([0-9]+)', line).groups()
#    return loc, int(val)
#
#mask_or, mask_and = 0, 0
#mem = {}
#
#for line in f.readlines():
#    if line[:3] == 'mas':
#        mask_or, mask_and = parse_mask(line)
#    else:
#        loc, val = parse_mem(line)
#        val = val | mask_or
#        val = val & mask_and
#        mem[loc] = val
#
#print(sum(mem.values()))
#

# Part 2

def parse_mask(line):
    return line.strip()[7:]

def parse_mem(line):
    loc, val = re.match(r'mem\[([0-9]+)\] = ([0-9]+)', line).groups()
    return '{0:036b}'.format(int(loc)), int(val)

def write_to_mem(task, mask, start):

    for i in range(start, 36):
        if mask[i] == '0':
            mask[i] = task[0][i]
        elif mask[i] == 'X':
            mask[i] = '0'
            write_to_mem(task, mask, i + 1)
            mask[i] = '1'
            write_to_mem(task, mask, i + 1)
            mask[i] = 'X'

    if 'X' not in mask:
        addr = ''.join(mask)
        if addr not in mem:
            print(len(addr))
            mem[addr] = task[1]

# pending tasks
queue = []

mem = {}

for line in reversed(f.readlines()):
    if line[:3] != 'mas':
        queue.append(parse_mem(line))
    else:
        mask = [x for x in parse_mask(line)]
        for task in queue:
            write_to_mem(task, mask, 0)
        queue = []

# print(mem)
print(sum(mem.values()))
