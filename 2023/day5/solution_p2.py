with open('input.txt', 'r') as f:
    data = f.read().splitlines()


maps = { "seed-to-soil": [], "soil-to-fertilizer": [], "fertilizer-to-water": [], "water-to-light": [], "light-to-temperature": [], "temperature-to-humidity": [], "humidity-to-location": []} 

i = 1
cur_map = ''
while i < len(data):
    line = data[i]
    if line == '':
        cur_map = ''
    elif not line[0].isdigit():
        map_name = line.split(' ')[0]
        cur_map = map_name
    else:
        parts = line.split()
        dest = int(parts[0])
        src = int(parts[1])
        length = int(parts[2])
        maps[cur_map].append((src, dest, length)) 
    i += 1

def findValInRanges(val, ranges):
    for r in ranges:
        if r[0] <= val and val < r[0]+r[2]:
            return r[1] + (val - r[0])
    return val


def findLocation(maps, seed):
    min_location = 1000000000
    soil = findValInRanges(seed, maps['seed-to-soil'])
    fertilizer = findValInRanges(soil, maps['soil-to-fertilizer'])
    water = findValInRanges(fertilizer, maps['fertilizer-to-water'])
    light = findValInRanges(water, maps['water-to-light'])
    temperature = findValInRanges(light, maps['light-to-temperature'])
    humidity = findValInRanges(temperature, maps['temperature-to-humidity'])
    location = findValInRanges(humidity, maps['humidity-to-location'])
    if location < min_location:
        min_location = location
    return min_location

seed_ranges = [int(x) for x in data[0].split(': ')[1].split()]
n = 0
answer = 1000000000
while n < len(seed_ranges):
    print(n)
    for i in range(seed_ranges[n+1]):
        loc = findLocation(maps, seed_ranges[n]+i)
        if loc < answer:
            answer = loc
    n += 2

print(f'Part one: {answer}')