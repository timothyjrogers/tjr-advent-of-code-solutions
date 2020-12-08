use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

fn run_program(code: Vec<String>) -> Option<i32> {
    let mut accumulator = 0;
    let mut pcs_seen: HashSet<i32> = HashSet::new();
    let mut pc: i32 = 0;
    loop {
        if pcs_seen.contains(&pc) { return None };
        if pc == code.len() as i32 { break };
        pcs_seen.insert(pc);
        let op: Vec<&str> = code[pc as usize].split(" ").collect();
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
    return Some(accumulator);
}

fn main() {
    let fname = "../data/input.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    for i in 0..vec.len() {
        if (&vec[i]).starts_with("acc") { continue };
        let op: Vec<&str> = vec[i].split(" ").collect();
        let mut nvec = vec.clone();
        if op[0] == "nop" {
            nvec[i] = vec!["jmp", op[1]].join(" ");
        } else {
            nvec[i] = vec!["nop", op[1]].join(" ");
        }
        match run_program(nvec) {
            Some(x) => {
                println!("{}", x);
                return;
            },
            None => continue,
        }
    }

}