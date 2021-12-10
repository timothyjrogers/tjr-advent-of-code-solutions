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

    let rubric: HashMap<char,u32> = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);
    let chunks: HashMap<char,char> = HashMap::from([
        ('(',')'),
        ('[',']'),
        ('{','}'),
        ('<','>'),
    ]);

    let mut result = 0;
    for line in lines {
        let mut opens: Vec<char> = vec![];
        let mut done = false;
        for c in line.chars() {
            if c == '(' || c == '[' || c == '{' || c == '<' {
                opens.push(c);
            } else {
                match opens.pop() {
                    Some(x) => {
                        if *(chunks.get(&x).unwrap()) == c {
                            continue;
                        } else {
                            done = true;
                            result += rubric.get(&c).unwrap();
                        }
                    },
                    None => ()
                }
            }
            if done { break }
        }
    }
    println!("{}", result);
}