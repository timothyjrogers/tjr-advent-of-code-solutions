use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let fname = "../data/input.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    //slope from puzzle
    let rvals: [usize; 5] = [1,3,5,7,1];
    let dvals: [usize; 5] = [1,1,1,1,2];
    //space symbols from puzzle
    let tree = '#';

    let lower_bound = vec.len();
    //puzzle result overflows i32
    let mut result: u32 = 1;
    for i in 0..rvals.len() {
        let mut trees: u32 = 0;
        let mut cur_x = rvals[i];
        for y in (dvals[i]..lower_bound).step_by(dvals[i]) {
            let row: Vec<char> = vec[y].chars().collect();
            if row[cur_x] == tree {
                trees = trees + 1;
            }
            cur_x = (cur_x + rvals[i]) % row.len();
        }
        result = result * trees;
    }
    println!("{}", result);
}