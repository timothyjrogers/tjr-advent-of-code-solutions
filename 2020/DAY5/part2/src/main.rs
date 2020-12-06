use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let fname = "../data/input.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut ids: Vec<u32> = vec![];
    for v in vec {
        let x: Vec<char> = v.chars().collect();
        let row_raw = &x[..7].to_vec();
        let col_raw = &x[7..].to_vec();
        let mut row_num: u32= 0;
        let mut col_num: u32 = 0;

        let mut row_mask = 0b1000000;
        let mut col_mask = 0b100;
        for i in 0..row_raw.len() {
            if row_raw[i] == 'B' { row_num = row_num | row_mask };
            row_mask = row_mask >> 1;
        }
        for i in 0..col_raw.len() {
            if col_raw[i] == 'R' { col_num = col_num | col_mask };
            col_mask = col_mask >> 1;
        }
        let res = (row_num * 8) + col_num;
        ids.push(res);
    }
    ids.sort();
    for i in 0..ids.len()-1 {
        if ids[i+1] != ids[i] + 1 {
            println!("{}", ids[i]+1);
            return;
        }
    }
}