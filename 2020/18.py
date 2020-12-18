from functools import reduce

f = open('Day18_input.txt', 'r')

def find_matching(line, i):
    status = 1

    while i < len(line):
        if line[i] == ')':
            status -= 1
        elif line[i] == '(':
            status += 1

        if status == 0:
            return i

        i += 1

    raise ValueError(f"No matching bracket found in line {line} with i {i}")


## Part 1
#
#def do_op(op, a, b):
#    if op == '*':
#        return a * b
#    elif op == '+':
#        return a + b
#    
#    raise ValueError(f"Unknown op {op}")
#
##def solve(line):
#    # delete all whitespace
#    line = line.replace(" ", "").strip()
#    acc = 0
#    op = '+'
#
#    i = 0
#    while i < len(line):
#        if line[i] == '(':
#            j = find_matching(line, i + 1)
#            acc = do_op(op, acc, solve(line[i + 1:j]))
#            i = j
#        elif line[i] == '+' or line[i] == '*':
#            op = line[i]
#        else:
#            # number
#            acc = do_op(op, acc, int(line[i]))
#        i += 1
#
#    return acc

# Part 2
prod_terms = []

def do_op(op, a, b, lst):
    if op =='+':
        return a + b
    elif op =='*':
        lst.append(a)
        return b

def solve(line):
    # delete all whitespace
    line = line.replace(" ", "").strip()
    prod_terms = []
    acc = 0
    op = '+'

    i = 0
    while i < len(line):
        if line[i] == '(':
            j = find_matching(line, i + 1)
            acc = do_op(op, acc, solve(line[i + 1:j]), prod_terms)
            i = j
        elif line[i] == '+' or line[i] == '*':
            op = line[i]
        else:
            # number
            acc = do_op(op, acc, int(line[i]), prod_terms)
        i += 1

    if acc != 0:
        prod_terms.append(acc)

    return reduce(lambda x,y : x * y, prod_terms, 1)

print(sum([solve(line) for line in f]))
