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
    let mut vec: Vec<String> = vec![];

    for l in reader.lines() {
        match l {
            Ok(v) => vec.push(String::from(v.trim())),
            Err(e) => panic!("{}", e)
        }
    }

    let mut sqft = 0;
    for i in 0..vec.len() {
        let dimensions: Vec<i32> = vec[i].split('x')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let side1 = dimensions[0] * dimensions[1];
        let side2 = dimensions[1] * dimensions[2];
        let side3 = dimensions[0] * dimensions[2];
        sqft += (2 * side1) + (2 * side2) + (2 * side3);
        sqft += std::cmp::min(side1, std::cmp::min(side2, side3));
    }
    println!("{}", sqft);
}