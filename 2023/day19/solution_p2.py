def parseWorfklow(workflow):
    s1 = workflow.split('{')
    name = s1[0]
    rules = s1[1][:-1].split(',')
    comparisons = []
    default = ('','','',rules[-1])
    del rules[-1]
    for rule in rules:
        s2 = rule.split(':')
        target = s2[1]
        field = s2[0][0]
        operator = s2[0][1]
        threshold = int(s2[0][2:])
        comparisons.append((field,operator,threshold,target))
    comparisons.append(default)
    return (name, comparisons)

def findAcceptableRanges(workflows, workflow, comp_index, cube):
    if workflow == 'R':
        return 0
    if workflow == 'A':
        return (cube['x'][1] - cube['x'][0] + 1) * (cube['m'][1] - cube['m'][0] + 1) * (cube['a'][1] - cube['a'][0] + 1) * (cube['s'][1] - cube['s'][0] + 1)
    total = 0
    comparison = workflows[workflow][comp_index]
    lcube = cube.copy()
    rcube = cube.copy()
    field = comparison[0]
    val = comparison[2]
    if field != '':
        if comparison[1] == '<':
            lcube[field] = (lcube[field][0], val-1)
            rcube[field] = (val, rcube[field][1])
        elif comparison[1] == '>':
            lcube[field] = (val+1, lcube[field][1])
            rcube[field] = (rcube[field][0], val)
        total += findAcceptableRanges(workflows, comparison[3], 0, lcube)
        total += findAcceptableRanges(workflows, workflow, comp_index + 1, rcube)
    else:
        total += findAcceptableRanges(workflows, comparison[3], 0, cube)
    return total

with open('input.txt', 'r') as f:
    data = f.read().split('\n\n')

workflows = {}
parts = data[1].split('\n')

for w in data[0].split('\n'):
    workflow = parseWorfklow(w)
    workflows[workflow[0]] = workflow[1]
workflows['A'] = []
workflows['R'] = []
answer = findAcceptableRanges(workflows, 'in', 0, {'x': [1,4000], 'm': [1,4000], 'a': [1,4000], 's': [1,4000]})
print(f'Part two: {answer}')