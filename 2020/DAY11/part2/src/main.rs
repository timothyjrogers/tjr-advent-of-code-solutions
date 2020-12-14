use std::fs::File;
use std::io::{BufReader, BufRead};

fn generation(data: &Vec<Vec<char>>) -> (bool, Vec<Vec<char>>) {
    let mut new_data: Vec<Vec<char>> = data.clone();
    let mut changed = false;
    for y in 0..data.len() {
        for x in 0..data[y].len() {
            if data[y][x] == '.' { continue };
            let mut neighbors = 0;
            //north
            if y > 0 {
                for i in 1..=y {
                    if data[y-i][x] == '#' {
                        neighbors += 1;
                        break;
                    } else if data[y-i][x] == 'L' {
                        break;
                    }
                }
            }
            //northeast
            if y > 0 && x < data[y].len()-1 {
                for i in 1..=std::cmp::min(data[y].len()-x-1,y) {
                    if data[y-i][x+i] == '#' {
                        neighbors += 1;
                        break;
                    } else if data[y-i][x+i] == 'L' {
                        break;
                    }
                }
            }
            //east
            if x < data[y].len()-1 {
                for i in x+1..data[y].len() {
                    if data[y][i] == '#' {
                        neighbors += 1;
                        break;
                    } else if data[y][i] == 'L' {
                        break;
                    }
                }
            }
            //southeast
            if y < data.len()-1 && x < data[y].len()-1 {
                for i in 1..=std::cmp::min(data.len()-y-1, data[y].len()-x-1) {
                    if data[y+i][x+i] == '#' {
                        neighbors += 1;
                        break;
                    } else if data[y+i][x+i] == 'L' {
                        break;
                    }
                }
            }
            //south
            if y < data.len()-1 {
                for i in y+1..data.len() {
                    if data[i][x] == '#' {
                        neighbors += 1;
                        break;
                    } else if data[i][x] == 'L' {
                        break;
                    }
                }
            }
            //southwest
            if y < data.len()-1 && x > 0 {
                for i in 1..=std::cmp::min(data.len()-y-1, x) {
                    if data[y+i][x-i] == '#' {
                        neighbors +=1 ;
                        break;
                    } else if data[y+i][x-i] == 'L' {
                        break;
                    }
                }
            }
            //west
            if x > 0 {
                for i in 1..=x {
                    if data[y][x-i] == '#' {
                        neighbors += 1;
                        break;
                    } else if data[y][x-i] == 'L' {
                        break;
                    }
                }
            }
            //northwest
            if y > 0 && x > 0 {
                for i in 1..=std::cmp::min(x,y) {
                    if data[y-i][x-i] == '#' {
                        neighbors += 1;
                        break;
                    } else if data[y-i][x-i] == 'L' {
                        break;
                    }
                }
            }
            if data[y][x] == 'L' && neighbors == 0 {
                new_data[y][x] = '#';
                changed = true;
            } else if data[y][x] == '#' && neighbors >= 5 {
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