with open('input.txt', 'r') as f:
    data = f.read().splitlines()

numbers = []
in_number = False
num_start = -1
num_end = -1
sum1 = 0
for y in range(len(data)):
    for x in range(len(data[y])):
        if data[y][x].isdigit():
            if not in_number:
                num_start = x
                in_number = True
            num_end = x
        else:
            if in_number:
                numbers.append((y,(num_start, num_end)))
                in_number = False
    if in_number:
        numbers.append((y,(num_start, num_end)))
        in_number = False

for number in numbers:
    y = number[0]
    x_start = number[1][0]
    x_end = number[1][1]
    print(f'y = {y}, x_start = {x_start}, x_end = {x_end}')
    if x_start > 0 and data[y][x_start - 1] != '.':
        sum1 += int(data[y][x_start:x_end+1])
        continue
    if x_end < len(data[y])-1 and data[y][x_end + 1] != '.':
        sum1 += int(data[y][x_start:x_end+1])
        continue
    if y > 0:
        for x in range(x_start, x_end+1):
            if data[y-1][x] != '.':
                sum1 += int(data[y][x_start:x_end+1])
                continue
        if x_start > 0:
            if data[y-1][x_start-1] != '.':
                sum1 += int(data[y][x_start:x_end+1])
                continue
        if x_end < len(data[y])-1:
            if data[y-1][x_end+1] != '.':
                sum1 += int(data[y][x_start:x_end+1])
                continue
    if y < len(data)-1:
        for x in range(x_start, x_end+1):
            if data[y+1][x] != '.':
                sum1 += int(data[y][x_start:x_end+1])
                continue
        if x_start > 0:
            if data[y+1][x_start-1] != '.':
                sum1 += int(data[y][x_start:x_end+1])
                continue
        if x_end < len(data[y])-1:
            if data[y+1][x_end+1] != '.':
                sum1 += int(data[y][x_start:x_end+1])
                continue

print(f'Part one: {sum1}')