use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

fn validate_field(field: &str, val: &str) -> bool {
    match field {
        "byr" => {
            let re = Regex::new(r"^\d{4}$").unwrap();
            let parsed_val = val.parse::<i32>().unwrap();
            return re.is_match(&val) && 1920 <= parsed_val && parsed_val <= 2002;
        },
        "iyr" => {
            let re = Regex::new(r"^\d{4}$").unwrap();
            let parsed_val = val.parse::<i32>().unwrap();
            return re.is_match(&val) && 2010 <= parsed_val && parsed_val <= 2020;
        },
        "eyr" => {
            let re = Regex::new(r"^\d{4}$").unwrap();
            let parsed_val = val.parse::<i32>().unwrap();
            return re.is_match(&val) && 2020 <= parsed_val && parsed_val <= 2030;
        },
        "hgt" => {
            let re_cm = Regex::new(r"^\d{3}cm$").unwrap();
            let re_in = Regex::new(r"^\d{2}in$").unwrap();
            if re_cm.is_match(&val) {
                let cms = &val[..3].parse::<i32>().unwrap();
                return 150 <= *cms && *cms <= 193;
            } else if re_in.is_match(&val) {
                let ins = &val[..2].parse::<i32>().unwrap();
                return 59 <= *ins && *ins <= 76;
            }
            return false;
        },
        "hcl" => {
            let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            return re.is_match(&val);
        }
        "ecl" => {
            let re = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
            return re.is_match(&val);
        }
        "pid" => {
            let re = Regex::new(r"^\d{9}$").unwrap();
            return re.is_match(&val);
        }
        _ => return false,
    }
}

fn validate_passport(passport: String) -> bool {
    let fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let codes: Vec<&str> = passport.split(" ").collect();
    let mut valid = true;
    for field in &fields {
        let mut present = false;
        for code in &codes {
            let parts: Vec<&str> = code.split(":").collect();
            if &parts[0] == field {
                present = true;
                if !validate_field(&parts[0], &parts[1]) { return false };
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