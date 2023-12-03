with open('input.txt', 'r') as f:
    data = f.read().splitlines()

numbers = []
in_number = False
num_start = -1
num_end = -1
stars = {}
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
    n = int(data[y][x_start:x_end+1])
    if x_start > 0 and data[y][x_start - 1] == '*':
        if f'{x_start - 1}:{y}' in stars:
            stars[f'{x_start - 1}:{y}'].append(n)
        else:
            stars[f'{x_start - 1}:{y}'] = [n]
    if x_end < len(data[y])-1 and data[y][x_end + 1] == '*':
        if f'{x_end + 1}:{y}' in stars:
            stars[f'{x_end + 1}:{y}'].append(n)
        else:
            stars[f'{x_end + 1}:{y}'] = [n]
    if y > 0:
        for x in range(x_start, x_end+1):
            if data[y-1][x] != '.':
                if f'{x}:{y-1}' in stars:
                    stars[f'{x}:{y-1}'].append(n)
                else:
                    stars[f'{x}:{y-1}'] = [n]
        if x_start > 0:
            if data[y-1][x_start-1] != '.':
                if f'{x_start-1}:{y-1}' in stars:
                    stars[f'{x_start-1}:{y-1}'].append(n)
                else:
                    stars[f'{x_start-1}:{y-1}'] = [n]
        if x_end < len(data[y])-1:
            if data[y-1][x_end+1] != '.':
                if f'{x_end+1}:{y-1}' in stars:
                    stars[f'{x_end+1}:{y-1}'].append(n)
                else:
                    stars[f'{x_end+1}:{y-1}'] = [n]
    if y < len(data)-1:
        for x in range(x_start, x_end+1):
            if data[y+1][x] != '.':
                if f'{x}:{y+1}' in stars:
                    stars[f'{x}:{y+1}'].append(n)
                else:
                    stars[f'{x}:{y+1}'] = [n]
        if x_start > 0:
            if data[y+1][x_start-1] != '.':
                if f'{x_start-1}:{y+1}' in stars:
                    stars[f'{x_start-1}:{y+1}'].append(n)
                else:
                    stars[f'{x_start-1}:{y+1}'] = [n]
        if x_end < len(data[y])-1:
            if data[y+1][x_end+1] != '.':
                if f'{x_end+1}:{y+1}' in stars:
                    stars[f'{x_end+1}:{y+1}'].append(n)
                else:
                    stars[f'{x_end+1}:{y+1}'] = [n]



answer = 0
for k,v in stars.items():
    if len(v) == 2:
        answer += v[0]*v[1]
print(f'Part two: {answer}')