use std::fs::File;
use std::io::{BufReader, BufRead};

fn validate_passport(passport: String) -> bool {
    let fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let codes: Vec<&str> = passport.split(" ").map(|x| { x.split(":").next().unwrap() }).collect();
    let mut valid = true;
    for field in &fields {
        let mut present = false;
        for code in &codes {
            if code == field {
                present = true;
                break;
            }
        }
        if !present {
            valid = false;
        }
    }
    return valid;
}

fn main() {
    let fname = "../data/input.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut valid = 0;
    let mut start = 0;
    for i in 0..vec.len() {
        if vec[i] == "" {
            if validate_passport(vec[start..i].to_vec().join(" ")) {
                valid = valid + 1;
            }
            start = i+1;
        } else if i == vec.len()-1 {
            if validate_passport(vec[start..=i].to_vec().join(" ")) {
                valid = valid + 1;
            }
        }
    }
    println!("{}", valid);
}