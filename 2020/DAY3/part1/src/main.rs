use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let fname = "../data/input.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    //slope from puzzle
    let rval = 3;
    let dval = 1;
    //space symbols from puzzle
    let tree = '#';

    let lower_bound = vec.len();
    let mut trees = 0;
    let mut cur_x = rval;
    for y in (dval..lower_bound).step_by(dval) {
        let row: Vec<char> = vec[y].chars().collect();
        if row[cur_x] == tree {
            trees = trees + 1;
        }
        cur_x = (cur_x + rval) % row.len();
    }
    println!("{}", trees);
}