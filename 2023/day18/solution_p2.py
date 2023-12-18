with open('input.txt', 'r') as f:
    data = f.read().splitlines()

modifiers = {'U': (0,1), 'D': (0,-1), 'L': (-1,0), 'R': (1,0)}
dir_map = {'0': 'R', '1': 'D', '2': 'L', '3': 'U'}
path = [(0,0)]
path_length = 0
for instruction in data:
    raw_direction, raw_distance, raw_color = instruction.split()
    color = raw_color[2:-1]
    distance = int(color[0:5], 16)
    direction = dir_map[color[-1]]
    path_length += distance
    nxt = (path[-1][0] + modifiers[direction][0] * distance, path[-1][1] + modifiers[direction][1] * distance)
    if nxt != path[0]:
        path.append(nxt)

a = 0
b = 0
for x in range(len(path)-1):
    a += path[x][0] * path[x+1][1]
    b += path[x][1] * path[x+1][0]
a += path[len(path)-1][0] * path[0][1]
b += path[0][0] * path[len(path)-1][1]
area = (abs(a-b)/2) - (path_length / 2) + 1

print(f'Part one: {int(area + path_length)}')