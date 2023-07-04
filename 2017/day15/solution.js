fs = require('fs');
data = fs.readFileSync('input.txt', 'utf-8');

let lines = data.trim().split("\n");

const aStart = parseInt(lines[0].split(" ")[4]);
const aFactor = 16807;
const bStart = parseInt(lines[1].split(" ")[4]);
const bFactor = 48271;
const divisor = 2147483647;

function nextVal(cur, factor, divisor) {
    return (cur * factor) % divisor;
}

function compareLow16(a, b) {
    return (a & 0xFFFF) == (b & 0xFFFF);
}

let matches = 0;
let aValue = aStart;
let bValue = bStart;
for (let i = 0; i < 40000000; i++) {
    aValue = nextVal(aValue, aFactor, divisor);
    bValue = nextVal(bValue, bFactor, divisor);
    if (compareLow16(aValue, bValue)) {
        matches++;
    }
}
console.log(`Part one: ${matches}`);


matches = 0;
aValue = aStart;
bValue = bStart;
for (let i = 0; i < 5000000; i++) {
    while (true) {
        aValue = nextVal(aValue, aFactor, divisor);
        if (aValue % 4 == 0) {
            break;
        }
    }
    while (true) {
        bValue = nextVal(bValue, bFactor, divisor);
        if (bValue % 8 == 0) {
            break;
        }
    }
    if (compareLow16(aValue, bValue)) {
        matches++;
    }
}
console.log(`Part two: ${matches}`);