var input = 289326;

var maxCorner = input;
var sideLength = 0;
while (true) {
    if (isOddSquare(maxCorner)) {
        sideLength = Math.sqrt(maxCorner);
        break;
    }
    maxCorner++;
}

while (!(input <= maxCorner && input >= maxCorner - sideLength + 1)) {
    maxCorner -= sideLength - 1;
}

var minCorner = (maxCorner - sideLength + 1)
var midPoint = minCorner + (maxCorner - minCorner)/2;
console.log("Part one: " + Math.floor(sideLength/2) + Math.abs(midPoint - input));

var x = 0
var y = 0
var xd = 1;
var yd = 0;
grid = {};
grid[x + "," + y] = 1;
console.log(grid[x + "," + y]);
while (true) {
    if (Math.abs(x) == Math.abs(y)) {
        if (x > y || (x == 0 && y == 0)) {
            x += 1;
            var val = sumNeighbors(grid, x, y);
            if (val > input) {
                console.log("Part two: " + val);
                break;
            }
            grid[x + "," + y] = val;
            xd = 0;
            yd = 1;
        } else if (x > 0 && x == y) {
            xd = -1;
            yd = 0;
        } else if (x < y) {
            xd = 0;
            yd = -1;
        } else if (x < 0 & x == y){
            xd = 1;
            yd = 0;
        }
    }
    x += xd;
    y += yd;
    var val = sumNeighbors(grid, x, y);
    var coord = x + "," + y;
    if (val > input) {
        console.log("Part two: " + val);
        break;
    }
    grid[coord] = val;
}

function isOddSquare(x) {
    return maxCorner % 2 == 1 && Math.sqrt(maxCorner) === Math.round(Math.sqrt(maxCorner));
}

function sumNeighbors(grid, x, y) {
    var val = 0;
    for (var xn = -1; xn <= 1; xn++) {
        for (var yn = -1; yn <= 1; yn++) {
            if (xn == 0 && yn == 0) {
                continue;
            }
            var coord = (x+xn) + "," + (y+yn);
            console.log(coord + ": " + grid[coord]);
            if (grid[coord]) {
                val += grid[coord];
            }
        }
    }
    console.log("VAL = " + val);
    return val;
}