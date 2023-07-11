fs = require('fs');
data = fs.readFileSync('input.txt', 'utf-8');

const grid = data.split("\n");

let solution = "";
let y = 1;
let x = grid[0].indexOf("|");
let dir = 2; // 0 = UP; 1 = RIGHT; 2 = DOWN; 3 = LEFT;
let steps = 1;
while (true) {
    steps++;
    if (dir == 0) {
        y--;
    } else if (dir == 1) {
        x++;
    } else if (dir == 2) {
        y++;
    } else if (dir == 3) {
        x--;
    }
    let val = grid[y].charAt(x);
    if (val == " ") {
        break;
    }
    if (val == "+") { //change direction
        if (dir == 0 || dir == 2) { //change to left/right
            if (grid[y].charAt(x-1) != " ") {
                dir = 3;
            } else {
                dir = 1;
            }
        } else { //change to up/down
            if (grid[y-1].charAt(x) != " ") {
                dir = 0;
            } else {
                dir = 2;
            }
        }
    } else if (val != "|" && val != "-") {
        solution += val;
    }
}
console.log(`Part one: ${solution}`);
console.log(`Part two: ${steps}`);