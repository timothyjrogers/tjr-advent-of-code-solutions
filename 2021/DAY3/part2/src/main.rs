use std::fs::File;
use std::io::{BufReader, BufRead};

const FNAME: &str = "../data/input.txt";

fn main() {
    let f = File::open(FNAME).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();
    let line_size = lines[0].len();

    let mut oxygen_rating = lines.clone();
    let mut idx = 0;
    while oxygen_rating.len() > 1 {
        let mut ones: Vec<u32> = vec![0; 12];
        for line in &oxygen_rating {
            for (i, c) in line.chars().enumerate() {
                if c == '1' { ones[i] += 1 }
            }
        }
        let mut common_bits: Vec<char> = vec![];
        for one in &ones {
            if *one as f32 >= (oxygen_rating.len() as f32) / 2.0 {
                common_bits.push('1');
            } else {
                common_bits.push('0');
            }
        }
        oxygen_rating.retain(|x| x.chars().collect::<Vec<char>>()[idx] == common_bits[idx]);
        idx += 1;
    }
    println!("Oxygen: {}", oxygen_rating[0]);

    let mut co2_rating = lines.clone();
    idx = 0;
    while co2_rating.len() > 1 {
        let mut ones: Vec<u32> = vec![0; 12];
        for line in &co2_rating {
            for (i, c) in line.chars().enumerate() {
                if c == '1' { ones[i] += 1 }
            }
        }
        println!("#ones (idx={}): {}", idx, ones[idx]);
        let mut common_bits: Vec<char> = vec![];
        for one in &ones {
            if (*one as f32) >= (co2_rating.len() as f32) / 2.0 {
                common_bits.push('1');
            } else {
                common_bits.push('0');
            }
        }
        println!("Most common (idx={}): {}", idx, common_bits[idx]);
        co2_rating.retain(|x| x.chars().collect::<Vec<char>>()[idx] != common_bits[idx]);
        idx += 1;
    }
    println!("CO2: {}", co2_rating[0]);
    println!("{}", u32::from_str_radix(oxygen_rating[0].as_str(), 2).unwrap() * u32::from_str_radix(co2_rating[0].as_str(), 2).unwrap());
}
