use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    let fname = "../data/input.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask: Vec<char> = vec![];
    for v in vec {
        if v.starts_with("mask") { //mask update
            mask = v.split('=').collect::<Vec<&str>>()[1].trim().chars().collect::<Vec<char>>();
        } else { //memory update
            let addr_start = v.find('[').unwrap();
            let addr_end = v.find(']').unwrap();
            let mut addr_bstring = format!("{:036b}", v[addr_start+1..addr_end].parse::<u64>().unwrap()).chars().collect::<Vec<char>>();
            let data = v.split('=').collect::<Vec<&str>>()[1].trim().parse::<u64>().unwrap();
            
            for (i, c) in mask.iter().enumerate() {
                if *c == '1' { 
                    addr_bstring[i] = '1'
                } else if *c == 'X' {
                    addr_bstring[i] = 'X';
                }
            }
            let mut addrs: Vec<Vec<char>> = vec![addr_bstring];
            while addrs.len() > 0 {
                let mut addr = addrs.pop().unwrap();
                match addr.iter().collect::<String>().find('X') {
                    Some(x) => {
                        addr[x] = '0';
                        addrs.push(addr.clone());
                        addr[x] = '1';
                        addrs.push(addr.clone());
                    },
                    None => {
                        let addr_int = u64::from_str_radix(&addr.into_iter().collect::<String>(), 2).unwrap();
                        mem.insert(addr_int, data);
                    }
                }
            }
        }
    }
    let mut res = 0;
    for (_k, v) in &mem {
        res += v;
    }
    println!("{}", res);
}