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