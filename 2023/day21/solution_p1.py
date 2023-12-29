START = 'S'
GARDEN = '.'
ROCK = '#'
GOAL = 64

def findSteps(grid, start_coord, steps, taken):
    if taken == steps:
        return [start_coord]
    coords = []
    for change in [[-1, 0], [1, 0], [0, -1], [0,1]]:
        coord = (start_coord[0] + change[0], start_coord[1] + change[1])
        if coord[0] >= 0 and coord[0] < len(grid[0]) and coord[1] >= 0 and coord[1] < len(grid) and grid[coord[1]][coord[0]] != '#':
            coords.extend(findSteps(grid, coord, steps, taken + 1))
    return coords
        

with open('input.txt', 'r') as f:
    data = f.read().splitlines()

grid = [list(x) for x in data]

start_coord = (-1,-1)
for y in range(len(grid)):
    for x in range(len(grid[0])):
        if grid[y][x] == START:
            start_coord = (x, y)
            break
    if start_coord != (-1, -1):
        break

nodes = [start_coord]
for _ in range(GOAL):
    new_nodes = []
    for node in nodes:
        for change in [[-1, 0], [1, 0], [0, -1], [0,1]]:
            coord = (node[0]+change[0], node[1]+change[1])
            if coord[0] >= 0 and coord[0] < len(grid[0]) and coord[1] >= 0 and coord[1] < len(grid) and grid[coord[1]][coord[0]] != '#':
                new_nodes.append(coord)
    nodes = list(set(new_nodes))

answer = len(set(nodes))
print(f'Part one: {answer}')