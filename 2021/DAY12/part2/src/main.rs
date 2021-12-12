use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, HashSet};

const FNAME: &str = "../data/input.txt";

fn count_paths(paths: &HashMap<String,Vec<String>>, node: String, seen_once: Option<String>, used: &mut HashSet<String>) -> u32 {
    if node == "end" { return 1 }
    let mut result = 0;
    if node.chars().collect::<Vec<char>>()[0].is_lowercase() { used.insert(String::from(&node)); }
    let neighbors = match paths.get(&node) {
        Some(x) => x.to_vec(),
        None => vec![]
    };
    for neighbor in neighbors {
        if neighbor != "start" && used.contains(&neighbor) && seen_once.clone().is_none() { result += count_paths(paths, String::from(&neighbor), Some(String::from(&neighbor)), used) }
        if neighbor != "start" && !used.contains(&neighbor) { result += count_paths(paths, String::from(&neighbor), seen_once.clone(), used) }
    }

    match &seen_once {
        Some(n) => {
            if *n != node { used.remove(&String::from(&node)); }
        },
        None => { used.remove(&String::from(&node)); }
    }
    return result;
}

fn main() {
    let f = File::open(FNAME).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();

    let mut paths: HashMap<String, Vec<String>> = HashMap::new();
    for line in &lines {
        let parts = line.split("-").map(|x| String::from(x)).collect::<Vec<String>>();
        match paths.get(&parts[0]) {
            Some(x) => {
                let mut tmp = x.to_vec();
                tmp.push(String::from(&parts[1]));
                paths.insert(String::from(&parts[0]), tmp);
            },
            None => {
                paths.insert(String::from(&parts[0]), vec![String::from(&parts[1])]);
            }
        }
        match paths.get(&parts[1]) {
            Some(x) => {
                let mut tmp = x.to_vec();
                tmp.push(String::from(&parts[0]));
                paths.insert(String::from(&parts[1]), tmp);
            },
            None => {
                paths.insert(String::from(&parts[1]), vec![String::from(&parts[0])]);
            }
        }
    }
    let mut used: HashSet<String> = HashSet::new();
    println!("{}", count_paths(&paths, String::from("start"), None,&mut used));
}