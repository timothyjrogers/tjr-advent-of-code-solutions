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

    let parts = lines[0].split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
    let x_bounds = &parts[2][2..parts[2].len()-1].split("..").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let y_bounds = &parts[3][2..parts[2].len()].split("..").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    //algorithm determined via post-it note doodle
    let mut result = 0;
    let bound = std::cmp::min(y_bounds[0], y_bounds[1]).abs() - 1;
    for y in 0..bound { result += bound - y }
    println!("{}", result);
}