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
    println!("{}", cur);
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
            println!("{}", expr);
            let end = expr.find(')').unwrap();
            let mut start = 0;
            for i in 0..end {
                if expr.as_bytes()[i] == '(' as u8 {
                    start = i;
                }
            }
            println!("{},{}", start, end);
            let repl = evaluate(expr[start+1..end].to_string());
            expr.replace_range(start..=end, &repl);
        }
        let res = evaluate(expr).parse::<u64>().unwrap();
        println!("{}", res);
        answer += res as u64;
    }
    println!("{}", answer);
}