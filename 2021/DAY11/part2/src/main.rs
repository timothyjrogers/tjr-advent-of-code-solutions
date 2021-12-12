use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

const FNAME: &str = "../data/input.txt";

fn main() {
    let f = File::open(FNAME).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let mut lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();

    let mut grid: Vec<Vec<u32>> = vec![];
    for line in lines {
        let nums = line.chars().map(|x| x.to_string().parse::<u32>().unwrap()).collect::<Vec<u32>>();
        grid.push(nums);
    }

    let mut k = 0;
    loop {
        k += 1;
        grid = grid.iter().map(|x| x.iter().map(|y| y + 1 ).collect()).collect();
        let mut changed = true;
        let mut flashes = 0;
        while changed {
            changed = false;
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    let coord = (i, j);
                    let num = grid[coord.0][coord.1];
                    let mut flash = 0;
                    if num > 9 {
                        flashes += 1;
                        grid[coord.0][coord.1] = 0;
                        flash = 1;
                        changed = true;
                    }
                    let top = coord.0 == 0;
                    let bottom = coord.0 == grid.len() -1 ;
                    let left = coord.1 == 0;
                    let right = coord.1 == grid[0].len() -1;
                    if !top {
                        if grid[coord.0 - 1][coord.1] != 0 {
                            grid[coord.0 - 1][coord.1] += flash;
                        }
                        if !left {
                            if grid[coord.0 - 1][coord.1 - 1] != 0 {
                                grid[coord.0 - 1][coord.1 - 1] += flash;
                            }
                        }
                        if !right {
                            if grid[coord.0 - 1][coord.1 + 1] != 0 {
                                grid[coord.0 - 1][coord.1 + 1] += flash;
                            }
                        }
                    }
                    if !left {
                        if grid[coord.0][coord.1 - 1] != 0 {
                            grid[coord.0][coord.1 - 1] += flash;
                        }
                    }
                    if !right {
                        if grid[coord.0][coord.1 + 1] != 0 {
                            grid[coord.0][coord.1 + 1] += flash;
                        }
                    }
                    if !bottom {
                        if grid[coord.0 + 1][coord.1] != 0 {
                            grid[coord.0 + 1][coord.1] += flash;
                        }
                        if !left {
                            if grid[coord.0 + 1][coord.1 - 1] != 0 {
                                grid[coord.0 + 1][coord.1 - 1] += flash;
                            }
                        }
                        if !right {
                            if grid[coord.0 + 1][coord.1 + 1] != 0 {
                                grid[coord.0 + 1][coord.1 + 1] += flash;
                            }
                        }
                    }
                }
            }
        }
        if flashes == grid.len() * grid[0].len() {
            println!("{}", k    );
            return;
        }
    }
}