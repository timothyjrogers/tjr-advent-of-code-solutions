use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

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
    
    let mut houses: HashSet<(i32,i32)> = HashSet::new();
    let mut coord: (i32, i32) = (0, 0);
    for c in line.chars() {
        houses.insert(coord);
        if c == '<' {
            coord.0 -= 1;
        } else if c == '>' {
            coord.0 += 1;
        } else if c == '^' {
            coord.1 += 1;
        } else if c == 'v' {
            coord.1 -= 1;
        }
    }
    println!("{}", houses.len());
}