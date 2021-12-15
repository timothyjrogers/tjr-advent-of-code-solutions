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

    let mut rules: HashMap<String, char> = HashMap::new();
    for line in &lines[2..] {
        let parts = line.split(" -> ").map(|x| x.to_string()).collect::<Vec<String>>();
        rules.insert(String::from(&parts[0]),parts[1].chars().collect::<Vec<char>>()[0]);
    }

    let mut polymer = lines[0].to_string().chars().collect::<Vec<char>>();
    for _ in 0..10 {
        let mut new_polymer = polymer.clone();
        let mut offset = 0;
        for i in 0..polymer.len()-1 {
            let pair = polymer[i..=i+1].to_vec().iter().collect::<String>();
            match rules.get(&pair) {
                Some(x) => {
                    new_polymer.insert(i+offset+1, x.clone());
                    offset += 1;
                },
                None => ()
            }
        }
        polymer = new_polymer;
    }

    let mut counts: HashMap<char,u32> = HashMap::new();
    for c in polymer {
        match counts.get(&c) {
            Some(x) => { counts.insert(c,*x + 1); },
            None => { counts.insert(c, 1); }
        }
    }
    let most = counts.values().max().unwrap();
    let least = counts.values().min().unwrap();
    println!("{}", most - least);
}