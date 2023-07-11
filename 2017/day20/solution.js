fs = require('fs');
data = fs.readFileSync('input.txt', 'utf-8');

const list = data.split("\n");

particles = [];
list.forEach(particle => {
    let vectors = particle.split(", ");
    let magnitudes = [];
    vectors.forEach(vector => {
        let vals = vector.substring(3, vector.length-1).split(",");
        let init = 0;
        let magnitude = vals.reduce((a, c) => a + Math.abs(parseInt(c)), init);
        magnitudes.push(magnitude);  
    })
    particles.push(magnitudes);
});

let slowestIndex = -1;
let slowestParticle = [Number.MAX_VALUE, Number.MAX_VALUE, Number.MAX_VALUE];
for (let i = 0; i < particles.length; i++) {
    if (particles[i][2] < slowestParticle[2]) {
        slowestParticle = particles[i];
        slowestIndex = i;
    } else if (particles[i][2] == slowestParticle[2]) {
        if (particles[i][1] < slowestParticle[1]) {
            slowestParticle = particles[i];
            slowestIndex = i;
        } else if (particles[i][1] == slowestParticle[1]) {
            if (particles[i][0] < slowestParticle[0]) {
                slowestParticle = particles[i];
                slowestIndex = i;
            }
        }
    }
}
console.log(`Part one: ${slowestIndex}`);