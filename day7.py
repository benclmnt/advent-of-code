from itertools import permutations
from day5 import parseInstruction, check

def Intcode(data, inputs, start):
	idx = start
	output = float('inf')

	# only for day7b
	input_used = False

	while(data[idx] and data[idx] != 99):
		opcode, instruction = parseInstruction(data, idx)
		id1 = check(idx, idx + 1, instruction, data)
		if(opcode == 1):
			id2 = check(idx, idx + 2, instruction, data)
			id3 = check(idx, idx + 3, instruction, data)
			data[id3] = data[id1] + data[id2]
			idx += 4
		elif(opcode == 2):
			id2 = check(idx, idx + 2, instruction, data)
			id3 = check(idx, idx + 3, instruction, data)
			data[id3] = data[id1] * data[id2]
			idx += 4
		elif(opcode == 3):
			# data[id1] = int(input("Enter the ID for the ship's air conditioner unit: "))

			# modified for day 7a
			# data[id1] = inputs[0]
			# inputs = inputs[1:]

			# modified for day 7b
			if input_used:
				return data, idx
			else:
				data[id1] = inputs
				input_used = True

			idx += 2
		elif(opcode == 4):
			# print(data[id1])
			output = data[id1]
			idx += 2
			# modified for day 7b
			return output, data, idx
		elif(opcode == 5):
			if data[id1] != 0:
				id2 = check(idx, idx + 2, instruction, data)
				idx = data[id2]
			else:
				idx += 3
		elif(opcode == 6):
			if data[id1] == 0:
				id2 = check(idx, idx + 2, instruction, data)
				idx = data[id2]
			else: 
				idx += 3
		elif opcode == 7:
			id2 = check(idx, idx + 2, instruction, data)
			id3 = check(idx, idx + 3, instruction, data)
			data[id3] = 1 if data[id1] < data[id2] else 0
			idx += 4
		elif opcode == 8:
			id2 = check(idx, idx + 2, instruction, data)
			id3 = check(idx, idx + 3, instruction, data)
			data[id3] = 1 if data[id1] == data[id2] else 0
			idx += 4
		else:
			print('error: opcode ' + str(opcode) + ' unknown')
			return 0, data, -2
	return output, data, -1

def part1():
	perms = list(permutations([0,1,2,3,4]))
	next_input = 0
	maxthruster = -float('inf')

	for item in perms:
		for i in range(5):
			with open('day7.txt', 'r') as f:
				data = [int(x) for x in f.read().split(',')]
				next_input = Intcode(data, [item[i], next_input])[0]
		if next_input > maxthruster:
			maxthruster = next_input
		next_input = 0
	return maxthruster

# print(part1())

def part2helper(item, data):
	#shallow copy state to data
	datas = [0] * 5
	for i in range(5):
		datas[i] = list(data)

	index = [0] * 5
	idx = 0

	next_input = 0
	maxthruster = -float('inf')

	for i in range(5):
		datas[i], index[i] = Intcode(datas[i], item[i], 0)

	while index[idx % 5] >= 0 :
		next_input, datas[idx % 5], index[idx % 5] = Intcode(datas[idx % 5], next_input, index[idx % 5])
		idx += 1
		if next_input > maxthruster and next_input < float('inf'):
		 	maxthruster = next_input
	return maxthruster
	
def part2():
	perms = list(permutations([5,6,7,8,9]))
	data = [int(x) for x in open('day7.txt','r').read().split(',')]
	# data = [3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,1,0,5]
	maxthruster = -float('inf')
	for item in perms:
		result = part2helper(item, data)
		if result > maxthruster:
			maxthruster = result
	return maxthruster
print(part2())

# What I learned:
# Difficulty: hard - v. hard
# - Used flags like input_used
# - think of how to emulate running 5 different program in 5 connected machines without stopping it for part b, which lead to passing the current index.
# - spend a lot of time to debug, since my part a and part b needs different input
# - itertools has some nice functions yay
# - learned that [x] * 4 refers to the same object x if x is mutable, so not a good idea to use. Learned how to shallow copy using list(x)