use std::env;
use std::io;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let intcode = fs::read_to_string(&args[1]).unwrap();
    let mut intcode: Vec<i64> = intcode.split(",").map(|x| x.parse().unwrap()).collect();

    let mut idx = 0;
    let mut instr = intcode[idx];
    let mut op = instr % 100;
    while op != 99 {
        if op == 1 {
            let mut operand1 = intcode[idx+1 as usize];
            if (instr / 100) % 10 == 0 {
                operand1 = intcode[operand1 as usize];
            }
            let mut operand2 = intcode[idx+2 as usize];
            if (instr / 1000) % 10 == 0 {
                operand2 = intcode[operand2 as usize];
            }
            let target = intcode[idx+3 as usize];
            intcode[target as usize] = operand1 + operand2;
            idx = idx + 4;
        } else if op == 2 {
            let mut operand1 = intcode[idx+1 as usize];
            if (instr / 100) % 10 == 0 {
                operand1 = intcode[operand1 as usize];
            }
            let mut operand2 = intcode[idx+2 as usize];
            if (instr / 1000) % 10 == 0 {
                operand2 = intcode[operand2 as usize];
            }
            let target = intcode[idx+3 as usize];
            intcode[target as usize] = operand1 * operand2;
            idx = idx + 4;
        } else if op == 3 {
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .expect("Failed to read line");
            let mut int_in: i64 = input.trim().parse().unwrap();
            let target = intcode[idx+1 as usize];
            intcode[target as usize] = int_in;
            idx = idx + 2;
        } else if op == 4 {
            let mut target = intcode[idx+1 as usize];
            if (instr / 100) % 10 == 0 {
                target = intcode[target as usize];
            }
            println!("{}", target);
            idx = idx + 2;
        } else if op == 5 {
            let mut operand1 = intcode[idx+1 as usize];
            if (instr / 100) % 10 == 0 {
                operand1 = intcode[operand1 as usize];
            }
            if operand1 != 0 {
                let mut operand2 = intcode[idx+2 as usize];
                if (instr / 1000) % 10 == 0 {
                    operand2 = intcode[operand2 as usize];
                }
                idx = operand2 as usize;
            } else {
                idx = idx + 3;
            }
        } else if op == 6 {
            let mut operand1 = intcode[idx+1 as usize];
            if (instr / 100) % 10 == 0 {
                operand1 = intcode[operand1 as usize];
            }
            if operand1 == 0 {
                let mut operand2 = intcode[idx+2 as usize];
                if (instr / 1000) % 10 == 0 {
                    operand2 = intcode[operand2 as usize];
                }
                idx = operand2 as usize;
            } else {
                idx = idx + 3;
            }
        } else if op == 7 {
            let mut operand1 = intcode[idx+1 as usize];
            if (instr / 100) % 10 == 0 {
                operand1 = intcode[operand1 as usize];
            }
            let mut operand2 = intcode[idx+2 as usize];
            if (instr / 1000) % 10 == 0 {
                operand2 = intcode[operand2 as usize];
            }
            let target = intcode[idx+3 as usize];
            if operand1 < operand2 {
                intcode[target as usize] = 1;
            } else {
                intcode[target as usize] = 0;
            }
            idx = idx + 4;
        } else if op == 8 {
            let mut operand1 = intcode[idx+1 as usize];
            if (instr / 100) % 10 == 0 {
                operand1 = intcode[operand1 as usize];
            }
            let mut operand2 = intcode[idx+2 as usize];
            if (instr / 1000) % 10 == 0 {
                operand2 = intcode[operand2 as usize];
            }
            let target = intcode[idx+3 as usize];
            if operand1 == operand2 {
                intcode[target as usize] = 1;
            } else {
                intcode[target as usize] = 0;
            }
            idx = idx + 4;
        }
        instr = intcode[idx as usize];
        op = instr % 100;
    }
}