fs = require('fs');
data = fs.readFileSync('input.txt', 'utf-8');

let lines = data.trim().split("\n");

let depths = {};
let directions = {};
let numLayers = 0;
lines.forEach(line => {
    let parts = line.split(": ");
    depths[parts[0]] = parseInt(parts[1]);
    directions[parts[0]] = 1;
    numLayers = parseInt(parts[0]);
});
let layers = {};
Object.keys(depths).forEach(l => {
    layers[l] = 0;
});
let pos = -1;
let severity = 0;
for (var i = 0; i <= numLayers; i++) {
    pos++;
    if (layers[pos] != undefined) {
        if (layers[pos] == 0) {
            severity += pos * depths[pos];
        }
    }
    for (var j = 0; j <= numLayers; j++) {
        if (layers[j] != undefined) {
            layers[j] += directions[j] * 1;
            if (layers[j] >= depths[j]) {
                layers[j] -= 2;
                directions[j] *= -1;
            } else if (layers[j] < 0) {
                layers[j] = 1;
                directions[j] = 1;
            }
        }
    }
}
console.log(`Part one: ${severity}`);

let delay = 0;
while (true) {
    let caught = false;
    for (let i = 0; i <= numLayers; i++) {
        if (depths[i]) {
            let offset = (delay + i) % ((depths[i] - 1) * 2);
            if (offset == 0) {
                caught = true;
                break;
            }
        }
    }
    if (!caught) {
        console.log(`Part two: ${delay}`);
        break;
    }
    delay++;
}