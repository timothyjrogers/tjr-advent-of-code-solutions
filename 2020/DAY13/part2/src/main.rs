use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    let fname = "../data/input.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut ids: HashMap<u32, u32> = HashMap::new();
    let mut index = 0;
    let mut max: u64 = 0;
    for id in vec[1].split(',') {
        let n;
        if id != "x" {
              n = id.parse::<u32>().unwrap();
        } else {
            index += 1;
            continue;
        }
        ids.insert(n, index);
        if n as u64 > max { max = n as u64};
        index += 1;
    }

    let mut t: u64 = 1;
    let mut lcd: u64 = 1;
    for (id, index) in &ids {
        loop {
            if (t + *index as u64) % *id as u64 == 0 {
                lcd = lcd * *id as u64;
                break;
            } else {
                t += lcd;
            }
        }
        
    }
    println!("{}", t);
}