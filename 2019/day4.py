min = 372037
max = 905157

from collections import Counter    

def part1(n):
    digits = str(n)
    return list(digits) == sorted(digits) and any(x >= 2 for x in Counter(digits).values())

def part2(n):
    digits = str(n)
    return list(digits) == sorted(digits) and any(x == 2 for x in Counter(digits).values())

print(sum(1 for n in range(min, max) if meets_part_1(n)))
print(sum(1 for n in range(min, max) if meets_part_2(n)))

# What I learned:
# difficulty: medium
# - Thinking of changing the int to string, but fails to do so before opening reddit
# - learned counter from collections is nice
# - learned how to use any