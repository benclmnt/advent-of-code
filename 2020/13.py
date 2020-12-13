import math

f = open('Day13_input.txt')
#f = open('test.txt')

time = int(f.readline())

## part 1
#buses = [int(x) for x in f.readline().split(',') if x != 'x']
#earliest = min([ (math.ceil(time / x) * x - time, x) for x in buses ])
#print(earliest[0] * earliest[1])

# part 2
a = f.readline().split(',')
c = []

for i in range(len(a)):
    if a[i] == 'x':
        continue
    c.append((int(a[i]), int(a[i]) - i % int(a[i])))

# https://www.geeksforgeeks.org/chinese-remainder-theorem-set-2-implementation/
def inv(a, m) :

    m0 = m
    x0 = 0
    x1 = 1

    if (m == 1) :
        return 0

    # Apply extended Euclid Algorithm
    while (a > 1) :
        # q is quotient
        q = a // m

        t = m

        # m is remainder now, process
        # same as euclid's algo
        m = a % m
        a = t

        t = x0

        x0 = x1 - q * x0

        x1 = t

    # Make x1 positive
    if (x1 < 0) :
        x1 = x1 + m0

    return x1

def findMinX(arr) :
    # Compute product of all numbers
    prod = 1
    for i in range(0, len(arr)) :
        prod = prod * arr[i][0]

    # Initialize result
    result = 0

    # Apply above formula
    for i in range(0, len(arr)):
        pp = prod // arr[i][0]
        result = result + arr[i][1] * inv(pp, arr[i][0]) * pp


    return result % prod

# Driver method
print(findMinX(c))
