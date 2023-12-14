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
    return new_grid

def findReflectionLine(grid):
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
            if reflection_start == 0 or reflection_end == len(grid)-1:
                return reflection_line
    return -1

for section in data:
    grid = section.split('\n')
    reflection_line = findReflectionLine(grid)
        
    if reflection_line > -1:
        answer += 100 * reflection_line
        continue
    else:
        grid = rotateGrid(grid)

    reflection_line = findReflectionLine(grid)
    if  reflection_line > -1:
        answer += reflection_line

print(f'Part one: {answer}')