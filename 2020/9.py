# Part 1
# f = open('Day09_input.txt', 'r')
# 
# N = 25
# trail = []
# 
# # trail initialization
# for i in range(N):
#     trail.append(int(f.readline().strip()))
# 
# while True:
#     line = f.readline().strip()
# 
#     if line == "":
#         break
# 
#     found = False
# 
#     curr = int(line)
# 
#     for i in range(N):
#         if found:
#             continue
# 
#         for j in range(N):
#             if i != j and (trail[i] + trail[j] == curr):
#                 found = True
#                 continue
# 
#     if not found:
#         print(curr)
#         break
#     
#     trail = trail[1:] + [curr]

# Part 2
target = 556543474

f = open('Day09_input.txt', 'r')

roll_s = 0
trail = []

for line in f.readlines():
    curr = int(line.strip())

    trail.append(curr)
    roll_s += curr

    idx = 0
    while roll_s > target:
        roll_s -= trail[idx]
        idx += 1

    trail = trail[idx:]

    if roll_s == target:
        if len(trail) == 1:
            trail = []
            continue
        break

print(max(trail) + min(trail))
