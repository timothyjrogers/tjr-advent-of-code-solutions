with open('input.txt', 'r') as f:
    data = f.read().splitlines()

# Expand empty rows
i = 0
while i < len(data):
    data[i] = list(data[i])
    if '#' not in data[i]:
        data.insert(i+1, ['M'] * len(data[i]))
        i += 1
    i += 1

# Expand empty columns
i = 0
while i < len(data[0]):
    col_empty = True
    for j in range(len(data)):
        if data[j][i] == '#':
            col_empty = False
            break
    if not col_empty:
        i += 1
        continue
    else:
        j = 0
        while j < len(data):
            data[j].insert(i, 'M')
            j += 1
        i += 1
    i += 1

# Find galaxy coordinates
galaxies = []
for y in range(len(data)):
    for x in range(len(data[0])):
        if data[y][x] == '#':
            galaxies.append((x,y))

distance = 0
for i in range(len(galaxies) - 1):
    for j in range(i + 1, len(galaxies)):
        a = galaxies[i]
        b = galaxies[j]

        x_steps = 0
        y_steps = 0
        if a[0] <= b[0]:
            for x in range(a[0]+1, b[0]+1):
                if data[a[1]][x] == 'M':
                    x_steps += 999999
                else:
                    x_steps += 1
        else:
            for x in range(b[0]+1, a[0]+1):
                if data[b[1]][x] == 'M':
                    x_steps += 999999
                else:
                    x_steps += 1
        if a[1] <= b[1]:
            for y in range(a[1]+1,b[1]+1):
                if data[y][a[0]] == 'M':
                    y_steps += 999999
                else:
                    y_steps += 1
        else:
            for y in range(b[1]+1,a[1]+1):
                if data[y][b[0]] == 'M':
                    y_steps += 999999
                else:
                    y_steps += 1
        distance += x_steps + y_steps

print(f'Part two: {distance}')