use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::borrow::Borrow;

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

    let mut polymer: HashMap<String, u64> = HashMap::new();
    for i in 0..lines[0].len()-1 {
        let mut pair = lines[0][i..=i+1].chars().collect::<String>();
        match polymer.get(&pair) {
            Some(x) => { polymer.insert(String::from(&pair), x+1); },
            None => { polymer.insert(String::from(&pair), 1); }
        }
    }

    for _ in 0..40 {
        let mut new_polymer: HashMap<String, u64> = HashMap::new();
        for (key, val) in &polymer {
            let mut pair = String::from(key);
            let c2 = pair.pop().unwrap();
            let c1 = pair.pop().unwrap();
            match rules.get(key) {
                Some(c) => {
                    let s1 = format!("{}{}", c, c2);
                    let s2 = format!("{}{}", c1, c);
                    match new_polymer.get(&s1) {
                        Some(v) => { new_polymer.insert(s1, *v + *val); },
                        None => { new_polymer.insert(s1, *val); }
                    }
                    match new_polymer.get(&s2) {
                        Some(v) => { new_polymer.insert(s2, *v + *val); },
                        None => { new_polymer.insert(s2, *val); }
                    }
                },
                None => {
                    match new_polymer.get(key) {
                        Some(v) => { new_polymer.insert(String::from(key), *v + *val); },
                        None => { new_polymer.insert(String::from(key), *val); }
                    }
                }
            }
        }
        polymer = new_polymer;
    }

    let mut counts: HashMap<char, u64> = HashMap::new();
    for (key, val) in &polymer {
        let mut pair = String::from(key);
        let c2 = pair.pop().unwrap();
        match counts.get(&c2) {
            Some(v) => { counts.insert(c2, *v + *val); },
            None => { counts.insert(c2, *val); }
        }
    }
    let first_char = lines[0].chars().collect::<Vec<char>>()[0];
    match counts.get(&first_char) {
        Some(v) => { counts.insert(first_char, *v + 1); },
        None => { counts.insert(first_char, 1); }
    }
    let most = counts.values().max().unwrap();
    let least = counts.values().min().unwrap();
    println!("{}", most - least);
}