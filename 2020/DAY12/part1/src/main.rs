use std::fs::File;
use std::io::{BufReader, BufRead};

enum Direction {
    North,
    South,
    East,
    West,
}

struct Ship {
    x: i32,
    y: i32,
    d: Direction,
}

impl Ship {
    fn turn_right(&mut self) {
        match self.d {
            Direction::North => self.d = Direction::East,
            Direction::East => self.d = Direction::South,
            Direction::South => self.d = Direction::West,
            Direction::West => self.d = Direction::North,
        }
    }

    fn turn_left(&mut self) {
        match self.d {
            Direction::North => self.d = Direction::West,
            Direction::West => self.d = Direction::South,
            Direction::South => self.d = Direction::East,
            Direction::East => self.d = Direction::North,
        }
    }
}

fn main() {
    let fname = "../data/input.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut ship = Ship{x: 0, y: 0, d: Direction::East};
    for v in vec {
        let n = v[1..].parse::<i32>().unwrap();
        match &v[0..1] {
            "N" => ship.y = ship.y + n,
            "S" => ship.y = ship.y - n,
            "E" => ship.x = ship.x + n,
            "W" => ship.x = ship.x - n,
            "L" => {
                let turns = n/90;
                for _i in 0..turns {
                    ship.turn_left();
                }
            }
            "R" => {
                let turns = n/90;
                for _i in 0..turns {
                    ship.turn_right();
                }
            }
            "F" => {
                match ship.d {
                    Direction::North => ship.y = ship.y + n,
                    Direction::East => ship.x = ship.x + n,
                    Direction::South => ship.y = ship.y - n,
                    Direction::West => ship.x = ship.x - n,
                }
            },
            _ => panic!("Unreachable")
        }
    }
    println!("{}", ship.x.abs() + ship.y.abs());
}