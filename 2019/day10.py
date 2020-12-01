from math import gcd

def seen(position, data):
	data = list(filter(lambda x: x != position, data))
	for item in data:
		diff_x = item[0] - position[0]
		diff_y = item[1] - position[1]
		d = gcd(diff_x, diff_y)
		print(d)


with open('day10test.txt', 'r') as f:
	data = [list(x) for x in f.read().split()]
	asteroids = []
	for row in range(len(data)):
		for col in range(len(data[row])):
				if data[row][col] == '#':
					asteroids.append((row,col))
	seen((0,4), asteroids)

	f.close()


