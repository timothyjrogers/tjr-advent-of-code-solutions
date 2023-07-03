fs = require('fs');
data = fs.readFileSync('input.txt', 'utf-8');

var nodes = data.trim().split("\t").map(x => parseInt(x));

var seen = {}
var steps = 0;
while (true) {
    steps++;
    var max = 0;
    var index = 0;
    for (var i = 0; i < nodes.length; i++) {
        if (nodes[i] > max) {
            max = nodes[i];
            index = i;
        }
    }
    nodes[index] = 0;
    index++;
    if (index == nodes.length) {
        index = 0;
    }
    for (var i = 0; i < max; i++) {
        nodes[index]++;
        index++;
        if (index == nodes.length) {
            index = 0;
        }
    }
    var s = nodes.join(",");
    if (seen[s]) {
        console.log("Part one: " + steps);
        console.log("Part two: " + (steps - seen[s]));
        break;
    }
    seen[s] = steps;
}