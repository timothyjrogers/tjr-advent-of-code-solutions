use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let fname = "../data/input.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut nums: Vec<i32> = vec[..25].into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
    for i in 25..vec.len() {
        let mut sums: Vec<i32> = vec![];
        for j in 0..nums.len()-1 {
            for k in j+1..nums.len() {
                sums.push(nums[j] + nums[k]);
            }
        }
        let target = vec[i].parse::<i32>().unwrap();
        let mut found = false;
        for sum in sums {
            if target == sum { 
                found = true;
                break;
            }
        }
        if !found {
            println!("{}", target);
            return;
        }
        nums.remove(0);
        nums.push(target);
    }
}