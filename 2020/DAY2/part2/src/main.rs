use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let fname = "../data/input.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let mut valid = 0;
    for v in vec {
        let parts: Vec<&str> = v.split(' ').collect();
        let positions: Vec<&str> = parts[0].split('-').collect();
        let pos1 = positions[0].parse::<usize>().expect("Position 1 is an invalid integer") - 1;
        let pos2 = positions[1].parse::<usize>().expect("Position 2 is an invalid integer") - 1;
        let pchar = parts[1].chars().next().unwrap();
        let pchars: Vec<char> = parts[2].trim().chars().collect();
        
        if (pchars[pos1] == pchar) ^ (pchars[pos2] == pchar) {
            valid = valid + 1;
        }
    }
    println!("{}", valid);
}