fs = require('fs');
data = fs.readFileSync('input.txt', 'utf-8')
var stream = data.trim();

var total = 0;

var curScore = 0;
var inGarbage = false;
var numGarbage = 0;
for (var i = 0; i < stream.length; i++) {
    if (stream[i] == "!") {
        i++;
    } else if (stream[i] == "{" && !inGarbage) {
        curScore++;
        total += curScore;
    } else if (stream[i] == "}" && !inGarbage) {
        curScore--;
    } else if (stream[i] == "<" && !inGarbage) {
        inGarbage = true;
    } else if (stream[i] == ">" && inGarbage) {
        inGarbage = false;
    } else if (inGarbage) {
        numGarbage++;
    }
}
console.log(`Part one: ${total}`);
console.log(`Part two: ${numGarbage}`);