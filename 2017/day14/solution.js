let keystring = "ugkiagan";

function generateKnotHash(input) {
    var ascii = input.trim().split("").map(x => x.charCodeAt(0));
    ascii.push(17, 31, 73, 47, 23);
    loop = [];
    for (var i = 0; i < 256; i++) {
        loop.push(i);
    }

    current = 0;
    skip = 0;
    for (j = 0; j < 64; j++) {
        ascii.forEach(step => {
            var segment = []
            for (var i = 0; i < step; i++) {
                if (current + i < loop.length) {
                    segment.push(loop[current + i]);
                } else {
                    segment.push(loop[current + i - loop.length]);
                }
            }
            segment = segment.reverse();
            for (var i = 0; i < step; i++) {
                if (current + i < loop.length) {
                    loop[current + i] = segment[i];
                } else {
                    loop[current + i - loop.length] = segment[i];
                }
            }
            current += step + skip;
            if (current > loop.length) {
                current = current % loop.length;
            }
            skip++;
        });
    }

    var denseHash = [];
    for (var i = 0; i < 16; i++) {
        denseHash.push(loop.slice(i * 16, i * 16 + 16).reduce((a,b) => a ^ b));
    }

    var knotHash = "";
    denseHash.forEach(d => {
        let hash = d.toString(16);
        if (hash.length < 2) {
            hash = "0" + hash;
        }
        knotHash += hash;
    })
    return knotHash;
}

function hexToBin(hash) {
    let binHash = "";
    for (let i = 0; i < hash.length; i++) {
        let bit = parseInt(hash.charAt(i), 16);
        let binString = bit.toString(2);
        let prefix = "";
        for (let p = 0; p < 4 - binString.length; p++) {
            prefix += "0";
        }
        binHash += prefix + binString;
    }
    return binHash;
}

let grid = [];
for (let i = 0; i < 128; i++) {
    let init = keystring + "-" + i;
    let hash = generateKnotHash(init);
    grid.push(hexToBin(hash));
}

let used = 0;
grid.forEach(row => {
    for (let i = 0; i < row.length; i++) {
        let c = row.charAt(i);
        if (c == "1") {
            used++;
        }
    }
})
console.log(`Part one: ${used}`);

let groups = 0;
let visited = [];
for (let i = 0; i < 128; i++) {
    for (let j = 0; j < 128; j++) {
        if (!visited.includes(i + "," + j) && grid[i].charAt(j) == "1") {
            groups++;
            let queue = [];
            queue.push([i,j]);
            while (queue.length > 0) {
                let cur = queue.pop();
                if (visited.includes(cur[0] + "," + cur[1])) {
                    continue;
                }
                if (grid[cur[0]].charAt(cur[1]) != "1") {
                    continue;
                }
                visited.push(cur[0] + "," + cur[1]);
                if (cur[0] > 0) {
                    queue.push([cur[0]-1, cur[1]]);
                }
                if (cur[0] < 127) {
                    queue.push([cur[0]+1, cur[1]]);
                }
                if (cur[1] > 0) {
                    queue.push([cur[0], cur[1]-1]);
                }
                if (cur[1] < 127) {
                    queue.push([cur[0], cur[1]+1]);
                }
            }
        }
    }
}
console.log(`Part two: ${groups}`);