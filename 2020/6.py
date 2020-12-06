f = open('Day06_input.txt','r')

count = 0
while True:
    line = f.readline().strip()
    data = {c for c in line}

    if line == "":
        break

    while line != "":
        data &= {c for c in line}
        line = f.readline().strip()

    count += len(data)

print(count)

