use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let fname = "../data/input.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let target = 26134589; //solution from part 1
    let nums: Vec<i64> = vec.into_iter().map(|x| x.parse::<i64>().unwrap()).collect();
    for i in 0..nums.len() {
        let mut sum = nums[i];
        let mut min = nums[i];
        let mut max = nums[i];
        for j in i+1..nums.len() {
            sum = sum + nums[j];
            if nums[j] < min { min = nums[j] };
            if nums[j] > max { max = nums[j] };
            if sum > target {
                break;
            } else if sum == target {
                println!("{}", min+max);
                return;
            }
        }
    }   
}