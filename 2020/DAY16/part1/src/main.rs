use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    let fname = "../data/input.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    //Generate rules HashMap<String, Vec<(u32,u32)>>
    // e.g. departure location: 39-715 or 734-949 -> rules["departure location"] = [(39,715), (734,949)]
    let mut rules: HashMap<String, Vec<(u32,u32)>> = HashMap::new();
    let mut section = 0;
    let mut error_rate = 0;
    let mut seen_nums: HashMap<u32, bool> = HashMap::new();
    for v in vec {
        if v == "" { 
            continue
        } else if v == "your ticket:" {
            section = 1;
            continue;
        } else if v == "nearby tickets:" {
            section = 2;
            continue;
        }

        if section == 0 {
            let rule_pair = v.split(": ").collect::<Vec<&str>>();
            let rule_name = rule_pair[0].to_string();
            let rule_ranges = rule_pair[1].split(" or ");
            let mut ranges: Vec<(u32,u32)> = vec![];
            for r in rule_ranges {
                let pair = r.split("-").collect::<Vec<&str>>();
                let low = pair[0].parse::<u32>().unwrap();
                let high = pair[1].parse::<u32>().unwrap();
                ranges.push((low,high));
            }
            rules.insert(rule_name, ranges);
        } else if section == 1 {
            continue;
        } else if section == 2 {
            let nums: Vec<u32> = v.split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            for n in nums {
                let mut valid = false;
                match seen_nums.get(&n) {
                    Some(v) => valid = *v,
                    None => {
                        for (_k, rule) in &rules {
                            for r in rule {
                                if n >= r.0 && n <= r.1 { 
                                    valid = true;
                                    break;
                                }
                            }
                            if valid { break };
                        }
                        seen_nums.insert(n, valid);
                    }
                }
                if !valid { error_rate += n };
            }
        }
    }
    println!("{}", error_rate);
}