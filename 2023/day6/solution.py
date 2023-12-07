with open('input.txt', 'r') as f:
    data = f.read().splitlines()

races = []
times = data[0].split()[1:]
distances = data[1].split()[1:]
for i in range(len(times)):
    races.append((int(times[i]),int(distances[i])))


result1 = 1
for race in races:
    wins = 0
    for x in range(race[0]):
        if x * (race[0] - x) > race[1]:
            wins += 1
    result1 *= wins

print(f'Part one: {result1}')

wins2 = 0
super_time = int(''.join(times))
super_distance = int(''.join(distances))
for x in range(super_time):
    wins = 0
    if x * (super_time - x) > super_distance:
        wins2 += 1

print(f'Part two: {wins2}')