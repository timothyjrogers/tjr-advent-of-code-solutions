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
console.log(depths);
let severity = 0;
console.log(layers);
for (var i = 0; i <= numLayers; i++) {
    pos++;
    console.log(pos);
    if (layers[pos] != undefined) {
        if (layers[pos] == 0) {
            console.log(`Caught in layer ${pos}, depth ${depths[pos]}`);
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
    console.log(layers);
}
console.log(`Part one: ${severity}`);