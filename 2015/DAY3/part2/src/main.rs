use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

fn main() {
    let fname = "../data/input.txt";
    let f: File;
    match File::open(fname) {
        Ok(v) => f = v,
        Err(e) => panic!(e)
    }
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    reader.read_line(&mut line);
    
    let mut houses: HashSet<(i32,i32)> = HashSet::new();
    let mut santa_coord: (i32, i32) = (0, 0);
    let mut robo_coord: (i32, i32) = (0, 0);
    houses.insert((0,0));
    let mut i = 0;
    while i < line.len() {
        let c = line.chars().collect::<Vec<char>>()[i];
        if c == '<' {
            santa_coord.0 -= 1;
        } else if c == '>' {
            santa_coord.0 += 1;
        } else if c == '^' {
            santa_coord.1 += 1;
        } else if c == 'v' {
            santa_coord.1 -= 1;
        }
        houses.insert(santa_coord);
        let c = line.chars().collect::<Vec<char>>()[i+1];
        if c == '<' {
            robo_coord.0 -= 1;
        } else if c == '>' {
            robo_coord.0 += 1;
        } else if c == '^' {
            robo_coord.1 += 1;
        } else if c == 'v' {
            robo_coord.1 -= 1;
        }
        houses.insert(robo_coord);
        i += 2;
    }
    println!("{}", houses.len());
}