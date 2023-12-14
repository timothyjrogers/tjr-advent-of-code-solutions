with open('input.txt', 'r') as f:
    data = f.read().split('\n\n')

answer = 0

def rotateGrid(grid):
    new_grid = [[0 for _ in range(len(grid))] for _ in range(len(grid[0]))]
    for i in range(len(new_grid)):
        new_grid[i].reverse()
    for i in range(len(grid)):
        for j in range(len(grid[0])):
            new_grid[j][i] = grid[i][j]
    return [''.join(x) for x in new_grid]

def findReflectionLine(grid, exclude):
    for y in range(1,len(grid)):
            # Two adjacent, matching rows could be the center of the reflection
            if grid[y] == grid[y-1]:
                reflection_line = y
            else:
                continue

            # Establish the current "outer edge" of the reflection
            reflection_start = reflection_line - 1
            reflection_end = reflection_line
            while reflection_start > -1 and reflection_end < len(grid): # While the reflection area is still within the grid bounds
                if grid[reflection_start] == grid[reflection_end]: # and reflection_start > -1 and reflection_end < len(grid):
                    reflection_start -= 1
                    reflection_end += 1
                else:
                    reflection_start += 1
                    reflection_end -= 1
                    break
            if reflection_start == -1:
                reflection_start = 0
            if reflection_end == len(grid):
                reflection_end = len(grid) - 1
            if reflection_line != exclude and (reflection_start == 0 or reflection_end == len(grid)-1):
                return reflection_line
    return -1

sections_p1 = {}
sections_p1_h = {}
sections_p1_v = {}
for n,section in enumerate(data):
    grid = section.split('\n')
    reflection_line = findReflectionLine(grid, -1)
        
    if reflection_line > -1:
        sections_p1_h[n] = reflection_line
        continue
    else:
        grid = rotateGrid(grid)

    reflection_line = findReflectionLine(grid, -1)
    if  reflection_line > -1:
        sections_p1_v[n] = reflection_line
    
# Part 2
for n, section in enumerate(data):
    grid = section.split('\n')
    done = False
    for col in range(len(grid[0])):
        if done:
            break
        for row in range(len(grid)):
            factor = 100
            if done:
                break
            if grid[row][col] == '.':
                grid[row] = grid[row][:col] + '#' + grid[row][col+1:]
            else:
                grid[row] = grid[row][:col] + '.' + grid[row][col+1:]
            reflection_line = findReflectionLine(grid, -1 if n not in sections_p1_h else sections_p1_h[n])
            if reflection_line == -1:
                factor = 1
                reflection_line = findReflectionLine(rotateGrid(grid), -1 if n not in sections_p1_v else sections_p1_v[n])
            if reflection_line > -1:
                answer += factor * reflection_line
                done = True
            if grid[row][col] == '.':
                grid[row] = grid[row][:col] + '#' + grid[row][col+1:]
            else:
                grid[row] = grid[row][:col] + '.' + grid[row][col+1:]


                


print(f'Part two: {answer}')