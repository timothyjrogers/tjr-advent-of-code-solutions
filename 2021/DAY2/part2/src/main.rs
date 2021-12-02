use std::fs::File;
use std::io::{BufReader, BufRead};

const FNAME: &str = "../data/input.txt";

#[derive(Default)]
struct Submarine {
    hPos: i32,
    depth: i32,
    aim: i32,
}

impl Submarine {
    fn update(&mut self, axis: &str, val: i32) {
        match axis {
            "forward" => {
                self.hPos += val;
                self.depth += self.aim * val;
            },
            "down" => self.aim += val,
            "up" => self.aim -= val,
            _ => panic!("Invalid directive")
        }
    }
}

fn main() {
    let f = File::open(FNAME).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();

    let mut submarine: Submarine = Default::default();
    for directive in vec {
        let parts: Vec<&str> = directive.split(' ').collect();
        submarine.update(parts[0], parts[1].parse::<i32>().unwrap());
    }
    println!("{}", submarine.hPos * submarine.depth);

}
