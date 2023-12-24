from math import gcd
from math import lcm

with open('input.txt', 'r') as f:
    data = f.read().splitlines()

nodes = {}
for line in data:
    name, _ = line.split(' -> ')
    if name[0] == '&':
        nodes[name[1:]] = {'type': 'conjunction', 'targets': [], 'value': {}}
    elif name[0] == '%':
        nodes[name[1:]] = {'type': 'flipflop', 'targets': [], 'value': False}
    else:
        nodes[name] = {'type': 'broadcaster', 'targets': []}

for line in data:
    name, targets_string = line.split(' -> ')
    targets = targets_string.split(', ')
    if name[0] == '&' or name[0] == '%':
        name = name[1:]
    nodes[name]['targets'] = targets
    for target in targets:
        if target in nodes and nodes[target]['type'] == 'conjunction':
            nodes[target]['value'][name] = False

last_feeder = [node for node in nodes if 'rx' in nodes[node]['targets']][0]
tracker = {}
for node in nodes[last_feeder]['value']:
    tracker[node] = 0

count_low = 0
count_high = 0
for iteration in range(1, 10000):
    if all(tracker.values()):
        break
    signals = [('broadcaster', False, t) for t in nodes['broadcaster']['targets']]
    if iteration <= 1000: count_low += 1
    while len(signals) > 0:
        sender, signal, receiver = signals.pop(0)
        if iteration <= 1000:
            if signal:
                count_high += 1
            else:
                count_low += 1
        if receiver not in nodes: continue
        cur = nodes[receiver]
        if receiver == last_feeder and signal == True:
            tracker[sender] = iteration - tracker[sender]
        if cur['type'] == 'flipflop' and signal:
            continue
        nxt = False
        if cur['type'] == 'flipflop':
            cur['value'] = not cur['value']
            nxt = cur['value']
        elif cur['type'] == 'conjunction':
            cur['value'][sender] = signal
            nxt = not all(cur['value'].values())
        for t in cur['targets']:
            signals.append((receiver, nxt, t))
    if iteration == 1000:
        print(f'Part one: {count_low * count_high}')

print(f'Part two: {lcm(*tracker.values())}')