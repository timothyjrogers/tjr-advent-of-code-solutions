fs = require('fs');
data = fs.readFileSync('input.txt', 'utf-8');

const steps = data.trim().split("\n");

let registers = {};
let rcv = 0;
let sound = 0;
let pc = 0;
while (pc < steps.length) {
    step = steps[pc];
    if (step.startsWith("snd")) {
        let parts = step.split(" ");
        let x = parseInt(parts[1]);
        if (isNaN(x)) {
            x = parts[1];
            if (registers[x]) {
                sound = registers[x];
            } else {
                registers[x] = 0;
                sound = 0;
            }
        } else {
            sound = x;
        }
    } else if (step.startsWith("set")) {
        let parts = step.split(" ");
        let y = parseInt(parts[2]);
        if (registers[parts[1]] == undefined) {
            registers[parts[1]] = 0;
        }
        if (isNaN(y)) {
            if (registers[parts[2]]) {
                registers[parts[1]] = registers[parts[2]];
            } else {
                registers[parts[1]] = 0;
                registers[parts[2]] = 0;
            }
        } else {
            registers[parts[1]] = y;
        }
    } else if (step.startsWith("add")) {
        let parts = step.split(" ");
        let y = parseInt(parts[2]);
        if (registers[parts[1]] == undefined) {
            registers[parts[1]] = 0;
        }
        if (isNaN(y)) {
            if (registers[parts[2]]) {
                registers[parts[1]] += registers[parts[2]];
            } else {
                registers[parts[2]] = 0;
            }
        } else {
            registers[parts[1]] += y;
        }
    } else if (step.startsWith("mult")) {
        let parts = step.split(" ");
        let y = parseInt(parts[2]);
        if (registers[parts[1]] == undefined) {
            registers[parts[1]] = 0;
        }
        if (isNaN(y)) {
            if (registers[parts[2]]) {
                registers[parts[1]] *= registers[parts[2]];
            } else {
                registers[parts[1]] = 0;
                registers[parts[2]] = 0;
            }
        } else {
            registers[parts[1]] *= y;
        }
    } else if (step.startsWith("mod")) {
        let parts = step.split(" ");
        let y = parseInt(parts[2]);
        if (registers[parts[1]] == undefined) {
            registers[parts[1]] = 0;
        }
        if (isNaN(y)) {
            if (registers[parts[2]]) {
                registers[parts[1]] %= registers[parts[2]];
            } else {
                registers[parts[1]] = 0;
                registers[parts[2]] = 0;
            }
        } else {
            registers[parts[1]] %= y;
        }
    } else if (step.startsWith("rcv")) {
        let parts = step.split(" ");
        if (registers[parts[1]]) {
            if (registers[parts[1]] != 0) {
                rcv = sound;
                console.log("Part one: " + rcv);
            }
        } else {
            registers[parts[1]] = 0;
        }
    } else if (step.startsWith("jgz")) {
        let parts = step.split(" ");
        if (registers[parts[1]]) {
            if (registers[parts[1]] != 0) {
                let y = parseInt(parts[2]);
                if (isNaN(y)) {
                    if (registers[parts[2]]) {
                        pc += registers[parts[2]] -1;
                    } else {
                        registers[parts[2]] = 0;
                    }
                } else {
                    pc += y - 1;
                }
            }
        } else {
            registers[parts[1]] = 0;
        }
    }
    pc++;
}