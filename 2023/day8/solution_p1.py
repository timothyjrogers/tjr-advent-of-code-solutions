with open('input.txt', 'r') as f:
    data = f.read().splitlines()

instructions = data[0]
nodes = {}
for line in data[2:]:
    parts = line.split(' = ')
    node = parts[0]
    paths = parts[1].split(', ')
    nodes[node] = [paths[0][1:], paths[1][:-1]]

cur = 'AAA'
steps = 0
while cur != 'ZZZ':
    direction = instructions[steps % len(instructions)]
    steps += 1
    if direction == 'L':
        cur = nodes[cur][0]
    else:
        cur = nodes[cur][1]

print(f'Part one: {steps}')