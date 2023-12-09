with open('input.txt', 'r') as f:
    data = f.read().splitlines()

result = 0

for line in data:
    points = [int(x) for x in line.split()]
    generations = [points]
    while True:
        next_generation = []
        for i in range(len(generations[-1]) - 1):
            diff = generations[-1][i+1] - generations[-1][i]
            next_generation.append(diff)
        generations.append(next_generation)
        if len(set(next_generation)) == 1:
            break
    for i in range(1, len(generations)):
        diff = generations[len(generations) - i][-1]
        generations[len(generations) - 1 - i].append(generations[len(generations) - 1 - i][-1] + diff)
    result += generations[0][-1]

print(f'Part one: {result}')