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
        let bounds: Vec<&str> = parts[0].split('-').collect();
        let lb = bounds[0].parse::<i32>().expect("Lower bound is an invalid integer");
        let ub = bounds[1].parse::<i32>().expect("Upper bound is an invalid integer");
        let pchar = parts[1].chars().next().unwrap();
        let pchars = parts[2].trim().chars();
        
        let mut count = 0;
        for c in pchars {
            if c == pchar {
                count = count + 1;
            }
        }
        if lb <= count && count <= ub {
            valid = valid + 1;
        }
    }
    println!("{}", valid);
}