use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    let fname = "../data/input.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let estimate = vec[0].parse::<u32>().unwrap();
    let mut ids: HashMap<u32, u32> = HashMap::new();
    for id in vec[1].split(',') {
        let n;
        if id != "x" {
            n = id.parse::<u32>().unwrap();
        } else {
            continue;
        }
        let mut cur = n;
        while cur < estimate { cur = cur + n };
        ids.insert(n, cur);
    }

    let mut best = u32::MAX;
    let mut res = 0;
    for (id, time) in &ids {
        if time < &best {
            best = *time;
            res = id * (time - estimate);
        }
    }
    println!("{}", res);
}