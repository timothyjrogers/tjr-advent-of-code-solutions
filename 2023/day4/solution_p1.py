with open('input.txt', 'r') as f:
    data = f.read().splitlines()

answer = 0
for line in data:
    parts = line.split(' | ')
    nums = parts[1].split(' ')
    winners = parts[0].split(': ')[1].split(' ')
    overlap = [x for x in nums if x in winners and x != '']
    if len(overlap) > 0:
        answer += 2**(len(overlap)-1)

print(f'Part one: {answer}')