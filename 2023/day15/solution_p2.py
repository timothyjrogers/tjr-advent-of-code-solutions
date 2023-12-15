def hashFunction(s):
    cur = 0
    for ch in s:
        cur += ord(ch)
        cur *= 17
        cur %= 256
    return cur

with open('input.txt', 'r') as f:
    data = f.read().splitlines()

steps = data[0].split(',')
answer = 0
boxes_labels = []
boxes_lengths = []
for i in range(256):
    boxes_labels.append([])
    boxes_lengths.append([])

for step in steps:
    if '-' in step:
        label = step[0:len(step)-1]
        box = hashFunction(label)
        if label in boxes_labels[box]:
            index = boxes_labels[box].index(label)
            del boxes_labels[box][index]
            del boxes_lengths[box][index]
    else:
        parts = step.split('=')
        label = parts[0]
        focal_length = int(parts[1])
        box = hashFunction(label)
        if label in boxes_labels[box]:
            index = boxes_labels[box].index(label)
            boxes_labels[box][index] = label
            boxes_lengths[box][index] = focal_length
        else:
            boxes_labels[box].append(label)
            boxes_lengths[box].append(focal_length)

for index in range(len(boxes_labels)):
    for lens in range(len(boxes_labels[index])):
        power = index + 1
        power *= (lens + 1)
        power *= boxes_lengths[index][lens]
        answer += power

print(f'Part two: {answer}')