use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn parse_vals(vals: String) -> Vec<String> {
    let items: Vec<String> = vals.split(",").map(|x| x.trim().to_string()).collect();
    let mut res: Vec<String> = vec![];
    for item in items {
        if item == "no other bags." {
            return vec![];
        } else {
            let start = item.find(" ").unwrap() + 1 as usize;
            let end = item.find(" bag").unwrap();
            res.push(item[start..end].to_string());
        }
    }
    return res;
}

fn contains_bag(bag: &str, color: &str, bag_map: &HashMap<String, Vec<String>>) -> bool {
    match bag_map.get(bag) {
        Some(x) => {    
            for b in x {
                if b == color { return true };
                if contains_bag(b, color, bag_map) { return true };
            }
        }
        None => return false,
    }
    return false;
}

fn main() {
    let fname = "../data/input.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut bag_map: HashMap<String, Vec<String>> = HashMap::new();
    for v in vec {
        let kv: Vec<String> = v.split("bags contain").map(|x| x.to_string()).collect();
        let vals: Vec<String> = parse_vals(kv[1].to_string());
        if vals.len() > 0 {
            bag_map.insert(kv[0].trim().to_string(), vals.clone());
        }
    }

    let mut sum = 0;
    for k in bag_map.keys() {
        if contains_bag(k, "shiny gold", &bag_map) {
            sum = sum + 1;
        }
    }
    println!("{}", sum);
}