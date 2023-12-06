with open('input.txt', 'r') as f:
    data = f.read().splitlines()

answer = 0
cards = {}
card_max = 0
memo = {}
for line in data:
    parts = line.split(' | ')
    nums = parts[1].split()
    winners = parts[0].split(': ')[1].split()
    card_num = int(parts[0].split(': ')[0].split()[1])
    overlap = [x for x in nums if x in winners]
    result = []
    for i in range(1,len(overlap)+1):
        result.append(card_num + i)
    memo[card_num] = result
    cards[card_num] = 1
    if card_num > card_max:
        card_max = card_num

for i in range(1, card_max):
    num_cards = cards[i]
    for card in memo[i]:
        cards[card] += num_cards

for v in cards.values():
    answer += v
print(f'Part two: {answer}')