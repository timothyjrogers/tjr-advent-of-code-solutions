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
    let mut input: Vec<Vec<&str>> = vec![];
    for line in &lines {
        let output: Vec<&str> = line.split(" | ").collect();
        let digits: Vec<&str> = output[1].split(" ").collect();
        input.push(digits);
    }

    let mut result = 0;
    for row in input {
        for i in row {
            if i.len() == 2 || i .len() == 3 || i.len() == 4 || i.len() == 7 { result += 1 }
        }
    }
    println!("{}", result);

}
