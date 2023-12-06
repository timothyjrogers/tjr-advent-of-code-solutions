with open('input.txt', 'r') as f:
    data = f.read().splitlines()

seed_ranges = [int(x) for x in data[0].split(': ')[1].split()]
seeds = []
s = 0
while s < len(seed_ranges):
    seeds.append((seed_ranges[s], seed_ranges[s], seed_ranges[s+1]))
    s += 2
seeds.sort()
print(seeds)


def splitRanges(r1, r2):
    print(f'Source range: {r1}, Destination range: {r2}')
    new_ranges = []
    if r1[0] >= r2[0] and r1[0] + r1[2] <= r2[0] + r2[2]:
        print(f'Source range contained in destination range')
        new_ranges.append((r1[0], r2[1], r1[2]))
    elif r1[0] < r2[0] and r1[0] + r1[2] >= r2[0] and r1[0] + r1[2] <= r2[0] + r2[2]:
        print(f'Source range overlaps left end of destination range')
        new_range1 = (r1[0], r1[1], r2[0]-r1[0])
        new_range2 = (r2[0], r2[1], r1[0]+r1[2]-r2[0])
        new_ranges.append(new_range1)
        new_ranges.append(new_range2)
    elif r1[0] >= r2[0] and r1[0] <= r2[0]+r2[2] and r1[0]+r1[2] > r2[0]+r2[2]:
        print(f'Source range overlaps right end of destination range')
        new_range1 = (r1[0], r2[1], r2[0]+r2[2]-r1[0])
        new_range2 = (r2[0]+r2[2]+1, r1[1], r1[0]+r1[2]-r2[0]+r2[2])
        new_ranges.append(new_range1)
        new_ranges.append(new_range2)
    else:
        new_ranges.append(r1)
        print(f'Source range does not overlap destination range')
    return new_ranges

i = 1
cur_map = ''
while i < len(data):
    line = data[i]
    if line == '':
        cur_map = ''
        i += 1
    elif not line[0].isdigit():
        map_name = line.split(' ')[0]
        cur_map = map_name
        i += 1
    else:
        map_ranges = []
        while i < len(data) and data[i] != '':
            line = data[i]
            parts = line.split()
            dest = int(parts[0])
            src = int(parts[1])
            length = int(parts[2])
            map_ranges.append((src, dest, length))
            i += 1
        print(f'Destination ranges for {cur_map}: {map_ranges}')
        new_seed_ranges = []
        for r1 in seeds:
            for r2 in map_ranges:
                new_seed_ranges.extend(splitRanges(r1, r2))
        seeds = new_seed_ranges
        print(f'Updated seed ranges: {seeds}')

seeds.sort()
#print(seeds[0])