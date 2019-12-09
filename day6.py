planet_list = dict()
planet_list['COM'] = 0

def buildList(key, data):
	planet_list[key[1]] = planet_list[key[0]] + 1
	item_with_key = [item for item in data if item[0] == key[1]]
	# print(planet_list, item_with_key)
	for item in item_with_key:
		buildList(item, data)

with open('day6.txt', 'r') as f:
	data = [x.split(')') for x in f.read().split()]
	start = [item for item in data if item[0] == 'COM'][0]
	buildList(start, data)
	print(sum(planet_list.values()))
	f.close()

# What I learned:
# difficulty: part a easy - med
# - dict is a nice data struct for storing the child degree of any given planet