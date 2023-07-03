fs = require('fs');
data = fs.readFileSync('input.txt', 'utf-8');

var nodes = data.trim().split("\n");

var registers = {};

var wipMax = 0;
nodes.forEach((node) => {
    var parts = node.split(" ");
    var r1 = parts[0];
    var op = parts[1];
    var val = parseInt(parts[2]);
    var r2 = parts[4];
    var comparator = parts[5];
    var threshold = parseInt(parts[6]);

    if (!registers[r1]) {
        registers[r1] = 0;
    }
    if (!registers[r2]) {
        registers[r2] = 0;
    }
    if (compare(registers[r2], threshold, comparator)) {
        if (op == "inc") {
            registers[r1] += val;
        } else {
            registers[r1] -= val;
        }
        if (registers[r1] >= wipMax) {
            wipMax = registers[r1];
        }
    }
});

function compare(r, threshold, comparator) {
    // >, < , >=, <=, ==, !=
    if (comparator == ">") {
        return r > threshold;
    } else if (comparator == ">=") {
        return r >= threshold;
    } else if (comparator == "<") {
        return r < threshold;
    } else if (comparator == "<=") {
        return r <= threshold;
    } else if (comparator == "==") {
        return r == threshold;
    } else if (comparator == "!=") {
        return r != threshold;
    } else {
        return false;
    }
}

var max = 0;
for (const [key, value] of Object.entries(registers)) {
    if (value > max) {
        max = value;
    }
}
console.log(`Part one: ${max}`);
console.log(`Part two: ${wipMax}`);