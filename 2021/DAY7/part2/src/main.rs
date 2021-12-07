use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

const FNAME: &str = "../data/input.txt";

fn get_fuel(steps: u32) -> u32 {
    return (steps*steps + steps)/2;
}

fn main() {
    let f = File::open(FNAME).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();
    let input: Vec<u32> = lines[0].split(",").map(|x| x.parse::<u32>().unwrap()).collect();

    let x_min = *input.iter().min().unwrap() as usize;
    let x_max = *input.iter().max().unwrap() as usize;
    let mut min_fuel = u32::MAX;
    for i in x_min..=x_max {
        let mut fuel = 0;
        for j in 0..input.len() {
            let distance = (i as i32 - input[j] as i32).abs();
            fuel += get_fuel(distance as u32);
        }
        if fuel < min_fuel { min_fuel = fuel }
    }
    println!("{}", min_fuel);
}
