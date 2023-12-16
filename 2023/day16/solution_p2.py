V_SPLIT = '|'
H_SPLIT = '-'
L_MIRROR = '\\'
R_MIRROR = '/'
EMPTY = '.'
BUFFER = '0'
DIR_RIGHT = 'R'
DIR_LEFT = 'L'
DIR_DOWN = 'D'
DIR_UP = 'U'

with open('input.txt', 'r') as f:
    data = f.read().splitlines()

grid = []
grid.append([BUFFER] * (len(data[0])+2))
for i in range(len(data)):
    grid.append([BUFFER] + list(data[i]) + [BUFFER])
grid.append([BUFFER] * (len(data[0])+2))

def gridEnergy(grid, start):
    rays = [start]
    seen = set()
    active = {}
    while len(rays) > 0:
        cur = rays.pop()
        if grid[cur[0][1]][cur[0][0]] == BUFFER:
            continue
        if cur in seen:
            continue
        else:
            seen.add(cur)
        active[(cur[0][0], cur[0][1])] = True
        cur_cell = grid[cur[0][1]][cur[0][0]]
        if cur[1] == DIR_RIGHT:
            if cur_cell == EMPTY or cur_cell == H_SPLIT:
                rays.append(((cur[0][0]+1, cur[0][1]), DIR_RIGHT))
            elif cur_cell == L_MIRROR:
                rays.append(((cur[0][0], cur[0][1]+1), DIR_DOWN))
            elif cur_cell == R_MIRROR:
                rays.append(((cur[0][0], cur[0][1]-1), DIR_UP))
            elif cur_cell == V_SPLIT:
                rays.append(((cur[0][0], cur[0][1]+1), DIR_DOWN))
                rays.append(((cur[0][0], cur[0][1]-1), DIR_UP))
        elif cur[1] == DIR_LEFT:
            if cur_cell == EMPTY or cur_cell == H_SPLIT:
                rays.append(((cur[0][0]-1, cur[0][1]), DIR_LEFT))
            elif cur_cell == L_MIRROR:
                rays.append(((cur[0][0], cur[0][1]-1), DIR_UP))
            elif cur_cell == R_MIRROR:
                rays.append(((cur[0][0], cur[0][1]+1), DIR_DOWN))
            elif cur_cell == V_SPLIT:
                rays.append(((cur[0][0], cur[0][1]-1), DIR_UP))
                rays.append(((cur[0][0], cur[0][1]+1), DIR_DOWN))
        elif cur[1] == DIR_UP:
            if cur_cell == EMPTY or cur_cell == V_SPLIT:
                rays.append(((cur[0][0], cur[0][1]-1), DIR_UP))
            elif cur_cell == L_MIRROR:
                rays.append(((cur[0][0]-1, cur[0][1]), DIR_LEFT))
            elif cur_cell == R_MIRROR:
                rays.append(((cur[0][0]+1, cur[0][1]), DIR_RIGHT))
            elif cur_cell == H_SPLIT:
                rays.append(((cur[0][0]-1, cur[0][1]), DIR_LEFT))
                rays.append(((cur[0][0]+1, cur[0][1]), DIR_RIGHT))
        elif cur[1] == DIR_DOWN:
            if cur_cell == EMPTY or cur_cell == V_SPLIT:
                rays.append(((cur[0][0], cur[0][1]+1), DIR_DOWN))
            elif cur_cell == L_MIRROR:
                rays.append(((cur[0][0]+1, cur[0][1]), DIR_RIGHT))
            elif cur_cell == R_MIRROR:
                rays.append(((cur[0][0]-1, cur[0][1]), DIR_LEFT))
            elif cur_cell == H_SPLIT:
                rays.append(((cur[0][0]+1, cur[0][1]), DIR_RIGHT))
                rays.append(((cur[0][0]-1, cur[0][1]), DIR_LEFT))
    return len(active)

answer = 0
for x in range(1, len(grid[0])-1):
    cur = gridEnergy(grid, ((x, 1), 'D'))
    if cur > answer:
        answer = cur
for x in range(1, len(grid[0])-1):
    cur = gridEnergy(grid, ((x, len(grid)-2), 'U'))
    if cur > answer:
        answer = cur
for y in range(1, len(grid)-1):
    cur = gridEnergy(grid, ((1, y), 'R'))
    if cur > answer:
        answer = cur
for y in range(1, len(grid)-1):
    cur = gridEnergy(grid, ((len(grid[0])-2, y), 'R'))
    if cur > answer:
        answer = cur
    

print(f'Part two: {answer}')