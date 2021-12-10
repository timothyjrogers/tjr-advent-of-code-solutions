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

    let mut minima: Vec<(u32,u32)> = vec![];
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            let val = grid[i][j];
            if i > 0 && grid[i-1][j] <= val { continue }
            if i < grid.len()-1 && grid[i+1][j] <= val { continue }
            if j > 0 && grid[i][j-1] <= val { continue }
            if j < row.len()-1 && grid[i][j+1] <= val { continue }
            minima.push((i as u32,j as u32));
        }
    }
    let mut basins: Vec<u32> = vec![];
    for minimum in minima {
        let mut basin_size = 0;
        let mut stack: Vec<(u32,u32)> = vec![];
        stack.push(minimum);
        while !stack.is_empty() {
            let point = stack.pop().unwrap();
            let x = point.0;
            let y = point.1;
            if grid[x as usize][y as usize] == 9 { continue }
            basin_size += 1;
            grid[x as usize][y as usize] = 9;
            if x > 0 { stack.push((x-1, y)) }
            if x < (grid.len()-1) as u32 { stack.push((x+1, y)) }
            if y > 0 { stack.push((x, y-1)) }
            if y < (grid[0].len()-1) as u32 { stack.push((x, y+1)) }
        }
        basins.push(basin_size);
    }
    basins.sort();
    println!("{}", basins[basins.len()-1]*basins[basins.len()-2]*basins[basins.len()-3]);
}