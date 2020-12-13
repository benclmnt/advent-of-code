def getPoints(A):
	DX = dict(zip('LRUD', [-1,1,0,0]))
	DY = dict(zip('LRUD', [0,0,1,-1]))
	x, y, length, points = 0, 0, 0, {}
	for i in A:
		direction = i[0]
		magnitude = int(i[1:])
		for j in range(magnitude):
			x += DX[direction]
			y += DY[direction]
			length += 1
			if((x,y) not in points):
				points[(x,y)] = length
	return points

with open('day3.txt', 'r') as f:
	[a, b] = [x.split(',') for x in f.read().split()]
	pointsA = getPoints(a)
	pointsB = getPoints(b)
	both = set(pointsA.keys()) & set(pointsB.keys())
	part1 = min(abs(x) + abs(y) for (x,y) in both)
	part2 = min(pointsA[i] + pointsB[i] for i in both)
	print(part1, part2)
	f.close()
