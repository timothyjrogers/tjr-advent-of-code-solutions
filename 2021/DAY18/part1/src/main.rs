use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

const FNAME: &str = "../data/input.txt";

fn explode(number: &mut Vec<String>) -> bool {
    let mut depth = 0;
    let mut idx_prev_num = 0;
    let mut idx_to_explode = 0;
    let mut explode = false;
    let mut left_num = 0;
    let mut right_num = 0;
    for (i,c) in number.iter().enumerate() {
        if c == "," {
            continue;
        }
        if c == "[" { //moving into pair
            depth += 1;
            if depth > 4 {
                explode = true;
                idx_to_explode = i;
                left_num = number[i+1].parse::<u32>().unwrap();
                right_num = number[i+3].parse::<u32>().unwrap(); //skip the comma
                break;
            }
        }
        else if c == "]" { //exiting pair
            depth -= 1;
        }
        else if depth <= 4 {
            idx_prev_num = i;
        }
    }
    if explode {
        if idx_prev_num > 0 { (*number)[idx_prev_num] = (left_num + number[idx_prev_num].parse::<u32>().unwrap()).to_string() }
        let mut offset = 1;
        loop {
            match number[idx_to_explode+3+offset].parse::<u32>() {
                Ok(n) => {
                    (*number)[idx_to_explode+3+offset] = (right_num + n).to_string();
                    break;
                }
                Err(e) => {
                    offset += 1;
                    if idx_to_explode+3+offset > number.len()-2 { break }
                }
            }
        }
        for i in 0..5 {
            (*number).remove(idx_to_explode);
        }
        (*number).insert(idx_to_explode, String::from("0"));
    }
    return explode;
}


fn split(number: &mut Vec<String>) -> bool {
    let mut idx_to_split = 0;
    let mut val_to_split = 0;
    let mut split = false;
    for (i, token) in number.iter().enumerate() {
        if token != "[" && token != "]" && token != "," {
            let token_value = token.parse::<u32>().unwrap();
            if token_value >= 10 {
                split = true;
                idx_to_split = i;
                val_to_split = token_value;
                break;
            }
        }
    }
    if split {
        let left = val_to_split / 2;
        let right = (val_to_split / 2) + (val_to_split % 2);
        //let pair = format!("[{},{}]", left, right);
        (*number).remove(idx_to_split);
        //(*number).insert_str(idx_to_split, &pair);
        (*number).insert(idx_to_split, String::from("]"));
        (*number).insert(idx_to_split, right.to_string());
        (*number).insert(idx_to_split, String::from(","));
        (*number).insert(idx_to_split, left.to_string());
        (*number).insert(idx_to_split, String::from("["));
    }
    return split;
}

fn magnitude(number: &mut Vec<String>) -> u32 {
    println!("Getting magnitude of: {}", number.join(""));
    while number.len() > 1 {
        let mut idx_to_squish = 0;
        for i in 0..number.len()-5 {
            if number[i] == "[" && number[i+4] == "]" {
                idx_to_squish = i;
                break;
            }
        }
        let left = number[idx_to_squish + 1].parse::<u32>().unwrap();
        let right = number[idx_to_squish + 3].parse::<u32>().unwrap(); //skip the comma
        let val = 3*left + 2*right;
        for _ in 0..5 { number.remove(idx_to_squish); }
        number.insert(idx_to_squish, val.to_string());
        println!("{}", number.join(""));
    }
    return number[0].parse::<u32>().unwrap();
}

fn main() {
    let f = File::open(FNAME).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();

    let mut number: Vec<String> = lines[0].chars().map(|x| x.to_string()).collect();
    for line in &lines[1..] {
        let mut new_number = vec![String::from("[")];
        new_number.append(&mut number);
        new_number.push(String::from(","));
        new_number.append(&mut line.chars().map(|x| x.to_string()).collect::<Vec<String>>());
        new_number.push(String::from("]"));
        number = new_number;
        loop {
            if explode(&mut number) {
                continue;
            } else if split(&mut number) {
                continue;
            }
            break;
        }
    }
    println!("{}", magnitude(&mut number));
}