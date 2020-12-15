N = 30_000_000
# start = [0, 3, 6]
start = [1, 2, 16, 19, 18, 0]

mem = {}

latest = start[0]

for i in range(1, len(start)):
    mem[latest] = i
    latest = start[i]

for i in range(len(start), N):
    to_say = 0 if latest not in mem else i - mem[latest]
    mem[latest] = i
    latest = to_say

print(latest)
