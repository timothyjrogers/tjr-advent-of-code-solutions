fs = require('fs');
data = fs.readFileSync('input.txt', 'utf-8');

const list = data.split("\n");
let rules = {};
list.forEach(rule => {
    let parts = rule.split(" => ");
    rules[parts[0]] = parts[1];
})

function transpose(grid) { //Assumes square grid
    let newGrid = grid;
    for (let i = 0; i < newGrid.length; i++) {
        for (let j = i; j < newGrid.length; j++) {
            let tmp = newGrid[i][j];
            newGrid[i][j] = newGrid[j][i];
            newGrid[j][i] = tmp;
        }
    }
    return newGrid;
}

function flip(grid) {
    let newGrid = grid;
    for (let i = 0; i < newGrid.length; i++) {
        newGrid[i] = newGrid[i].reverse();
    }
    return newGrid;
}

let image = [".#.", "..#", "###"];
for (let i = 0; i < 5; i++) {

}