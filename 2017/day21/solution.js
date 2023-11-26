fs = require('fs');
data = fs.readFileSync('input.txt', 'utf-8');

/*Converts grid string into 2D array*/
function assemble(input) {
    let parts = input.split('/');
    let grid = [];
    for (let i = 0; i < parts.length; i++) {
        let row = parts[i].split();
        grid.push(row);
    }
    return grid;
}

/*Converts 2D array into grid string*/
function stringify(grid) {
    let gridstrings = [];
    for (let i = 0; i < grid.length; i++) {
        let gridstring = grid[i].join('');
        gridstrings.push(gridstring);
    }
    return gridstrings.join('/');
}

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
    for (let i = 0; i < grid.length/2; i++) {
        for (let j = 0; j < grid[i].length; j++) {
            let tmp = newGrid[i][j];
            newGrid[i][j] = newGrid[grid.length - 1 - i][j];
            newGrid[grid.length - 1 - i][j] = tmp;
        }
    }
    return newGrid;
}

const list = data.split("\n");
let rules = {}; //rule looks like .#/#. => .#./##./#.#
/*  For each input rule, generate all 9 flipped/rotated versions and map same rule*/
list.forEach(rule => {
    let parts = rule.split(" => ");
    let gridstring = parts[0];
    rules[gridstring] = parts[1];
    gridstring = stringify(transpose(assemble(gridstring)));
    rules[gridstring] = parts[1];
    gridstring = stringify(flip(assemble(gridstring)));
    rules[gridstring] = parts[1];
    gridstring = stringify(transpose(assemble(gridstring)));
    rules[gridstring] = parts[1];
    gridstring = stringify(flip(assemble(gridstring)));
    rules[gridstring] = parts[1];
    gridstring = stringify(transpose(assemble(gridstring)));
    rules[gridstring] = parts[1];
    gridstring = stringify(flip(assemble(gridstring)));
    rules[gridstring] = parts[1];
    gridstring = stringify(transpose(assemble(gridstring)));
    rules[gridstring] = parts[1];
    gridstring = stringify(flip(assemble(gridstring)));
    rules[gridstring] = parts[1];
})

//let imageString = ".#./..#/###";
let image = [[".","#","."], [".",".","#"], ["#","#","#"]];
//let image = [".","#",".",".",".","#","#","#","#"];
let image_size = 3;
for (let i = 0; i < 5; i++) {
    let newImagePieces = [];
    let xOffset = 0;
    let yOffset = 0;
    if (image_size % 2 == 0) {
        for (let xstart = 0; xstart < image_size; xstart += 2) {
            for (let ystart = 0; ystart < image_size; ystart += 2) {
                let grid = [];
                for (let y = ystart; y <= ystart + 2; y++) {
                    let row = []
                    for (let x = xstart; x <= xstart + 2; x++) {
                        row.push(image[y][x])
                    }
                    grid.push(row)
                }
            }
            let evolution = assemble(rules[stringify(grid)])
            newImagePieces.push(evolution)
        }
        
    } else if (image_size % 3 == 0) {

    }
    
}
