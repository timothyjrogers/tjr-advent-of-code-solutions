use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let intcode = fs::read_to_string(&args[1]).unwrap();
    let intcode: Vec<i64> = intcode.split(",").map(|x| x.parse().unwrap()).collect();

    for i in 0..100 {
        for j in 0..100 {
            let mut code = intcode.to_vec();
            code[1] = i;
            code[2] = j;
            let mut idx = 0;
            let mut op = intcode[idx];
            while op != 99 {
                let operand1 = code[intcode[idx+1 as usize] as usize];
                let operand2 = code[intcode[idx+2 as usize] as usize];
                let target = code[idx+3 as usize];
                if op == 1 {
                    code[target as usize] = operand1 + operand2;
                } else if op == 2 {
                    code[target as usize] = operand1 * operand2;
                }
                idx = idx + 4;
                op = code[idx as usize];
            }
            if code[0] == 19690720 {
                println!("100 * {0} + {1} = {2}", i, j, 100 * i + j);
                return
            }
        }
    }
    println!("Answer not found");
}