use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    let fname = "../data/input.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut numbers: Vec<u64> = vec![];
    let mut latest_turns: HashMap<u64, u64> = HashMap::new();
    let mut turn = 0;
    let mut latest: u64 = 0;
    for n in vec[0].split(',') {
        let num = n.parse::<u64>().unwrap();
        numbers.push(num);
        latest = num;
        turn += 1;
    }
    for i in 0..numbers.len()-1 {
        latest_turns.insert(numbers[i], (i+1) as u64);
    }
    
    while turn < 30000000 {
        let new;
        match latest_turns.get(&latest) {
            Some(x) => {
                new = turn - x;
            },
            None => {
                new = 0;
            }
        }
        latest_turns.insert(latest, turn);
        latest = new;
        turn += 1;
    }
    println!("Answer: {}", latest);

}