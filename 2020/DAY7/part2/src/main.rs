use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn parse_vals(vals: String) -> Vec<(i32, String)> {
    let items: Vec<String> = vals.split(",").map(|x| x.trim().to_string()).collect();
    let mut res: Vec<(i32, String)> = vec![];
    for item in items {
        if item == "no other bags." {
            return vec![];
        } else {
            let start = item.find(" ").unwrap() + 1 as usize;
            let end = item.find(" bag").unwrap();
            res.push((item[..(start - 1 as usize)].parse::<i32>().unwrap(), item[start..end].to_string()));
        }
    }
    return res;
}

fn num_bags(bag: &str, bag_map: &HashMap<String, Vec<(i32, String)>>) -> i32 {
    let mut num = 0;
    match bag_map.get(bag) {
        Some(x) => {
            for b in x {
                num = num + b.0 + (b.0 * num_bags(&b.1, bag_map));
            }
            return num;
        },
        None => return num,
    }
}

fn main() {
    let fname = "../data/input.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut bag_map: HashMap<String, Vec<(i32, String)>> = HashMap::new();
    for v in vec {
        let kv: Vec<String> = v.split("bags contain").map(|x| x.to_string()).collect();
        let vals: Vec<(i32,String)> = parse_vals(kv[1].to_string());
        if vals.len() > 0 { 
            bag_map.insert(kv[0].trim().to_string(), vals.clone());
        }
    }

    let  sum = num_bags("shiny gold", &bag_map);
    println!("{}", sum);
}