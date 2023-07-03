fs = require('fs');
data = fs.readFileSync('input.txt', 'utf-8');

let steps = data.trim().split(",");
let count = {};
count["s"] = 0;
count["n"] = 0;
count["ne"] = 0;
count["nw"] = 0;
count["se"]= 0;
count["sw"] = 0;

let max = 0;
steps.forEach(step => {
    switch (step) {
        case "n":
            if (count["s"] > 0) {
                count["s"]--;
            } else if (count["sw"] > 0) {
                count["nw"]++;
                count["sw"]--;
            } else if (count["se"] > 0) {
                count["ne"]++;
                count["se"]--;
            } else {
                count["n"]++;
            }
            break;
        case "s":
            if (count["n"] > 0) {
                count["n"]--;
            } else if (count["nw"] > 0) {
                count["sw"]++;
                count["nw"]--;
            } else if (count["ne"] > 0) {
                count["se"]++;
                count["ne"]--;
            } else {
                count["s"]++;
            }
            break;
        case "ne":
            if (count["sw"] > 0) {
                count["sw"]--;
            } else if (count["s"] > 0) {
                count["se"]++;
                count["s"]--;
            } else if (count["nw"] > 0) {
                count["n"]++;
                count["nw"]--;
            } else {
                count["ne"]++;
            }
            break;
        case "se":
            if (count["nw"] > 0) {
                count["nw"]--;
            } else if (count["n"] > 0) {
                count["ne"]++;
                count["n"]--;
            } else if (count["sw"] > 0) {
                count["s"]++;
                count["sw"]--;
            } else {
                count["se"]++;
            }
            break;
        case "nw":
            if (count["se"] > 0) {
                count["se"]--;
            } else if (count["s"] > 0) {
                count["sw"]++;
                count["s"]--;
            } else if (count["ne"] > 0) {
                count["n"]++;
                count["ne"]--;
            } else {
                count["nw"]++;
            }
            break;
        case "sw":
            if (count["ne"] > 0) {
                count["ne"]--;
            } else if (count["n"] > 0) {
                count["nw"]++;
                count["n"]--;
            } else if (count["se"] > 0) {
                count["s"]++;
                count["se"]--;
            } else {
                count["sw"]++;
            }
            break;
        default:
            break;
    }
    let dist = 0;
    for (const [key, value] of Object.entries(count)) {
        dist += value;
    }
    if (dist > max) {
        max = dist;
    }
})

let sum = 0;
for (const [key, value] of Object.entries(count)) {
    sum += value;
}
console.log(`Part one: ${sum}`);
console.log(`Part two: ${max}`);