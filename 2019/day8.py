from collections import Counter

def part1(data):
	minzero, one, two = float('inf'),0,0
	for item in data:
		x = Counter(item)
		if x['0'] < minzero:
			minzero, one, two = x['0'], x['1'], x['2']
	print(one * two)

def part2(data):
	layers = [''.join([item[i] for item in data]) for i in range(150)]
	colors = ''.join([item.lstrip('2')[0] for item in layers])
	print(colors[0:25] + '\n' + colors[25:50] + '\n' + colors[50:75] + '\n' + colors[75:100] + '\n' + colors[100:125] + '\n' + colors[125:150])


with open('day8.txt', 'r') as f:
	data = []

	# for chunk in iter(lambda: f.read(150), ''):
	# 	data.append(chunk)

	c = f.read(150)
	while c != '':
		data.append(c)
		c = f.read(150)

	# part1(data)
	part2(data)

	# What I learned:
	# difficulty: medium
	# I learned how to read a file in chunks
	# remember that python has lstrip, rstrip and strip
