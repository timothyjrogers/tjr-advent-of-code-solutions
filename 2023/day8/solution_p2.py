from math import gcd

with open('input.txt', 'r') as f:
    data = f.read().splitlines()

instructions = data[0]
nodes = {}
for line in data[2:]:
    parts = line.split(' = ')
    node = parts[0]
    paths = parts[1].split(', ')
    nodes[node] = [paths[0][1:], paths[1][:-1]]

cur_nodes = [x for x in nodes.keys() if x.endswith('A')]
cur_steps = [0 for x in cur_nodes]

def checkAllNodes(nodes):
    for n in nodes:
        if not n.endswith('Z'):
            return False
    return True

steps = 0
while not checkAllNodes(cur_nodes):
    direction = instructions[steps % len(instructions)]
    steps += 1
    for i in range(len(cur_nodes)):
        if not cur_nodes[i].endswith('Z'):
            cur_steps[i] += 1
            if direction == 'L':
                cur_nodes[i] = nodes[cur_nodes[i]][0]
            else:
                cur_nodes[i] = nodes[cur_nodes[i]][1]

lcm = 1
for i in cur_steps:
    lcm = lcm * i // gcd(lcm,i)

print(f'Part two: {lcm}')