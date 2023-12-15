with open('input.txt', 'r') as f:
    data = f.read().splitlines()

ROUND = 'O'
SQUARE = '#'
EMPTY = '.'
answer = 0
grid = [list(x) for x in data]

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
            answer += len(grid) - (row - shift)

print(f'Part one: {answer}')