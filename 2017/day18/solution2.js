fs = require('fs');
data = fs.readFileSync('input.txt', 'utf-8');

const steps = data.trim().split("\n");

let registers = {};
let registers0 = {};
let buffer0 = [];
let pc0 = 0;
let waiting0 = false;
let registers1 = {};
let buffer1 = [];
let pc1 = 0;
let waiting1 = false;

["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"].forEach(letter => {
    registers[letter] = 0;
    registers0[letter] = 0;
    registers1[letter] = 0;
})

while (pc0 < steps.length && pc1 < steps.length) {
    if (waiting0 && buffer1.length == 0 && waiting1 && buffer0.length == 0) {
        break;
    }
    step = steps[pc];
    let parts = step.split(" ");
    if (parts.length == 2) {
        let val = parseInt(parts[1]);
        if (isNaN(val)) {
            val = registers[parts[1]];
        }
        if (parts[0] == "snd") {
            //TODO
        } else {
            //TODO
        }
    } else if (parts.length == 3) {
        if (parts[0] == "jgz") {
            let val1 = parseInt(parts[1]);
            if (isNaN(val1)) {
                val1 = registers[parts[1]];
            }
            if (val1 > 0) {
                let val2 = parseInt(parts[2]);
                if (isNaN(val2)) {
                    val2 = registers[parts[2]];
                }
                pc += val2 - 1;
            }
        } else {
            let val = parseInt(parts[2]);
            if (isNaN(val)) {
                val = registers[parts[2]];
            }
            if (parts[0] == "set") {
                registers[parts[1]] = val;
            } else if (parts[0] == "add") {
                registers[parts[1]] += val;
            } else if (parts[0] == "mul") {
                registers[parts[1]] *= val;
            } else if (parts[0] == "mod") {
                registers[parts[1]] %= val;
            }
        }
    }
    pc++;
}