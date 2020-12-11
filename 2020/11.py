f = open('Day11_input.txt', 'r')
# f = open('test.txt', 'r')

seats = []
for line in f.readlines():
    seats.append([x for x in line.strip()])

def adj_occ(i, j):
    count = 0
    for a in [-1, 0, 1]:
        for b in [-1, 0, 1]:
            if a == 0 and b == 0:
                continue

            mul = 1
            while (0 <= mul * a + i < len(seats)
                    and 0 <= mul * b + j < len(seats[0])):
                mul += 1
                if seats[(mul-1)*a + i][(mul-1)*b + j] == '#':
                    count += 1
                    break
                elif seats[(mul-1)*a + i][(mul-1)*b + j] == 'L':
                    break
    return count

changed = True
while changed:
    new_seats = []
    changed = False
    for i in range(len(seats)):
        new_seats.append([0]*len(seats[0]))
        for j in range(len(seats[0])):
            if (seats[i][j] == '.') :
                new_seats[i][j] = '.'
                continue

            count = adj_occ(i, j)
            if (seats[i][j] == 'L' and count == 0):
                new_seats[i][j] = '#'
                changed = True
            elif (seats[i][j] == '#' and count > 4):
                new_seats[i][j] = 'L'
                changed = True
            else:
                new_seats[i][j] = seats[i][j]

    seats = new_seats

count = 0
for i in range(len(seats)):
    for j in range(len(seats[0])):
        if (seats[i][j] == '#') :
            count += 1

print(count)
