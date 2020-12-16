# Day16_input-cleaned is all nearby tickets (no constraints, no ownself ticket)
f = open('Day16_input-cleaned.txt', 'r');

# count = 0
possib = []

my_ticket = [int(x) for x in "179,101,223,107,127,211,191,61,199,193,181,131,89,109,197,59,227,53,103,97".split(",")]
N = len(my_ticket)

for i in range(N):
    possib.append(set(range(0, N)))

for line in f:
    tokens = line.strip().split(",")
    for idx in range(len(tokens)):
        i = int(tokens[idx])
        possible_positions = set()

        if 25<= i <=863 or 882<= i <=957:
            possible_positions.add(0)
        if 50<= i <=673 or 690<= i <=972:
            possible_positions.add(1)
        if 25<= i <=312 or 321<= i <=959:
            possible_positions.add(2)
        if 48<= i <=337 or 358<= i <=971:
            possible_positions.add(3)
        if 31<= i <=458 or 476<= i <=957:
            possible_positions.add(4)
        if 32<= i <=800 or 821<= i <=973:
            possible_positions.add(5)
        if 34<= i <=502 or 528<= i <=951:
            possible_positions.add(6)
        if 30<= i <=650 or 662<= i <=957:
            possible_positions.add(7)
        if 50<= i <=148 or 160<= i <=966:
            possible_positions.add(8)
        if 27<= i <=572 or 587<= i <=969:
            possible_positions.add(9)
        if 46<= i <=893 or 913<= i <=964:
            possible_positions.add(10)
        if 36<= i <=161 or 179<= i <=962:
            possible_positions.add(11)
        if 38<= i <=294 or 311<= i <=965:
            possible_positions.add(12)
        if 26<= i <=391 or 397<= i <=962:
            possible_positions.add(13)
        if 28<= i <=111 or 122<= i <=967:
            possible_positions.add(14)
        if 48<= i <=65 or 84<= i <=973:
            possible_positions.add(15)
        if 33<= i <=827 or 839<= i <=960:
            possible_positions.add(16)
        if 47<= i <=436 or 454<= i <=959:
            possible_positions.add(17)
        if 45<= i <=136 or 147<= i <=959:
            possible_positions.add(18)
        if 36<= i <=252 or 275<= i <=957:
            possible_positions.add(19)

        # count += i
        if len(possible_positions) > 0:
            possib[idx] &= possible_positions

# Based on possible positions, fixed the obvious fields, and continue
# to do so untill all fields are assigned.
fixed = set()
changed = True
while changed:
    any_change = False
    for i in range(len(possib)):
        if i in fixed:
            continue

        if len(possib[i]) == 1:
            fixed.add(i)
            tmp = possib[i].pop()
            for possible_positions in possib:
                possible_positions.discard(tmp)
            possib[i].add(tmp)
            any_change = True

    changed = any_change

print(possib)

# Calculate the product of "departure" fields from my_ticket
prod = 1
for i in range(N):
    if possib[i].pop() < 6:
        prod *= my_ticket[i]

print(prod)

