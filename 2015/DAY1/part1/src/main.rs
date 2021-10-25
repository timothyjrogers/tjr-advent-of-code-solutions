use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let fname = "../data/input.txt";
    let f: File;
    match File::open(fname) {
        Ok(v) => f = v,
        Err(e) => panic!(e)
    }
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    reader.read_line(&mut line);
    let mut floor = 0;
    for c in line.chars() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
    }
    println!("{}", floor);
}