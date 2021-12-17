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
    let max_v_y = std::cmp::min(y_bounds[0], y_bounds[1]).abs() - 1;
    let min_v_y = std::cmp::min(y_bounds[0], y_bounds[1]);
    let min_y = min_v_y;
    let max_y = std::cmp::max(y_bounds[0], y_bounds[1]);
    let mut result = 0;

    for start_x in 0..=x_bounds[1] {
        for start_y in min_v_y..=max_v_y {
            let mut x = 0;
            let mut vx = start_x;
            let mut y = 0;
            let mut vy = start_y;
            loop {
                x += vx;
                y += vy;
                if x >= x_bounds[0] && x <= x_bounds[1] && y >= min_y && y <= max_y {
                    result += 1;
                    break;
                }
                if x > x_bounds[1] || y < min_y || (x < x_bounds[0] && vx == 0) { break }
                if vx > 0 { vx -= 1 } else if vx < 0 { vx += 1 }
                vy -= 1;
            }
        }
    }
    println!("{}", result);
}