def Intcode(data):
	idx = 0
	while(data[idx] and data[idx] != 99):
		if(data[idx] == 1):
			data[data[idx + 3]] = data[data[idx + 1]] + data[data[idx + 2]]
		elif(data[idx] == 2):
			data[data[idx + 3]] = data[data[idx + 1]] * data[data[idx + 2]]
		idx += 4
	return data

# print(Intcode([1,0,0,0,99]))
# print(Intcode([2,3,0,3,99]))
# print(Intcode([2,4,4,5,99,0]))
# print(Intcode([1,1,1,4,99,5,6,0,99]))
# print(Intcode([1,9,10,3,2,3,11,0,99,30,40,50]))


for i in range(100):
	for j in range(100):
		with open('day2.txt', 'r') as f:
			data = [int(x) for x in f.read().split(',')]
			data[1] = i
			data[2] = j
			if(Intcode(data)[0] == 19690720):
				print(100 * i + j)
				break
			f.close()

