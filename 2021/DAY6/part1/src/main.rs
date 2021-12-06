use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

const FNAME: &str = "../data/input.txt";

fn main() {
    let f = File::open(FNAME).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();
    let input = lines[0].clone();

    let mut fish: HashMap<u32,u32> = HashMap::new();
    for val in input.split(",") {
        let age = val.parse::<u32>().unwrap();
        match fish.get(&age) {
            Some(n) => fish.insert(age, n+1),
            None => fish.insert(age, 1)
        };
    }

    for _ in 0..80 {
        let mut new_fish: HashMap<u32,u32> = HashMap::new();
        for i in 1..=8 {
            match fish.get(&i) {
                Some(n) => new_fish.insert(i-1, *n),
                None => None
            };
        }
        match fish.get(&0) {
            Some(n) => {
                match new_fish.get(&6) {
                    Some(m) => new_fish.insert(6,m+n),
                    None => new_fish.insert(6,*n)
                };
                new_fish.insert(8,*n);
            },
            None => ()
        };
        fish = new_fish;
    }
    let mut result = 0;
    for (_,v) in &fish {
        result += v;
    }
    println!("{}", result);
}
