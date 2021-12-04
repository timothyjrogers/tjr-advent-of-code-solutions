use std::fs::File;
use std::io::{BufReader, BufRead};

const FNAME: &str = "../data/input.txt";

fn main() {
    let f = File::open(FNAME).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    let mut ones: Vec<u32> = vec![0; 12];
    for line in &lines {
        for (i, c) in line.chars().enumerate() {
            if c == '1' { ones[i] += 1 }
        }
    }
    for one in ones {
        if one >= (lines.len() as u32) / 2 {
            gamma = (gamma | 0x1) << 1;
            epsilon = epsilon << 1;
        } else {
            gamma = gamma << 1;
            epsilon = (epsilon | 0x1) << 1;
        }
    }
    gamma = gamma >> 1;
    epsilon = epsilon >> 1;
    println!("{}", gamma * epsilon);
}
