use std::fs::File;
use std::io::{BufReader, BufRead};

fn connections(adapters: &[i32]) -> i64 {
    let mut sum = 0;
    if adapters.len() == 1 || adapters.len() == 2 { return 1 };
    for i in 1..adapters.len() {
        if adapters[i] - adapters[0] > 3 { break };
        sum = sum + connections(&adapters[i..]);
    }
    return sum;
}

fn main() {
    let fname = "../data/input.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut nums: Vec<i32> = vec.into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
    nums.sort();
    nums.insert(0,0);
    nums.push(nums[nums.len()-1]+3);
    let mut start = 0;
    let mut num_paths: i64 = 1;
    for i in 1..nums.len() {
        if nums[i] - nums[i-1] == 3 {
            num_paths = num_paths * connections(&nums[start..i]);
            start = i;
        }
    }
    println!("{}", num_paths);
}