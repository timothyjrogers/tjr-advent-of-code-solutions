fs = require('fs');
data = fs.readFileSync('input.txt', 'utf-8');

var lines = data.split("\n");
var checksum1 = 0;
lines.forEach((line) => {
    var max = 0;
    var min = Number.MAX_VALUE;
    line.split("\t").forEach((item) => {
        var val = parseInt(item);
        max = Math.max(max, val);
        min = Math.min(min, val);
    })
    checksum1 += max - min;
});
console.log("Part one: " + checksum1);

var checksum2 = 0;
lines.forEach((line) => {
    var items = line.split("\t");
    for (var i = 0; i < items.length - 1; i++) {
        for (var j = i+1; j < items.length; j++) {
            var x = parseInt(items[i]);
            var y = parseInt(items[j]);
            if (x % y == 0) {
                checksum2 += x / y;
                break;
            } else if (y % x == 0) {
                checksum2 += y / x;
                break;
            }
        }
    }
});
console.log("Part two: " + checksum2);