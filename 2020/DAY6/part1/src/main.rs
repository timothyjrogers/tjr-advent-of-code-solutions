use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

fn main() {
    let fname = "../data/input.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut sum = 0;
    let mut start = 0;
    for i in 0..vec.len() {
        let mut unique_answers = HashSet::new();
        if vec[i] == "" {
            let answers = vec[start..i].to_vec().join("");
            for c in answers.chars() {
                unique_answers.insert(c);
            }
            start = i+1;
        } else if i == vec.len()-1 {
            let answers = vec[start..=i].to_vec().join("    ");
            for c in answers.chars() {
                unique_answers.insert(c);
            }
        }
        sum = sum + unique_answers.len();
    }
    println!("{}", sum);
}