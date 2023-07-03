fs = require('fs');
data = fs.readFileSync('input.txt', 'utf-8');

var nodes = data.trim().split("\n");

var adj = {};
var refs = {};
var weights = {};
nodes.forEach((node) => {
    var split1 = node.split(" -> ");
    var name = split1[0].split(" ")[0];
    var weight = split1[0].split(" ")[1];
    weights[name] = parseInt(weight.substring(1, weight.length-1));
    if (split1.length == 1) {
        adj[name] = [];
    } else {
        var subprograms = split1[1].split(", ");
        adj[name] = subprograms;
        subprograms.forEach((s) => {
            if (refs[s]) {
                refs[s] = refs[s] + 1;
            } else {
                refs[s] = 1;
            }
        });
    }
});

var root = "";
for (const [key, value] of Object.entries(adj)) {
    if (refs[key] == undefined) {
        console.log(`Part one: ${key}`);
        root = key;
        break;
    }
}

function calculateWeight(adj, weights, root) {
    var w = weights[root];
    adj[root].forEach((node) => {
        w += calculateWeight(adj, weights, node);
    })
    return w;
}

function getLeastFrequent(list) {
    var count = {};
    list.forEach((l) => {
        if (count[l]) {
            count[l]++;
        } else {
            count[l] = 1;
        }
    });
    var minv = Number.MAX_VALUE;
    var mins = 0;
    for (const [key, value] of Object.entries(count)) {
        if (value < minv) {
            minv = value;
            mins = key;
        }
    }
    return mins;
}

function getMostFrequent(list) {
    var count = {};
    list.forEach((l) => {
        if (count[l]) {
            count[l]++;
        } else {
            count[l] = 1;
        }
    });
    var maxv = 0;
    var maxs = 0;
    for (const [key, value] of Object.entries(count)) {
        if (value > maxv) {
            maxv = value;
            maxs = key;
        }
    }
    return maxs;
}

function calculateImbalance(adj, weights, root) {
    if (adj[root].length == 0) {
        return 
    }
    var child_weights = []
    adj[root].forEach((node) => {
        child_weights.push(calculateWeight(adj, weights, node));
    });
    var child_weights_unique = new Set(child_weights);
    if (child_weights_unique.size == 1) {
        return;
    }
    var off_child_weight = getLeastFrequent(child_weights);
    var off_child = "";
    adj[root].forEach((child) => {
        if (off_child_weight == calculateWeight(adj, weights, child)) {
            off_child = child;
        }
    });
    var child_imbalance = calculateImbalance(adj, weights, off_child);
    if (child_imbalance) {
        return child_imbalance;
    } else {
        var good_child_weight = getMostFrequent(child_weights);
        return weights[off_child] + (good_child_weight - off_child_weight);
    }
}

var imbalance = calculateImbalance(adj, weights, root);
console.log(`Part two: ${imbalance}`);