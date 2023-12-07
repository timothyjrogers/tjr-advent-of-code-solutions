with open('input.txt', 'r') as f:
    data = f.read().splitlines()

hands = []
for line in data:
    parts = line.split()
    hand = (parts[0], int(parts[1]))
    hands.append(hand)

def handType(hand):
    counts = {}
    for card in hand[0]:
        if card in counts:
            counts[card] += 1
        else:
            counts[card] = 1
    if len(counts) == 5:
        return 'high card'
    elif len(counts) == 4:
        return 'one pair'
    elif len(counts) == 3:
        count_counts = {}
        for v in counts.values():
            if v in count_counts:
                count_counts[v] += 1
            else:
                count_counts[v] = 1
        if 3 in count_counts:
            return 'three of a kind'
        else:
            return 'two pair'
    elif len(counts) == 2:
        count_counts = {}
        for v in counts.values():
            if v in count_counts:
                count_counts[v] += 1
            else:
                count_counts[v] = 1
        if 4 in count_counts:
            return 'four of a kind'
        else:
            return 'full house'
    else:
        return 'five of a kind'

def cardRank(card):
    ranks = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2']
    rank = ranks.index(card)
    return rank

labeled_hands = {}
for hand in hands:
    label = handType(hand)
    if label in labeled_hands:
        labeled_hands[label].append(hand)
    else:
        labeled_hands[label] = []
        labeled_hands[label].append(hand)

rank = 1
hand_rank_orders = ['high card', 'one pair', 'two pair', 'three of a kind', 'full house', 'four of a kind', 'five of a kind']
card_alphabet = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2']
card_alphabet.reverse()
result = 0
for hand_rank in hand_rank_orders:
    if hand_rank not in labeled_hands:
        continue
    cur_hands = labeled_hands[hand_rank]
    cur_hands.sort(key = lambda x: [card_alphabet.index(c) for c in x[0]])
    for cur_hand in cur_hands:
        result += rank * cur_hand[1]
        rank += 1

print(f'Part one: {result}')