f = open('Day10_input.txt', 'r')

data = [0] + [int(x) for x in f.readlines()]
data.sort()

# # part 1
# count_1 = 0
# count_3 = 1 # output is 3 larger than max(data)
# 
# for i in range(1, len(data)):
#     if data[i] - data[i - 1] == 1:
#         count_1 += 1
#     elif data[i] - data[i - 1] == 3:
#         count_3 += 1
# 
# print(count_1 * count_3)

# part 2
dp = {data[len(data) - 1] : 1}

for el in reversed(data):
    count = 0
    # handle max(data) case :')
    if el in dp:
        continue

    if el + 3 in dp:
        count += dp[el + 3]

    if el + 2 in dp:
        count += dp[el + 2]

    if el + 1 in dp:
        count += dp[el + 1]

    dp[el] = count

print(dp[0])


