use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    let fname = "../data/input.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut rules: HashMap<String, Vec<(u32,u32)>> = HashMap::new();
    let mut section = 0;
    let mut seen_nums: HashMap<u32, bool> = HashMap::new();
    let mut field_vals: Vec<Vec<u32>> = vec![];
    let mut my_ticket: Vec<u32> = vec![];
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
            let nums: Vec<u32> = v.split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            my_ticket = nums.clone();
            for n in nums {
                let mut cur: Vec<u32> = vec![];
                cur.push(n.clone());
                field_vals.push(cur);
            }
        } else if section == 2 {
            let nums: Vec<u32> = v.split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            let mut valid_ticket = true;
            for n in &nums {
                let mut valid = false;
                match seen_nums.get(&n) {
                    Some(v) => valid = *v,
                    None => {
                        for (_k, rule) in &rules {
                            for r in rule {
                                if n >= &r.0 && n <= &r.1 { 
                                    valid = true;
                                    break;
                                }
                            }
                            if valid { break };
                        }
                        seen_nums.insert(n.clone(), valid);
                    }
                }
                if !valid { 
                    valid_ticket = false;
                    break;  
                 }
            }
            if valid_ticket {
                for (i, n) in nums.iter().enumerate() {
                    field_vals[i].push(n.clone());
                }
            }
        }
    }

    let mut field_positions: HashMap<String, usize> = HashMap::new();
    let mut confirmed_indexes: Vec<u32> = vec![];
    while rules.len() > 0 {
        let mut found_rule = String::new();
        let mut found_index = 0;
        for (rule_string, rule_ranges) in &rules {
            let mut matches = 0;
            for (index, values) in field_vals.iter().enumerate() {
                if confirmed_indexes.contains(&(index as u32)) { continue };
                let mut valid_field = true;
                for value in values {
                    let mut valid_value = false;
                    for rule_range in rule_ranges {
                        if value >= &rule_range.0 && value <= &rule_range.1 {
                            valid_value = true;
                            break;
                        }
                    }
                    if !valid_value {
                        valid_field = false;
                        break;
                    }
                }
                if valid_field {
                    found_index = index;
                    matches += 1;
                }
            }
            if matches == 1 {
                found_rule = rule_string.clone();
                break
            }
        }
        field_positions.insert(found_rule.clone(), found_index);
        confirmed_indexes.push(found_index as u32);
        rules.remove(&found_rule);
    }

    let mut res: u64 = 1;
    for (k, v) in field_positions {
        if k.starts_with("departure") {
            println!("{} (index {}) = {}", k, v, my_ticket[v]);
            res = res * my_ticket[v] as u64;
        }
    }
    println!("{}", res);
}