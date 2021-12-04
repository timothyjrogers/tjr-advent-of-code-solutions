use std::fs::File;
use std::io::{BufReader, BufRead};

const FNAME: &str = "../data/input.txt";
const BOARD_DIM: usize = 5;

struct Board {
    squares: Vec<(bool, u32)>,
    solved: bool,
}

impl Board {
    fn new() -> Self {
        Self { squares: Vec::new(), solved: false }
    }

    fn update(&mut self, draw: u32) -> bool {
        for i in 0..self.squares.len() {
            if self.squares[i].1 == draw {
                self.squares[i].0 = true;
                return true;
            }
        }
        return false;
    }

    fn is_complete(&self) -> bool {
        for i in 0..5 {
            if self.squares[5*i].0 && self.squares[5*i+1].0 && self.squares[5*i+2].0 && self.squares[5*i+3].0 && self.squares[5*i+4].0 { return true }
            if self.squares[i].0 && self.squares[i + 5].0 && self.squares[i + 10].0 && self.squares[i + 15].0 && self.squares[i + 20].0 { return true }
        }
        return false;
    }
}

fn main() {
    let f = File::open(FNAME).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();

    let draws: Vec<&str> = lines[0].split(",").collect();
    let mut boards: Vec<Board> = vec![];

    for i in 1..lines.len() {
        if lines[i] == "" {
            boards.push(Board::new());
            continue;
        }
        let row = lines[i].split(" ");
        for pos in row {
            if pos == " " || pos == "" { continue }
            let last = boards.len() - 1;
            let item = pos.parse::<u32>().unwrap();
            boards[last].squares.push((false, item));
        }
    }

    let mut winner = 0;
    let mut latest = 0;
    let mut solved = false;
    for draw in &draws {
        latest = draw.parse::<u32>().unwrap();
        for i in 0..boards.len() {
            if boards[i].update(latest) {
                if boards[i].is_complete() {
                    winner = i;
                    solved = true;
                    break;
                }
            }
        }
        if solved { break }
    }

    let mut result = 0;
    for square in &boards[winner].squares {
        if !square.0 { result += square.1 }
    }
    println!("Winner: {}, Latest: {}", winner, latest);
    result = result * latest;
    println!("{}", result);
}
