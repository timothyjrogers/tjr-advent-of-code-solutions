with open('input.txt', 'r') as f:
    data = f.read().splitlines()

# Expand empty rows
i = 0
while i < len(data):
    data[i] = list(data[i])
    if '#' not in data[i]:
        data.insert(i+1, ['.'] * len(data[i]))
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
            data[j].insert(i, '.')
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
        manhattan = abs(a[0] - b[0]) + abs(a[1] - b[1])
        distance += manhattan

print(f'Part one: {distance}')