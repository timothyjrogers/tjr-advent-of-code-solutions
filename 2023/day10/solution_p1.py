with open('input.txt', 'r') as f:
    data = f.read().splitlines()

#Pad input grid with dots
grid = []
grid.append(['.'] * (len(data[0])+2))
for line in data:
    padded_line = list(line)
    padded_line.insert(0, '.')
    padded_line.append('.')
    grid.append(padded_line)
grid.append(['.'] * (len(data[0])+2))

start_x = -1
start_y = -1
for y in range(len(grid)):
    if start_x > -1 and start_y > -1:
        break
    for x in range(len(grid[y])):
        if grid[y][x] == 'S':
            start_y = y
            start_x = x
            break

seen = set([(start_x, start_y)])
grid[start_y][start_x] = 0 #Begin path counting from 'S' / step 0
nodes = []
#Add 'S' neighbors as first nodes; 'S' encoding invalidates normal getNeighbors call
if grid[start_y - 1][start_x] == '|' or grid[start_y - 1][start_x] == '7' or grid[start_y - 1][start_x] == 'F':
    nodes.append((start_x, start_y - 1))
if grid[start_y][start_x + 1] == '-' or grid[start_y][start_x + 1] == '7' or grid[start_y][start_x + 1] == 'J':
    nodes.append((start_x + 1, start_y))
if grid[start_y + 1][start_x] == '|' or grid[start_y + 1][start_x] == 'L' or grid[start_y + 1][start_x] == 'J':
    nodes.append((start_x, start_y + 1))
if grid[start_y][start_x - 1] == '-' or grid[start_y][start_x - 1] == 'F' or grid[start_y][start_x - 1] == 'L':
    nodes.append((start_x - 1, start_y))

def getNeighbors(coord, grid):
    x = coord[0]
    y = coord[1]
    if grid[y][x] == '|':
        return [(x, y-1), (x, y+1)]
    elif grid[y][x] == '-':
        return [(x-1, y), (x+1, y)]
    elif grid[y][x] == 'L':
        return [(x, y-1), (x+1, y)]
    elif grid[y][x] == 'J':
        return [(x-1, y), (x, y-1)]
    elif grid[y][x] == '7':
        return [(x, y+1), (x-1, y)]
    elif grid[y][x] == 'F':
        return [(x+1, y), (x, y+1)]
    return []

steps = 0
while len(nodes) > 0:
    steps += 1
    for i in range(2):
        if len(nodes) > 0:
            cur = nodes.pop(0)
            seen.add(cur)
            for n in getNeighbors(cur, grid):
                if n not in seen:
                    nodes.append(n)
            grid[cur[1]][cur[0]] = steps

print(f'Part one: {steps}')