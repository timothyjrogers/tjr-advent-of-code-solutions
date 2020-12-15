use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    let fname = "../data/input.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask: String = String::new();
    for v in vec {
        if v.starts_with("mask") { //mask update
            mask = v.split('=').collect::<Vec<&str>>()[1].trim().to_string();
        } else { //memory update
            let addr_start = v.find('[').unwrap();
            let addr_end = v.find(']').unwrap();
            let addr = v[addr_start+1..addr_end].parse::<u64>().unwrap();
            let mut data = format!("{:036b}", v.split('=').collect::<Vec<&str>>()[1].trim().parse::<u64>().unwrap()).chars().collect::<Vec<char>>();
            
            for (i, c) in mask.chars().enumerate() {
                if c == 'X' { continue };
                data[i] = c;
            }
            let data_string = data.into_iter().collect::<String>();
            let data_int = u64::from_str_radix(&data_string, 2).unwrap();
            mem.insert(addr, data_int);
        }
    }
    let mut res = 0;
    for (k, v) in &mem {
        res += v;
    }
    println!("{}", res);
}