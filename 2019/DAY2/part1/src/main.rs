use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let intcode = fs::read_to_string(&args[1]).unwrap();
    let mut intcode: Vec<i64> = intcode.split(",").map(|x| x.parse().unwrap()).collect();

    //Initialize intcode 1,2
    intcode[1] = args[2].parse().unwrap();
    intcode[2] = args[3].parse().unwrap();

    let mut idx = 0;
    let mut op = intcode[idx];
    while op != 99 {
        let operand1 = intcode[intcode[idx+1 as usize] as usize];
        let operand2 = intcode[intcode[idx+2 as usize] as usize];
        let target = intcode[idx+3 as usize];
        if op == 1 {
            intcode[target as usize] = operand1 + operand2;
        } else if op == 2 {
            intcode[target as usize] = operand1 * operand2;
        }
        idx = idx + 4;
        op = intcode[idx as usize];
    }
    println!("{}", intcode[0]);
}