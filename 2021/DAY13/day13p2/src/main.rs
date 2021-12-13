use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

const FNAME: &str = "../data/input.txt";

fn main() {
    let f = File::open(FNAME).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();

    //let mut max_x = 0;
    //let mut max_y = 0;
    //let mut grid: Vec<Vec<u32>> = vec![];

    let mut paper: HashSet<(u32,u32)> = HashSet::new();
    let mut i = 0;
    for line in &lines {
        if line.starts_with("fold") {
            let parts = line.split("=").collect::<Vec<&str>>();
            let dim = parts[1].parse::<u32>().unwrap();
            let axis = parts[0].to_string().pop().unwrap();
            let mut new_points: HashSet<(u32,u32)> = HashSet::new();
            for point in &paper {
                if axis == 'x' { //fold 'LEFT'
                    //new_x = x - (n-x)
                    if point.0 >= dim {
                        new_points.insert((dim - (point.0 - dim), point.1));
                    } else {
                        new_points.insert((point.0, point.1));
                    }
                } else { //fold 'UP'
                    //new_y = y - (n - y)
                    if point.1 >= dim {
                        new_points.insert((point.0, dim - (point.1 - dim)));
                    } else {
                        new_points.insert((point.0,point.1));
                    }
                }
            }
            paper = new_points;
        } else if line == "" {
            continue;
        } else {
            let parts = line.split(",").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            paper.insert((parts[0],parts[1]));
        }
    }

    let mut points: Vec<(u32,u32)> = paper.into_iter().collect();
    let max_x = (*(points.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap())).0;
    let min_x = (*(points.iter().min_by(|x, y| x.0.cmp(&y.0)).unwrap())).0;
    let max_y = (*(points.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap())).1;
    let min_y = (*(points.iter().min_by(|x, y| x.1.cmp(&y.1)).unwrap())).1;
    let mut grid: Vec<Vec<char>> = vec  ![];
    println!("X MAX = {}\nY MAX = {}", max_x, max_y);
    for x in min_y..=max_y {
        grid.push(vec!['.'; (max_x - min_x + 1) as usize]);
    }
    for point in points {
        grid[point.1 as usize][point.0 as  usize] = '#';
    }
    for row in grid {
        println!("{}", row.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
    }
    //println!("{}", paper.len());
}