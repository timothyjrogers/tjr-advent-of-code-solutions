const input = 349;

let lock = [0];
let cur = 0;
for (let i = 1; i <= 2017; i++) {
    for (let j = 0; j < input; j++) {
        cur++;
        if (cur > lock.length - 1) {
            cur = cur % lock.length;
        }
    }
    if (cur == lock.length - 1) {
        lock.push(i);
    } else {
        lock.splice(cur+1, 0, i);
    }
    cur++;
}
console.log(`Part one: ${lock[cur+1]}`);

let size = 1;
let pos = 0;
let res = 0;
for (let i = 1; i <= 50000000; i++) {
    let newPos = (pos + input) % size;
    newPos++;
    if (newPos == 1) {
        res = i;
    }
    size++;
    pos = newPos;
}
console.log(`Part two: ${res}`);