use std::fs::File;
use std::io::{BufReader, BufRead};

const FNAME: &str = "../data/input.txt";
fn main() {
    let f = File::open(FNAME).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();
    let mut nums: Vec<i32> = vec.into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut increases = 0;
    let mut current = nums[0] + nums[1] + nums[2];
    for i in 1..nums.len()-2 {
        let slide = nums[i] + nums[i+1] + nums[i+2];
        if slide > current { increases += 1 }
        current = slide;
    }
    println!("{}", increases);
}
