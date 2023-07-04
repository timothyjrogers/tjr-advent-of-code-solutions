fs = require('fs');
data = fs.readFileSync('input.txt', 'utf-8');

const steps = data.trim().split(",");

let programs = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p"];
const initial = programs.join("");
let iterations = 0;
for (let k = 0; k < 1000000000; k++) {
    steps.forEach(step => {
        let parts, p1, p2, tmp;
        switch (step.charAt(0)) {
            case "s":
                let newPrograms = [];
                let threshold = parseInt(step.substring(1));
                for (let i = programs.length - threshold; i < programs.length; i++) {
                    newPrograms.push(programs[i]);
                }
                for (let i = 0; i < programs.length - threshold; i++) {
                    newPrograms.push(programs[i]);
                }
                programs = newPrograms;
                break;
            case "x":
                parts = step.substring(1).split("/");
                p1 = parseInt(parts[0]);
                p2 = parseInt(parts[1]);
                tmp = programs[p1];
                programs[p1] = programs[p2];
                programs[p2] = tmp;
                break;
            case "p":
                parts = step.substring(1).split("/");
                p1 = programs.indexOf(parts[0]);
                p2 = programs.indexOf(parts[1]);
                tmp = programs[p1];
                programs[p1] = programs[p2];
                programs[p2] = tmp;
                break;
            default:
                break;
        }
    })
    if (k == 0) {
        console.log(`Part one: ${programs.join("")}`);
    }
    iterations++;
    if (programs.join("") == initial) {
        programs = initial.split("");
        k = 1000000000 - (1000000000 % iterations);
    }
}

console.log(`Part two: ${programs.join("")}`);