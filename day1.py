def countFuel(n, start):
	if n <= 6:
		return start
	else:
		return countFuel(n // 3 - 2, start + n // 3 - 2)

def fuelCounterUpper(lines):
	data = [int(line) for line in lines]
	# part a) print(sum([math.floor(x / 3) - 2 for x in data])) --> 3391707
	print(sum([countFuel(x, 0) for x in data])) # --> 5084676


fuelCounterUpper(open('day1.txt', 'r'))