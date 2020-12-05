f = open('Day05_input.txt','r')

def parse_line(line):
    line = line.replace('R','1').replace('L','0').replace('F','0').replace('B','1') 
    return int(line[:7], 2) * 8 + int(line[7:], 2)

seats = [parse_line(line) for line in f.readlines()]
seat_max = max(seats)
seat_min = min(seats)
print(seat_max)
print(seat_min)

missing = []
for i in range(seat_min, seat_max):
    if i not in seats:
        missing.append(i)

print(missing)

