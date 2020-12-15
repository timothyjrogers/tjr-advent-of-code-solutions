use std::fs::File;
use std::io::{BufReader, BufRead};

struct Ship {
    x: i32,
    y: i32,
}

struct Waypoint {
    x: i32,
    y: i32,
}

impl Waypoint {
    fn turn_right(&mut self, x: i32, y: i32) {
        let wp_x_diff = self.x - x;
        let wp_y_diff = self.y - y;
        if wp_x_diff > 0 {
            self.y = y - wp_x_diff.abs();
        } else {
            self.y = y + wp_x_diff.abs();
        }
        if wp_y_diff > 0 {
            self.x = x + wp_y_diff.abs();
        } else {
            self.x = x - wp_y_diff.abs();
        }
    }

    fn turn_left(&mut self, x: i32, y: i32) {
        let wp_x_diff = self.x - x;
        let wp_y_diff = self.y - y;
        if wp_x_diff > 0 {
            self.y = y + wp_x_diff.abs();
        } else {
            self.y = y - wp_x_diff.abs();
        }
        if wp_y_diff > 0 {
            self.x = x - wp_y_diff.abs();
        } else {
            self.x = x + wp_y_diff.abs();
        }
    }
}

fn main() {
    let fname = "../data/input.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut ship = Ship{x: 0, y: 0};
    let mut waypoint = Waypoint{x: 10, y: 1};
    for v in vec {
        let n = v[1..].parse::<i32>().unwrap();
        match &v[0..1] {
            "N" => waypoint.y = waypoint.y + n,
            "S" => waypoint.y = waypoint.y - n,
            "E" => waypoint.x = waypoint.x + n,
            "W" => waypoint.x = waypoint.x - n,
            "L" => {
                let turns = n/90;
                for _i in 0..turns {
                    waypoint.turn_left(ship.x, ship.y);
                }
            },
            "R" => {
                let turns = n/90;
                for _i in 0..turns {
                    waypoint.turn_right(ship.x, ship.y);
                }
            },
            "F" => {
                let wp_x_diff = waypoint.x - ship.x;
                let wp_y_diff = waypoint.y - ship.y;
                ship.x = ship.x + wp_x_diff * n;
                ship.y = ship.y + wp_y_diff * n;
                waypoint.x = ship.x + wp_x_diff;
                waypoint.y = ship.y + wp_y_diff;
            },
            _ => panic!("Unreachable")
        }
    }
    println!("{}", ship.x.abs() + ship.y.abs());
}