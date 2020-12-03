nums = []

with open('1a.in', 'r') as f:
    for line in f:
        nums.append(int(line))

    for j in range(len(nums)):
        for i in range(j + 1, len(nums)):
            if (2020 - nums[j] - nums[i]) in nums[i + 1:]:
                print( nums[j] * nums[i] * (2020 - nums[j] - nums[i] ) )

