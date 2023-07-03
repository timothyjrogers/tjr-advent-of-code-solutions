fs = require('fs');
data = fs.readFileSync('input.txt', 'utf-8');

var lines = data.split("\n").map(x => parseInt(x));
var steps = 0;
var index = 0;
while (index < lines.length && index >= 0) {
    steps++;
    var instr = lines[index];
    var j = lines[index];
    lines[index] = lines[index] + 1;
    index += j;
}
console.log("Part one: " + steps);

lines = data.split("\n").map(x => parseInt(x));
steps = 0;
index = 0;
while (index < lines.length && index >= 0) {
    steps++;
    var instr = lines[index];
    var j = lines[index];
    if (j >= 3) {
        lines[index] = lines[index] - 1;
    } else {
        lines[index] = lines[index] + 1;
    }
    index += j;
}
console.log("Part two: " + steps);