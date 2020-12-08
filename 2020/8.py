f = open('Day08_input.txt','r')
# f = open('test.txt','r')

insts = [line.strip() for line in f.readlines()]

def run():
    pc = 0
    acc = 0
    visited = [False] * len(insts)

    while pc < len(insts) and not visited[pc]:
        visited[pc] = True

        line = insts[pc]

        if line[:3] == 'jmp':
            pc += int(line[4:])
            continue
        elif line[:3] == 'acc':
            acc += int(line[4:])
        
        pc += 1

    return pc, acc

def stop_now(pc):
    return pc == len(insts)

## Part 1
#pc, acc = run()
#print(acc)

# Part 2
for i in range(len(insts)):
    if insts[i][:3] == 'jmp':
        insts[i] = 'nop' + insts[i][3:]

        pc, acc = run()

        if stop_now(pc):
            print(acc)
            break

        insts[i] = 'jmp' + insts[i][3:]

    elif insts[i][:3] == 'nop':
        insts[i] = 'jmp' + insts[i][3:]

        pc, acc = run()

        if stop_now(pc):
            print(acc)
            break

        insts[i] = 'nop' + insts[i][3:]

