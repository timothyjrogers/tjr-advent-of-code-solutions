var fs = require('fs');
var data = fs.readFileSync('input.txt', 'utf-8');

var solution1 = 0;
for (var i = 0; i < data.length-1; i++) {
    if (data[i] == data[i+1]) {
        solution1 += parseInt(data[i]);
    }
}
if (data[data.length-1] == data[0]) {
    solution1 += parseInt(data[data.length-1]);
}

console.log("Part one: " + solution1);

var solution2 = 0;
var offset = data.length/2;
for (var i = 0; i < data.length-1; i++) {
    var index = (i + offset) % data.length;
    if (data[i] == data[index]) {
        solution2 += parseInt(data[i]);
    }
}

if (data[data.length-1] == data[(data.length-1 + offset) % data.length]) {
    solution2 += parseInt(data[data.length-1]);
}

console.log("Part two: " + solution2);