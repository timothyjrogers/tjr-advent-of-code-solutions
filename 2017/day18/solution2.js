fs = require('fs');
data = fs.readFileSync('input.txt', 'utf-8');

const steps = data.trim().split("\n");

let registers0 = {};
let buffer0 = [];
let pc0 = 0;
let waiting0 = false;
let terminated0 = false;
let registers1 = {};
let buffer1 = [];
let pc1 = 0;
let waiting1 = false;
let terminated1 = false;
let sends = 0;
["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"].forEach(letter => {
    registers0[letter] = 0;
    registers1[letter] = 0;
})
registers1["p"] = 1;

while (!terminated0 || !terminated1) {
    while (!terminated0) {
        let step = steps[pc0];
        let parts = step.split(" ");
        if (parts.length == 2) {
            if (parts[0] == "snd") {
                let val = parseInt(parts[1]);
                if (isNaN(val)) {
                    val = registers0[parts[1]];
                }
                buffer1.push(val);
            } else {
                if (buffer0.length > 0) {
                    registers0[parts[1]] = buffer0.shift();
                    waiting0 = false;
                } else {
                    if (!terminated1 && !(waiting1 && buffer1.length == 0)) {
                        waiting0 = true;
                    } else {
                        terminated0 = true;
                    }
                    break;
                }
            }
        } else if (parts.length == 3) {
            if (parts[0] == "jgz") {
                let val1 = parseInt(parts[1]);
                if (isNaN(val1)) {
                    val1 = registers0[parts[1]];
                }
                if (val1 > 0) {
                    let val2 = parseInt(parts[2]);
                    if (isNaN(val2)) {
                        val2 = registers0[parts[2]];
                    }
                    pc0 += val2 - 1;
                }
            } else {
                let val = parseInt(parts[2]);
                if (isNaN(val)) {
                    val = registers0[parts[2]];
                }
                if (parts[0] == "set") {
                    registers0[parts[1]] = val;
                } else if (parts[0] == "add") {
                    registers0[parts[1]] += val;
                } else if (parts[0] == "mul") {
                    registers0[parts[1]] *= val;
                } else if (parts[0] == "mod") {
                    registers0[parts[1]] %= val;
                }
            }
        }
        pc0++;
        if (pc0 >= steps.length) {
            terminated0 = true;
        }
    }
    while (!terminated1) {
        let step = steps[pc1];
        let parts = step.split(" ");
        if (parts.length == 2) {
            if (parts[0] == "snd") {
                let val = parseInt(parts[1]);
                if (isNaN(val)) {
                    val = registers1[parts[1]];
                }
                buffer0.push(val);
                sends++;
            } else {
                if (buffer1.length > 0) {
                    registers1[parts[1]] = buffer1.shift();
                    waiting1 = false;
                } else {
                    if (!terminated0 && !(waiting0 && buffer0.length == 0)) {
                        waiting1 = true;
                    } else {
                        terminated1 = true;
                    }
                    break;
                }
            }
        } else if (parts.length == 3) {
            if (parts[0] == "jgz") {
                let val1 = parseInt(parts[1]);
                if (isNaN(val1)) {
                    val1 = registers1[parts[1]];
                }
                if (val1 > 0) {
                    let val2 = parseInt(parts[2]);
                    if (isNaN(val2)) {
                        val2 = registers1[parts[2]];
                    }
                    pc1 += val2 - 1;
                }
            } else {
                let val = parseInt(parts[2]);
                if (isNaN(val)) {
                    val = registers1[parts[2]];
                }
                if (parts[0] == "set") {
                    registers1[parts[1]] = val;
                } else if (parts[0] == "add") {
                    registers1[parts[1]] += val;
                } else if (parts[0] == "mul") {
                    registers1[parts[1]] *= val;
                } else if (parts[0] == "mod") {
                    registers1[parts[1]] %= val;
                }
            }
        }
        pc1++;
        if (pc1 >= steps.length) {
            terminated1 = true;
        }
    }
}
console.log(`Part two: ${sends}`);