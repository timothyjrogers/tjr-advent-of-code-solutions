fs = require('fs');
data = fs.readFileSync('input.txt', 'utf-8');

var steps = data.trim().split(",").map(x => parseInt(x));

var loop = [];
for (var i = 0; i < 256; i++) {
    loop.push(i);
}

var current = 0;
var skip = 0;
steps.forEach(step => {
    var segment = []
    for (var i = 0; i < step; i++) {
        if (current + i < loop.length) {
            segment.push(loop[current + i]);
        } else {
            segment.push(loop[current + i - loop.length]);
        }
    }
    segment = segment.reverse();
    for (var i = 0; i < step; i++) {
        if (current + i < loop.length) {
            loop[current + i] = segment[i];
        } else {
            loop[current + i - loop.length] = segment[i];
        }
    }
    current += step + skip;
    if (current > loop.length) {
        current = current % loop.length;
    }
    skip++;
});

console.log(`Part one: ${loop[0]} * ${loop[1]} = ${loop[0] * loop[1]}`);

var ascii = data.trim().split("").map(x => x.charCodeAt(0));
ascii.push(17, 31, 73, 47, 23);
loop = [];
for (var i = 0; i < 256; i++) {
    loop.push(i);
}

current = 0;
skip = 0;
for (j = 0; j < 64; j++) {
    ascii.forEach(step => {
        var segment = []
        for (var i = 0; i < step; i++) {
            if (current + i < loop.length) {
                segment.push(loop[current + i]);
            } else {
                segment.push(loop[current + i - loop.length]);
            }
        }
        segment = segment.reverse();
        for (var i = 0; i < step; i++) {
            if (current + i < loop.length) {
                loop[current + i] = segment[i];
            } else {
                loop[current + i - loop.length] = segment[i];
            }
        }
        current += step + skip;
        if (current > loop.length) {
            current = current % loop.length;
        }
        skip++;
    });
}

var denseHash = [];
for (var i = 0; i < 16; i++) {
    denseHash.push(loop.slice(i * 16, i * 16 + 16).reduce((a,b) => a ^ b));
}

var knotHash = "";
denseHash.forEach(d => {
    var hex = d.toString(16);
    if (hex.length == 1) {
        hex = "0" + hex;
    }
    knotHash += hex;
})
console.log(`Part two: ${knotHash}`);