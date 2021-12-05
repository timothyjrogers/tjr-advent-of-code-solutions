use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::cmp::{min, max};

const FNAME: &str = "../data/input.txt";

fn get_points(p1: (i32,i32), p2: (i32,i32)) -> Vec<(i32,i32)> {
    let mut points: Vec<(i32,i32)> = vec![];
    if p1.0 == p2.0 { //vertical
        for y in min(p1.1, p2.1)..=max(p1.1, p2.1) {
            let point = (p1.0, y);
            points.push(point);
        }
    } else if p1.1 == p2.1 { //horizontal
        for x in min(p1.0, p2.0)..=max(p1.0, p2.0) {
            let point = (x, p1.1);
            points.push(point);
        }
    } else { //45-degree
        let x_factor: i32 = if p1.0 < p2.0 { 1 } else { -1 };
        let y_factor: i32 = if p1.1 > p2.1 { -1 } else { 1 };
        for i in 0..=max(p1.0, p2.0)-min(p1.0, p2.0) {
            let point = (p1.0+i*x_factor, p1.1+i*y_factor);
            points.push(point);
        }
    }
    return points;
}

fn main() {
    let f = File::open(FNAME).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();

    let mut line_segments = vec![];
    for line in lines {
        let coord_strings = line.split(" -> ").collect::<Vec<&str>>();
        let mut point_strings = coord_strings[0].split(",").collect::<Vec<&str>>();
        let x1 = point_strings[0].parse::<i32>().unwrap();
        let y1 = point_strings[1].parse::<i32>().unwrap();
        point_strings = coord_strings[1].split(",").collect::<Vec<&str>>();
        let x2 = point_strings[0].parse::<i32>().unwrap();
        let y2 = point_strings[1].parse::<i32>().unwrap();
        line_segments.push(((x1,y1),(x2,y2)));
    }
    //line_segments.retain(|p| p.0.0 == p.1.0 || p.0.1 == p.1.1); //keep only horizontal or vertical segments

    let mut points: HashMap<(i32,i32), u32> = HashMap::new();
    for segment in line_segments  {
        let ps = get_points(segment.0, segment.1);
        for p in ps {
            match points.get(&p) {
                Some(n) => points.insert(p, n + 1),
                None => points.insert(p, 1)
            };
        }
    }
    let mut result = 0;
    for (point, count) in points {
        if count > 1 { result += 1 }
    }
    println!("{}", result);
}
