use std::fs::File;
use std::io::{BufReader, BufRead};

fn generation(data: &Vec<Vec<char>>) -> (bool, Vec<Vec<char>>) {
    let mut new_data: Vec<Vec<char>> = data.clone();
    let mut changed = false;
    for y in 0..data.len() {
        for x in 0..data[y].len() {
            if data[y][x] == '.' { continue };
            let mut neighbors = 0;
            if y > 0 && data[y-1][x] == '#' { neighbors += 1 }; //north
            if y > 0 && x < data[y].len()-1 && data[y-1][x+1] == '#' { neighbors += 1}; //northeast
            if x < data[y].len()-1 && data[y][x+1] == '#' { neighbors += 1 }; //east
            if y < data.len()-1 && x < data[y].len()-1 && data[y+1][x+1] == '#' { neighbors += 1 }; //southeast
            if y < data.len()-1 && data[y+1][x] == '#' { neighbors += 1}; //south
            if y < data.len()-1 && x > 0 && data[y+1][x-1] == '#' { neighbors += 1 }; //southwest
            if x > 0 && data[y][x-1] == '#' { neighbors += 1 }; //west
            if y > 0 && x > 0 && data[y-1][x-1] == '#' { neighbors += 1 }; //northwest

            if data[y][x] == 'L' && neighbors == 0 {
                new_data[y][x] = '#';
                changed = true;
            } else if data[y][x] == '#' && neighbors >= 4 {
                new_data[y][x] = 'L';
                changed = true;
            }
        }
    }
    return (changed, new_data);
}

fn main() {
    let fname = "../data/input.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut data: Vec<Vec<char>> = vec![];
    for v in vec {
        let mut cvec: Vec<char> = vec![];
        for c in v.chars() {
            cvec.push(c);
        }
        data.push(cvec);
    }
    
    loop {
        let next_gen = generation(&data);
        data = next_gen.1;
        if !next_gen.0 { break };
    }

    let mut sum = 0;
    for i in data {
        for j in i {
            if j == '#' { sum = sum + 1 };
        }
    }
    println!("{}", sum);
}