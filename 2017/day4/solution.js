fs = require('fs');
data = fs.readFileSync('input.txt', 'utf-8');

var lines = data.split("\n");

var validPhrases1 = 0;
var validPhrases2 = 0;
lines.forEach((line) => {
    var parts = line.split(" ");
    var unique = new Set(parts);
    if (parts.length == unique.size) {
        validPhrases1++;
    }
    var seenSorted = {}
    var valid = true;
    parts.forEach((part) => {
        var sorted = part.split("").sort().join("");
        if (seenSorted[sorted]) {
            valid = false;
        }
        seenSorted[sorted] = true;
    });
    if (valid && parts.length == unique.size) {
        validPhrases2++;
    }
});
console.log("Part one: " + validPhrases1);
console.log("Part two: " + validPhrases2);