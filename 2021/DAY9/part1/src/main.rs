use std::fs::File;
use std::io::{BufReader, BufRead};

const FNAME: &str = "../data/input.txt";


fn main() {
    let f = File::open(FNAME).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();

    let mut grid: Vec<Vec<u32>> = vec![];
    for line in lines {
        let nums: Vec<u32> = line.chars().map(|x| String::from(x).parse::<u32>().unwrap()).collect();
        grid.push(nums);
    }

    let mut result = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            let val = grid[i][j];
            if i > 0 && grid[i-1][j] <= val { continue }
            if i < grid.len()-1 && grid[i+1][j] <= val { continue }
            if j > 0 && grid[i][j-1] <= val { continue }
            if j < row.len()-1 && grid[i][j+1] <= val { continue }
            result += val + 1
        }
    }
    println!("{}", result);
}