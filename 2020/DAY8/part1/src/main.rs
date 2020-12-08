use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

fn main() {
    let fname = "../data/input.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut accumulator = 0;
    let mut pcs_seen: HashSet<i32> = HashSet::new();
    let mut pc: i32 = 0;
    loop {
        if pcs_seen.contains(&pc) { break };
        pcs_seen.insert(pc);
        let op: Vec<&str> = vec[pc as usize].split(" ").collect();
        match op[0] {
            "nop" => pc = pc + 1,
            "acc" => {
                accumulator = accumulator + op[1].parse::<i32>().unwrap();
                pc = pc + 1;
            },
            "jmp" => pc = pc + op[1].parse::<i32>().unwrap(),
            _ => panic!("Unreachable state"),
        }
    }
    println!("{}", accumulator);
}