use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

fn get_neighbors(point: (i32, i32, i32, i32)) -> Vec<(i32,i32,i32, i32)> {
    let mut neighbors: Vec<(i32,i32,i32,i32)> = vec![];
    for i in -1..=1 {
        for j in -1..=1 {
            for k in -1..=1 {
                for w in -1..=1 {
                    let new_point = (point.0 + i, point.1 + j, point.2 + k, point.3 + w);
                    if new_point != point { neighbors.push(new_point) };
                }
            }
        }
    }
    return neighbors;
}

fn main() {
    let fname = "../data/input.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut active_points:  HashSet<(i32,i32,i32,i32)> = HashSet::new();
    for (y, v) in vec.iter().enumerate() {
        for (x, c) in v.chars().enumerate() {
            if c == '#' {
                active_points.insert((x as i32, y as i32, 0, 0));
            }
        }
    }

    for generation in 0..6 {
        let mut new_active_points: HashSet<(i32,i32,i32,i32)> = HashSet::new();
        let mut inactive_points: HashSet<(i32,i32,i32,i32)> = HashSet::new();
        for point in &active_points {
            let neighbors = get_neighbors(*point);
            let mut num_neighbors = 0;
            for neighbor in neighbors {
                for n in get_neighbors(neighbor) {
                    match active_points.get(&n) {
                        Some(_x) => (),
                        None => {
                            inactive_points.insert(n);
                        }
                    }
                }
                match active_points.get(&neighbor) {
                    Some(x) => num_neighbors += 1,
                    None => {},
                }
            }
            if num_neighbors == 2 || num_neighbors == 3 {
                new_active_points.insert(*point);
            }
        }
        for point in inactive_points {
            let neighbors = get_neighbors(point);
            let mut num_neighbors = 0;
            for neighbor in neighbors {
                match active_points.get(&neighbor) {
                    Some(x) => num_neighbors += 1,
                    None => (),
                }
            }
            if num_neighbors == 3 {
                new_active_points.insert(point);
            }
        }
        active_points = new_active_points;
    }
    println!("Active cubes: {}", active_points.len());
}