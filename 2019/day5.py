def Intcode(data):
	idx = 0
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
			data[id1] = int(input("Enter the ID for the ship's air conditioner unit: "))
			idx += 2
		elif(opcode == 4):
			print(data[id1])
			idx += 2
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
		print(data)

	return data

def parseInstruction(data, idx):
	start = idx
	instruction_table = [0] * 4
	instruction = data[idx]
	opcode = instruction % 100
	instruction = instruction//100
	while(instruction > 0):
		instruction_table[idx - start] = instruction % 10
		instruction = instruction // 10
		idx += 1
	return opcode, instruction_table

def check(start, n, instruction_table, data):
	return n if instruction_table[n-start-1] == 1 else data[n]

# Intcode([1002,4,3,4,33])

# with open('day5.txt', 'r') as f:
# 	data = [int(x) for x in f.read().split(',')]
# 	Intcode(data)
# 	f.close()


# What I learned: 
# difficulty: easy - med (harder to understand the problem than coding)
# - spend some time understanding the problem, esp for opcode 5 and 6, i confused on how much to add to index)