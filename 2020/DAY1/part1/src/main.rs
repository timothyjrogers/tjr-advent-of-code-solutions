use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let fname = "../data/input.txt";
    let f: File;
    match File::open(fname) {
        Ok(v) => f = v,
        Err(_e) => panic!("Unable to open data file")
    }
    let reader = BufReader::new(f);
    let mut vec: Vec<i32> = vec![];

    for l in reader.lines() {
        match l {
            Ok(v) => {
                match v.trim().parse::<i32>() {
                    Ok(n) => vec.push(n),
                    Err(_e) => panic!("Invalid number format")
                }
            },
            Err(e) => panic!("{}", e)
        }
    }

    for i in 0..vec.len()-1 {
        for j in i+1..vec.len() {
            if vec[i] + vec[j] == 2020 {
                println!("Entries: {}, {}", vec[i], vec[j]);
                println!("Answer: {}", vec[i] * vec[j]);
            }
        }
    }
}