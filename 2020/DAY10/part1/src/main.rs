use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    let fname = "../data/input.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut nums: Vec<i32> = vec.into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
    nums.sort();
    let mut distribution: HashMap<i32, i32> = HashMap::new();
    let mut cur = 0;
    for num in nums {
        let diff = num - cur;
        if diff <= 3 {
            cur = num;
            match distribution.get(&diff) {
                Some(x) => distribution.insert(diff, x+1),
                None => distribution.insert(diff, 1),
            };
        } else {
            break;
        }
    }
    match distribution.get(&3) {
        Some(x) => distribution.insert(3, x+1),
        None => distribution.insert(3, 1),
    };
    println!("{}", distribution.get(&1).unwrap() * distribution.get(&3).unwrap());
}