with open('input.txt', 'r') as f:
    data = f.read().splitlines()

steps = data[0].split(',')
answer = 0
for step in steps:
    cur = 0
    for ch in step:
        cur += ord(ch)
        cur *= 17
        cur %= 256
    answer += cur

print(f'Part one: {answer}')