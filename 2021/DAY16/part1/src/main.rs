use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

const FNAME: &str = "../data/input.txt";

fn process_packet(input: &Vec<char>, start: usize) -> (u32, u32) {
    let mut i = start;
    let mut version = u32::from_str_radix(&input[i..i+3].iter().collect::<String>(), 2).unwrap();
    let type_id = u32::from_str_radix(&input[i+3..i+6].iter().collect::<String>(), 2).unwrap();
    i += 6;
    if type_id == 4 {
        while input[i] == '1' {
            i += 5;
        }
        i+= 5;
        //i += 4 - ((i - start) % 4);
    } else {
        let length_id = input[i];
        i += 1;
        if length_id == '0'  {
            let length = u32::from_str_radix(&input[i..i+15].iter().collect::<String>(), 2).unwrap();
            i += 15;
            let mut sub_length = 0;
            while sub_length < length {
                let (l,v) = process_packet(input, i);
                version += v;
                i += l as usize;
                sub_length += l;
            }
        } else {
            let num_packets = u32::from_str_radix(&input[i..i+11].iter().collect::<String>(), 2).unwrap();
            i += 11;
            for _ in 0..num_packets {
                let (l,v) = process_packet(input, i);
                version += v;
                i += l as usize;
            }
        }
    }
    return ((i - start) as u32, version);
}

fn main() {
    let f = File::open(FNAME).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();

    let hex_dec_map: HashMap<char, &str> = HashMap::from([
        ('0', "0000"),
        ('1', "0001"),
        ('2', "0010"),
        ('3', "0011"),
        ('4', "0100"),
        ('5', "0101"),
        ('6', "0110"),
        ('7', "0111"),
        ('8', "1000"),
        ('9', "1001"),
        ('A', "1010"),
        ('B', "1011"),
        ('C', "1100"),
        ('D', "1101"),
        ('E', "1110"),
        ('F', "1111"),
    ]);

    let mut input: Vec<char> = vec![];
    for c in lines[0].chars() {
        let mut chunk = hex_dec_map.get(&c).unwrap().chars().collect::<Vec<char>>();
        input.append(&mut chunk);
    }

    let (l, v) = process_packet(&input, 0);
    println!("Result: {}", v);
}