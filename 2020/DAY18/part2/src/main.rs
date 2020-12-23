use std::fs::File;
use std::io::{BufReader, BufRead};

//Accepts a string with no parentheses, formatted as "OPERAND OPERATOR OPERAND"...
fn evaluate(expr: String) -> String {
    let tokens = expr.split(" ").collect::<Vec<&str>>();
    let mut cur: u64 = 0;
    let mut operation = "+";
    for (index, token) in tokens.iter().enumerate() {
        if index % 2 == 0 {
            if operation == "+" {
                cur = cur + token.parse::<u64>().unwrap();
            } else {
                cur = cur * token.parse::<u64>().unwrap();
            }
        } else {
            operation = token;
        }
    }
    return  cur.to_string();
}

fn main() {
    let fname = "../data/input.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut answer: u64 = 0;
    for v in vec {
        let mut expr = v.clone();
        while expr.contains(')') {
            let paren_end = expr.find(')').unwrap();
            let mut paren_start = 0;
            for i in 0..paren_end {
                if expr.as_bytes()[i] == '(' as u8 {
                    paren_start = i;
                }
            }
            let mut sub_expr = expr[paren_start+1..paren_end].to_string();
            while sub_expr.contains('+') {
                let mut sub_expr_tokens = sub_expr.split(" ").collect::<Vec<&str>>();
                let mut add_index = 0;
                for i in 1..sub_expr_tokens.len()-1 {
                    if sub_expr_tokens[i] == "+" {
                        add_index = i;
                        break;
                    }
                }
                let operand1 = sub_expr_tokens[add_index-1].parse::<u32>().unwrap();
                let operand2 = sub_expr_tokens[add_index+1].parse::<u32>().unwrap();
                let add_repl = (operand1 + operand2).to_string();
                for _i in 0..3 {
                    sub_expr_tokens.remove(add_index-1);
                }
                sub_expr_tokens.insert(add_index-1, &add_repl);
                sub_expr = sub_expr_tokens.join(" ");
            }
            let paren_repl = evaluate(sub_expr);
            expr.replace_range(paren_start..=paren_end, &paren_repl);
        }
        while expr.contains('+') {
            let mut expr_tokens = expr.split(" ").collect::<Vec<&str>>();
            let mut add_index = 0;
            for i in 1..expr_tokens.len()-1 {
                if expr_tokens[i] == "+" {
                    add_index = i;
                    break;
                }
            }
            let operand1 = expr_tokens[add_index-1].parse::<u32>().unwrap();
            let operand2 = expr_tokens[add_index+1].parse::<u32>().unwrap();
            let add_repl = (operand1 + operand2).to_string();
            for _i in 0..3 {
                expr_tokens.remove(add_index-1);
            }
            expr_tokens.insert(add_index-1, &add_repl);
            expr = expr_tokens.join(" ");
        }
        let res = evaluate(expr).parse::<u64>().unwrap();
        answer += res as u64;
    }
    println!("{}", answer);
}