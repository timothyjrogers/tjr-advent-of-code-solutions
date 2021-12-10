use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

const FNAME: &str = "../data/input.txt";

fn is_corrupted(line: &str) -> bool {
    let chunks: HashMap<char,char> = HashMap::from([
        ('(',')'),
        ('[',']'),
        ('{','}'),
        ('<','>'),
    ]);
    let mut opens: Vec<char> = vec![];
    for c in line.chars() {
        if c == '(' || c == '[' || c == '{' || c == '<' {
        opens.push(c);
        } else {
            match opens.pop() {
                Some(x) => {
                    if *(chunks.get(&x).unwrap()) == c {
                        continue;
                    } else {
                        return true;
                    }
                },
                None => ()
            }
        }
    }
    return false;
}
fn main() {
    let f = File::open(FNAME).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let mut lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();

    let rubric: HashMap<char,u64> = HashMap::from([
        (')', 1),
        (']', 2),
        ('}', 3),
        ('>', 4),
    ]);
    let chunks: HashMap<char,char> = HashMap::from([
        ('(',')'),
        ('[',']'),
        ('{','}'),
        ('<','>'),
    ]);

    lines.retain(|x| !is_corrupted(x));
    let mut scores: Vec<u64> = vec![];
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
                        }
                    },
                    None => ()
                }
            }
            if done { break }
        }
        let mut points = 0;
        while !opens.is_empty() {
            let cur = opens.pop().unwrap();
            let closer = chunks.get(&cur).unwrap();
            points = points * 5;
            points += rubric.get(&closer).unwrap();
        }
        scores.push(points);
    }
    scores.sort();
    println!("{}", scores[scores.len()/2]);
}