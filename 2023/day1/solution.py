with open('input.txt', 'r') as f:
    data = f.read().splitlines()

sum1 = 0
data_parsed = []
sum2 = 0
# word: (numeral, length)
digit_words = {'one': ('1',3), 'two': ('2',3), 'three': ('3',5), 'four': ('4',4), 'five': ('5',4), 'six': ('6',3), 'seven': ('7',5), 'eight': ('8',5), 'nine': ('9',4)}
for line in data:
    digits = [x for x in line if x.isdigit()]
    if len(digits) > 1:
        sum1  += int(digits[0] + digits[len(digits)-1])
    
    parsed_line = line
    for k,v in digit_words.items():
        index = parsed_line.find(k)
        while index > -1:
            parsed_line = parsed_line[0:index] + v[0] + parsed_line[index+v[1]:len(parsed_line)]
            index = parsed_line.find(k)
    print(f'{parsed_line} -> {digits[0]}, {digits[len(digits)-1]}')
    digits = [x for x in parsed_line if x.isdigit()]
    sum2  += int(digits[0] + digits[len(digits)-1])

print(f'Part one: {sum1}')
print(f'Part two: {sum2}')