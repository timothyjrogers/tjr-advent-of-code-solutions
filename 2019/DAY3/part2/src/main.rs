use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::cmp;

#[derive(Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn manhattan(p1: &Point, p2: &Point) -> i64{
        (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
    }
    
    fn is_equal(self, p: &Point) -> bool {
        self.x == p.x && self.y == p.y
    }
}

#[derive(Clone)]
struct Line {
    points: Vec<Point>,
}

impl Line {
    fn new() -> Self {
        let mut origin: Point = Point { x: 0, y: 0};
        let mut points: Vec<Point> = Vec::new();
        points.push(origin);
        Line { points: points }
    }

    fn add_points(&mut self, x_change: i64, y_change: i64, modifier: i64) {
        let latest: Point = self.points[self.points.len() - 1];
        if x_change != 0 {
            for i in 1..=x_change {
                let mut next: Point = Point { x: latest.x + i * modifier, y: latest.y };
                self.points.push(next);
            }
        } else if y_change != 0 {
            for i in 1..=y_change {
                let mut next: Point = Point { x: latest.x, y: latest.y + i * modifier };
                self.points.push(next);
            }
        }
    }

    fn intersections(l1: &Line, l2: &Line) -> Vec<i64> {
        let mut intersections: Vec<i64> = Vec::new();
        for (idx1, p1) in l1.points.iter().enumerate() {
            for (idx2, p2) in l2.points.iter().enumerate() {
                if p1.is_equal(&p2) {
                    intersections.push((idx1 + idx2 + 2) as i64);
                }
            }
        }
        intersections
    }
}

fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    br.lines().collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).unwrap();
    let lines = read(f).unwrap();

    let mut wires: Vec<Line> = Vec::new();
    let origin = Point { x: 0, y: 0 };
    wires.push(Line::new());
    wires.push(Line::new());
    
    for (idx, line) in lines.iter().enumerate() {
        let edges: Vec<String> = line.split(",").map(|s| String::from(s)).collect();
        for edge in edges {
            let e: String = edge.chars().collect();
            let step: i64 = e[1..].to_string().parse().unwrap();
            if edge.as_bytes()[0] as char == 'D' {                
                wires[idx].add_points(0, step, -1);
            } else if edge.as_bytes()[0] as char == 'U' {
                wires[idx].add_points(0, step, 1);
            } else if edge.as_bytes()[0] as char == 'L' {
                wires[idx].add_points(step, 0, -1);
            } else if edge.as_bytes()[0] as char == 'R' {
                wires[idx].add_points(step, 0, 1);
            }
        }
    }

    wires[0].points.remove(0);
    wires[1].points.remove(0);
    let mut intersections: Vec<i64> = Line::intersections(&wires[0], &wires[1]);

    let mut min = std::i64::MAX;
    for intersection in intersections {
        if intersection < min { min = intersection }
    }
    println!("{}", min);
}