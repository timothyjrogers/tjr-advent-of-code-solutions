use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
const FNAME: &str = "../data/input.txt";

fn includes(s1: &str, s2: &str) -> bool {
    if s1.len() == 0 || s2.len() == 0 { return false }
    for c in s2.chars() {
        if !s1.contains(c) { return false }
    }
    return true;
}

fn difference(s1: &str, s2: &str) -> String {
    let mut diff: Vec<char> = vec![];
    for c in s1.chars() {
        if !s2.contains(c) { diff.push(c) }
    }
    return diff.iter().collect::<String>();
}

fn main() {
    let f = File::open(FNAME).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();

    let mut result = 0;
    for line in &lines {
        let parts: Vec<&str> = line.split(" | ").collect();
        let mut inputs: Vec<&str> = parts[0].split(" ").collect();
        let digits: Vec<&str> = parts[1].split(" ").collect();
        let mut str_to_int_map: HashMap<String, String> = HashMap::new();
        let mut int_to_str_map: HashMap<u32, String> = HashMap::new();
        for i in inputs.clone().into_iter().filter(|x| (*x).len() == 2 || (*x).len() == 4 || (*x).len() == 3 || (*x).len() == 7) {
            let mut input = String::from(i);
            let mut input_chars = input.chars().collect::<Vec<char>>();
            input_chars.sort();
            input = input_chars.iter().collect::<String>();
            if input.len() == 2 {
                str_to_int_map.insert(String::from(&input), String::from("1"));
                int_to_str_map.insert(1, String::from(&input));
            } else if input.len() == 4 {
                str_to_int_map.insert(String::from(&input), String::from("4"));
                int_to_str_map.insert(4, String::from(&input));
            } else if input.len() == 3 {
                str_to_int_map.insert(String::from(&input), String::from("7"));
                int_to_str_map.insert(7, String::from(&input));
            } else if input.len() == 7 {
                str_to_int_map.insert(String::from(&input), String::from("8"));
                int_to_str_map.insert(8, String::from(&input));
            }
        }
        for i in inputs.clone().into_iter().filter(|x| (*x).len() == 6) {
            let mut input = String::from(i);
            let mut input_chars = input.chars().collect::<Vec<char>>();
            input_chars.sort();
            input = input_chars.iter().collect::<String>();
            let mut includes4 = includes(&input, int_to_str_map.get(&4).unwrap());
            let mut includes7 = includes(&input, int_to_str_map.get(&7).unwrap());
            if  includes4 && includes7 {
                str_to_int_map.insert(String::from(&input), String::from("9"));
                int_to_str_map.insert(9, String::from(&input));
            } else if !includes4 && includes7 {
                str_to_int_map.insert(String::from(&input), String::from("0"));
                int_to_str_map.insert(0, String::from(&input));
            } else if !includes4 && !includes7 {
                str_to_int_map.insert(String::from(&input), String::from("6"));
                int_to_str_map.insert(6, String::from(&input));
            }
        }
        while inputs.len() > 1 {
            let mut i = inputs.swap_remove(0);
            let mut input = String::from(i);
            let mut input_chars = input.chars().collect::<Vec<char>>();
            input_chars.sort();
            input = input_chars.iter().collect::<String>();
            match str_to_int_map.get(&input) {
                Some(x) => {
                    continue;
                },
                None => ()
            }
            if input.len() == 5 {
                let mut includes1 = includes(&input, int_to_str_map.get(&1).unwrap());
                let mut includes6minus4 = includes(&input, difference(int_to_str_map.get(&6).unwrap(), int_to_str_map.get(&4).unwrap()).as_str());
                if includes1 {
                    str_to_int_map.insert(String::from(&input), String::from("3"));
                    int_to_str_map.insert(3, String::from(&input));
                } else if !includes1 && includes6minus4 {
                    str_to_int_map.insert(String::from(&input), String::from("2"));
                    int_to_str_map.insert(2, String::from(&input));
                } else {
                    inputs.push(i);
                }
            }
        }
        let mut input = String::from(inputs[0]);
        let mut input_chars = input.chars().collect::<Vec<char>>();
        input_chars.sort();
        input = input_chars.iter().collect::<String>();
        str_to_int_map.insert(String::from(&input), String::from("5"));
        int_to_str_map.insert(5, String::from(&input));

        let mut decoded_digits: Vec<&str> = vec![];
        for d in digits {
            let mut digit = String::from(d);
            let mut digit_chars = digit.chars().collect::<Vec<char>>();
            digit_chars.sort();
            digit = digit_chars.iter().collect::<String>();
            match str_to_int_map.get(&digit) {
                Some(x) => decoded_digits.push(x),
                None => ()
            }
        }
        let value = decoded_digits.join("").parse::<u32>().unwrap();
        result += value;
    }
    println!("{}", result);
}