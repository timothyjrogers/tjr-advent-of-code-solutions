from functools import cache

with open('input.txt', 'r') as f:
    data = f.read().splitlines()

@cache
def checkCombinations(springs, groups):
    split_groups = []
    if len(groups) > 0:
        split_groups = [int(x) for x in groups.split(',')]
    if sum(split_groups) > len(springs):
        return 0
    if len(springs) == 0 and sum(split_groups) == 0:
        return 1
    elif len(springs) == 0 and sum(split_groups) > 0:
        return 0

    if springs[0] == '.':
        result = checkCombinations(springs[1:], groups)
        return result

    if springs[0] == '#':
        if len(split_groups) == 0:
            return 0
        cur = split_groups[0]
        remaining = split_groups[1:]
        for i in range(cur):
            if springs[i] == '.':
                return 0
        if len(springs) == cur and len(remaining) == 0:
            return 1
        if springs[cur] == '#':
            return 0
        return checkCombinations(springs[cur+1:], ','.join([str(x) for x in remaining]))

    if springs[0] == '?':
        p = 0
        q = 0
        for i in springs:
            if i == '?':
                q += 1
            elif i == '#':
                p += 1
        if p + q > sum(split_groups):
            return checkCombinations('#' + springs[1:], groups) + checkCombinations('.' + springs[1:], groups)
        else:
            return checkCombinations('#' + springs[1:], groups)


answer = 0
for line in data:
    parts = line.split()
    springs = parts[0]
    groups = parts[1]
    answer += checkCombinations(springs, groups)

print(f'Part one: {answer}')