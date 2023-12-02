with open('input.txt', 'r') as f:
    data = f.read().splitlines()

limits_p1 = {'red': 12, 'green': 13, 'blue': 14}
sum1 = 0
sum2 = 0
for line in data:
    mins = {}
    valid = True
    parts = line.split(': ')
    game_id = parts[0].split(' ')[1]
    rounds = parts[1].split('; ')
    for rnd in rounds:
        draws = rnd.split(', ')
        for draw in draws:
            cubes = draw.split(' ')
            if cubes[1] not in mins or int(cubes[0]) > mins[cubes[1]]:
                mins[cubes[1]] = int(cubes[0])
            if int(cubes[0]) > limits_p1[cubes[1]]:
                valid = False
    if valid:
        sum1 += int(game_id)
    sum2 += mins['red'] * mins['green'] * mins['blue']

print(f'Part one: {sum1}')
print(f'Part two: {sum2}')