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
grid[start_y][start_x] = '#'
nodes = []
neighbor_dirs = []
#Add 'S' neighbors as first nodes; 'S' encoding invalidates normal getNeighbors call
if grid[start_y - 1][start_x] == '|' or grid[start_y - 1][start_x] == '7' or grid[start_y - 1][start_x] == 'F':
    nodes.append((start_x, start_y - 1))
    neighbor_dirs.append('N')
if grid[start_y][start_x + 1] == '-' or grid[start_y][start_x + 1] == '7' or grid[start_y][start_x + 1] == 'J':
    nodes.append((start_x + 1, start_y))
    neighbor_dirs.append('E')
if grid[start_y + 1][start_x] == '|' or grid[start_y + 1][start_x] == 'L' or grid[start_y + 1][start_x] == 'J':
    nodes.append((start_x, start_y + 1))
    neighbor_dirs.append('S')
if grid[start_y][start_x - 1] == '-' or grid[start_y][start_x - 1] == 'F' or grid[start_y][start_x - 1] == 'L':
    nodes.append((start_x - 1, start_y))
    neighbor_dirs.append('W')

if 'N' in neighbor_dirs and 'E' in neighbor_dirs:
    grid[start_y][start_x] = 'L'
elif 'N' in neighbor_dirs and 'S' in neighbor_dirs:
    grid[start_y][start_x] = '|'
elif 'N' in neighbor_dirs and 'W' in neighbor_dirs:
    grid[start_y][start_x] = 'J'
elif 'S' in neighbor_dirs and 'E' in neighbor_dirs:
    grid[start_y][start_x] = 'F'
elif 'S' in neighbor_dirs and 'W' in neighbor_dirs:
    grid[start_y][start_x] = '7'
elif 'E' in neighbor_dirs and 'W' in neighbor_dirs:
    grid[start_y][start_x] = '-'

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


coordinates = []
while len(nodes) > 0:
    for i in range(2):
        if len(nodes) > 0:
            cur = nodes.pop(0)
            seen.add(cur)
            for n in getNeighbors(cur, grid):
                if n not in seen:
                    nodes.append(n)
            coordinates.append((cur[0], cur[1]))

ordered_coords = []
steps = 0
cur = (start_x, start_y)
seen = set()
while len(seen) < len(coordinates):
    neighbors = getNeighbors(cur, grid)
    seen.add(cur)
    ordered_coords.append(cur)
    if neighbors[0] in seen:
        cur = neighbors[1]
    else:
        cur = neighbors[0]
    
a = 0
b = 0
for x in range(len(ordered_coords)-1):
    a += ordered_coords[x][0] * ordered_coords[x+1][1]
    b += ordered_coords[x][1] * ordered_coords[x+1][0]
a += ordered_coords[len(ordered_coords)-1][0] * ordered_coords[0][1]
b += ordered_coords[0][0] * ordered_coords[len(ordered_coords)-1][1]
area = (abs(a-b)/2) - (len(ordered_coords) / 2) + 1
print(f'Part two: {int(area)}')
