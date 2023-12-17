with open('input.txt', 'r') as f:
    data = f.read().splitlines()

grid = [[int(x) for x in y] for y in data]
START_X = 0
START_Y = 0
END_X = len(grid[0])-1
END_Y = len(grid)-1

# State = (x position, y position, x direction, y direction, length of run)
costs = {(START_X, START_Y, 0, 0, 0): 0}
seen = set()
nodes = set([(START_X, START_Y, 0, 0, 0)])
while len(nodes) > 0:
    cur = min(nodes, key=costs.__getitem__)
    x, y, x_dir, y_dir, length = cur
    nodes.remove(cur)
    seen.add(cur)
    if length < 4 and x_dir != y_dir:
        dx = x_dir
        dy = y_dir
        steps = 4 - length
        new_length = 4
        if x + (dx * steps) < START_X or x + (dx * steps) > END_X or y + (dy * steps) < START_Y or y + (dy * steps) > END_Y:
            continue
        cost = costs[(x,y,x_dir,y_dir,length)]
        for i in range(1, steps+1):
            cost += grid[y + (dy * i)][x + (dx * i)]
        new_state = (x+(dx * steps),y+(dy * steps),dx,dy,new_length)
        if new_state in seen:
            continue
        if new_state not in costs or cost < costs[new_state]:
            costs[new_state] = cost
        nodes.add(new_state)        
    else:
        for dx, dy in [[-1,0],[1,0],[0,-1],[0,1]]:
            new_length = length
            if x + dx < START_X or x + dx > END_X or y + dy < START_Y or y + dy > END_Y:
                continue
            if x_dir == -1 * dx and y_dir == -1 * dy:
                continue
            if x_dir == dx and y_dir == dy:
                new_length += 1
                if new_length > 10:
                    continue
            else:
                new_length = 1
            cost = costs[(x,y,x_dir,y_dir,length)] + grid[y+dy][x+dx]
            new_state = (x+dx,y+dy,dx,dy,new_length)
            if new_state in seen:
                continue
            if new_state not in costs or cost < costs[new_state]:
                costs[new_state] = cost
            nodes.add(new_state)

minimum = 1000000000
for k,v in costs.items():
    if k[0] == END_X and k[1] == END_Y and k[4] >= 4 and v < minimum:
        minimum = v

print(f'Part two: {minimum}')