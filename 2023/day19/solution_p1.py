with open('input.txt', 'r') as f:
    data = f.read().split('\n\n')

workflows = {}
parts = data[1].split('\n')

def parseWorfklow(workflow):
    s1 = workflow.split('{')
    name = s1[0]
    rules = s1[1][:-1].split(',')
    comparisons = []
    default = rules[-1]
    del rules[-1]
    for rule in rules:
        s2 = rule.split(':')
        target = s2[1]
        field = s2[0][0]
        operator = s2[0][1]
        threshold = int(s2[0][2:])
        comparisons.append((field,operator,threshold,target))
    return (name, comparisons, default)

for w in data[0].split('\n'):
    workflow = parseWorfklow(w)
    workflows[workflow[0]] = (workflow[1], workflow[2])

answer = 0
for part in parts:
    ratings = {}
    total = 0
    for x in part[1:-1].split(','):
        val = int(x.split('=')[1])
        total += val
        ratings[x.split('=')[0]] = val
    
    cur_workflow = 'in'

    accepted = True
    while True:
        next_workflow = ''
        for comparison in workflows[cur_workflow][0]:
            val = ratings[comparison[0]]
            if comparison[1] == '<':
                if val < comparison[2]:
                    next_workflow = comparison[3]
                    break
            elif comparison[1] == '>':
                if val > comparison[2]:
                    next_workflow = comparison[3]
                    break
        if next_workflow == '':
            next_workflow = workflows[cur_workflow][1]
        if next_workflow == 'R':
            accepted = False
            break
        elif next_workflow == 'A':
            break
        cur_workflow = next_workflow
    if accepted:
        answer += total

print(f'Part one: {answer}')