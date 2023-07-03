const { group } = require('console');

fs = require('fs');
data = fs.readFileSync('input.txt', 'utf-8');

let pipes = data.trim().split("\n");

let adjacency = {};
pipes.forEach(pipe => {
    let parts = pipe.split(" <-> ");
    adjacency[parts[0]] = parts[1].split(", ");
});

let targets = Object.keys(adjacency).sort();
let groups = {}
let group0 = new Set();
while (targets.length) {
    let target = targets[0];
    groups[target] = new Set();
    let queue = [];
    let visited = new Set();
    let connected = false;
    queue.push(target);
    while (queue.length > 0) {
        let cur = queue.shift();
        if (cur == target) {
            connected = true;
        }
        if (visited.has(cur)) {
            continue;
        }
        visited.add(cur);
        adjacency[cur].forEach(node => {
            queue.push(node);
        });
    }
    if (connected) {
        visited.forEach(node => {
            let index = targets.indexOf(node);
            targets.splice(index, 1);
            groups[target].add(node);
        });
    }
}
console.log(`Part one: ${groups[0].size}`);
console.log(`Part two: ${Object.keys(groups).length}`);