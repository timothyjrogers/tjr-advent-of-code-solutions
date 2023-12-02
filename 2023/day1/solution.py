with open('input.txt', 'r') as f:
    data = f.read().splitlines()

sum1 = 0
sum2 = 0
# word: (modified word, length); inject numeral into word to avoid edge cases like sevenine
digit_words = {'one': ('on1e',3), 'two': ('tw2o',3), 'three': ('thre3e',5), 'four': ('fou4r',4), 'five': ('fiv5e',4), 'six': ('si6x',3), 'seven': ('seve7n',5), 'eight': ('eigh8t',5), 'nine': ('nin9e',4)}
for line in data:
    digits = [x for x in line if x.isdigit()]
    if len(digits) > 0:
        sum1  += int(digits[0] + digits[-1])
    
    parsed_line = line
    for k,v in digit_words.items():
        index = parsed_line.find(k)
        while index > -1:
            parsed_line = parsed_line[0:index] + v[0] + parsed_line[index+v[1]:len(parsed_line)]
            index = parsed_line.find(k)
    digits = [x for x in parsed_line if x.isdigit()]
    if len(digits) > 0:
        sum2  += int(digits[0] + digits[-1])    

print(f'Part one: {sum1}')
print(f'Part two: {sum2}')