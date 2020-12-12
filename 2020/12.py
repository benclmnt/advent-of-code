# Only works because input is good enough
# I'm also lazy to implement more general solutions

f = open('Day12_input.txt', 'r')
# f = open('test.txt', 'r')

x, y = 0, 0
way_x, way_y = 10, 1

face = 0 # facing east
for line in f.readlines():
    line = line.strip()
    act = line[:1]
    n = int(line[1:])
    if act == 'F':
        x += n * way_x
        y += n * way_y
    elif act == 'N':
        way_y += n
    elif act == 'S':
        way_y -= n
    elif act == 'E':
        way_x += n
    elif act == 'W':
        way_x -= n
    elif act == 'L':
        if n == 0:
            pass
        elif n == 90:
            way_x, way_y = -way_y, way_x
        elif n == 180:
            way_x, way_y = -way_x, -way_y
        elif n == 270:
            way_x, way_y = way_y, -way_x
        else:
            raise ValueError("face not good leh")
    elif act == 'R':
        if n == 0:
            pass
        elif n == 270:
            way_x, way_y = -way_y, way_x
        elif n == 180:
            way_x, way_y = -way_x, -way_y
        elif n == 90:
            way_x, way_y = way_y, -way_x
        else:
            raise ValueError("face not good leh")
    else:
        raise ValueError("wah")

print(abs(x) + abs(y))

