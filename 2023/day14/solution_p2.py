with open('input.txt', 'r') as f:
    data = f.read().splitlines()

ROUND = 'O'
SQUARE = '#'
EMPTY = '.'
answer = 0
grid = [list(x) for x in data]

def rollNorth(grid):
    for row in range(len(grid)):
        for col in range(len(grid[0])):
            if grid[row][col] == ROUND:
                shift = 0
                for i in range(1, row+1):
                    if grid[row - i][col] == EMPTY:
                        shift += 1
                    else:
                        break
                grid[row][col] = EMPTY
                grid[row - shift][col] = ROUND
                
def rotateGrid(grid):
    new_grid = [[0 for _ in range(len(grid))] for _ in range(len(grid[0]))]
    for i in range(len(grid)):
        for j in range(len(grid[0])):
            new_grid[j][i] = grid[i][j]
    for i in range(len(grid)):
        new_grid[i].reverse()
    return new_grid

initial_cycle_state = ''
states = []
states.append(''.join([''.join(row) for row in grid]))
steps = 0
for step in range(1000000000):
    for _ in range(4):
        rollNorth(grid)
        grid = rotateGrid(grid)

    gridstring = ''.join([''.join(row) for row in grid])
    if gridstring in states:
        initial_cycle_state = gridstring
        break
    else:
        steps = step
        states.append(gridstring)

cycle_start = states.index(initial_cycle_state)
cycle_length = len(states[cycle_start:])
state_index = cycle_start + ((1000000000 - cycle_start) % cycle_length)
final_state = states[state_index]

final_state_grid = []
for i in range(len(grid)):
    final_state_grid.append(final_state[i*len(grid[0]):(i+1)*len(grid[0])])

for i in range(len(final_state_grid)):
    answer += (len(final_state_grid) - i) * final_state_grid[i].count('O')

print(f'Part two: {answer}')