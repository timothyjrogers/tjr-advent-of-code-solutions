use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::cmp::{min, max};

const FNAME: &str = "../data/input.txt";

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
        let x1 = point_strings[0].parse::<u32>().unwrap();
        let y1 = point_strings[1].parse::<u32>().unwrap();
        point_strings = coord_strings[1].split(",").collect::<Vec<&str>>();
        let x2 = point_strings[0].parse::<u32>().unwrap();
        let y2 = point_strings[1].parse::<u32>().unwrap();
        line_segments.push(((x1,y1),(x2,y2)));
    }
    line_segments.retain(|p| p.0.0 == p.1.0 || p.0.1 == p.1.1); //keep only horizontal or vertical segments

    let mut points: HashMap<(u32,u32), u32> = HashMap::new();
    for segment in line_segments  {
        if segment.0.0 == segment.1.0 { //vertical segment
            for y in min(segment.0.1, segment.1.1)..=max(segment.0.1, segment.1.1) {
                let point = (segment.0.0, y);
                match points.get(&point) {
                    Some(n) => points.insert(point, n+1),
                    None => points.insert(point, 1)
                };
            }
        } else { //horizontal segment
            for x in min(segment.0.0, segment.1.0)..=max(segment.0.0, segment.1.0) {
                let point = (x, segment.0.1);
                match points.get(&point) {
                    Some(n) => points.insert(point, n+1),
                    None => points.insert(point, 1)
                };
            }
        }
    }
    let mut result = 0;
    for (point, count) in points {
        if count > 1 { result += 1 }
    }
    println!("{}", result);
}
