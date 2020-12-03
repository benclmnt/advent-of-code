from functools import reduce

def go_down(r, d):
    f = open('Day03_Input.txt','r')
    line = f.readline()
    count = 0
    x = r
    while True:
        for i in range(d):
            line = f.readline().strip()
        
        if not line:
            break

        x = x if x < len(line) else x - len(line)
        if line[x] == "#":
            count += 1

        x += r

    print(r, d, count)
    return count

print(reduce((lambda x,y: x * y), [go_down(x[0], x[1]) for x in [(1,1), (3,1), (5,1), (7,1), (1,2)]]))
    
    
