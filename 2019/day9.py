def Intcode(data):
	idx = 0
	relative_base = 0
	data = data + [0] * max(data)
	while(data[idx] and data[idx] != 99):
		opcode, instruction = parseInstruction(data, idx)
		id1 = check(idx, idx + 1, instruction, data, relative_base)
		if(opcode == 1):
			id2 = check(idx, idx + 2, instruction, data, relative_base)
			id3 = check(idx, idx + 3, instruction, data, relative_base)
			data[id3] = data[id1] + data[id2]
			idx += 4
		elif(opcode == 2):
			id2 = check(idx, idx + 2, instruction, data, relative_base)
			id3 = check(idx, idx + 3, instruction, data, relative_base)
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
				id2 = check(idx, idx + 2, instruction, data, relative_base)
				idx = data[id2]
			else:
				idx += 3
		elif(opcode == 6):
			if data[id1] == 0:
				id2 = check(idx, idx + 2, instruction, data, relative_base)
				idx = data[id2]
			else: 
				idx += 3
		elif opcode == 7:
			id2 = check(idx, idx + 2, instruction, data, relative_base)
			id3 = check(idx, idx + 3, instruction, data, relative_base)
			data[id3] = 1 if data[id1] < data[id2] else 0
			idx += 4
		elif opcode == 8:
			id2 = check(idx, idx + 2, instruction, data, relative_base)
			id3 = check(idx, idx + 3, instruction, data, relative_base)
			data[id3] = 1 if data[id1] == data[id2] else 0
			idx += 4
		elif opcode == 9:
			relative_base += data[id1]
			idx += 2
		else:
			print('error: opcode ' + str(opcode) + ' unknown')
			break

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

def check(start, n, instruction_table, data, relative_base):
	if instruction_table[n-start-1] == 1:
		return n
	elif instruction_table[n-start-1] == 2: 
		return relative_base + data[n]
	else: 
		return data[n]

with open('day9.txt', 'r') as f:
	data = [int(x) for x in f.read().split(',')]
	Intcode(data)
	f.close()


# What I learned:
# difficulty : easy- medium
# list in python can be added using + sign
# this code cannot handle inputs with really large number in instruction_table
