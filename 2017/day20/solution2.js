fs = require('fs');
data = fs.readFileSync('input.txt', 'utf-8');

const list = data.split("\n");

particles = [];
list.forEach(particle => {
    let p = {};
    let vectors = particle.split(", ");
    p["p"] = vectors[0].substring(3, vectors[0].length-1).split(",").map(x => parseInt(x));
    p["v"] = vectors[1].substring(3, vectors[1].length-1).split(",").map(x => parseInt(x));
    p["a"] = vectors[2].substring(3, vectors[2].length-1).split(",").map(x => parseInt(x));
    particles.push(p);
});

for (let i = 0; i < 100; i++) {
    for (let j = 0; j < particles.length; j++) {
        let particle = particles[j];
        for (let k = 0; k < 3; k++) {
            particle["v"][k] += particle["a"][k];
            particle["p"][k] += particle["v"][k];
        }
        particles[j] = particle;
    }
    particles.forEach((particle, ind) => {
        particles.forEach((p, index) => {
            if (ind != index && particle["p"].join(",") == p["p"].join(",")) {
                particles[index]["collided"] = true;; 
            }
        });
    });
    particles.forEach((particle, index) => {
        if (particle["collided"]) {
            particles.splice(index, 1);
        }
    });
}
console.log(`Part two: ${particles.length}`);