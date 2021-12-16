use std::fs::File;
use std::io::{BufReader, BufRead};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

const FNAME: &str = "../data/input.txt";


#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

//required for binaryheap per docs
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Edge {
    node: usize,
    cost: usize,
}

fn find_shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> usize {
    let mut dist: Vec<usize> = vec![usize::MAX; adj_list.len()];
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State{ cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal { return cost }
        if cost > dist[position] { continue }
        for edge in &adj_list[position] {
            let next = State { cost: cost + edge.cost, position: edge.node };
            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }
    return 0;
}

fn main() {
    let f = File::open(FNAME).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();

    let mut grid: Vec<Vec<usize>> = vec![];
    for line in &lines {
        let row = line.chars().map(|x| x.to_string().parse::<usize>().unwrap()).collect::<Vec<usize>>();
        grid.push(row);
    }
    for i in 1..5 {
        for j in 0..lines.len() {
            for k in 0..lines[0].len() {
                let new = if grid[j][k] + i > 9 { (grid[j][k] + i) % 10 + 1 } else { grid[j][k] + i };
                grid[j].push(new);
            }
        }
    }
    for i in 1..5 {
        for j in 0..lines.len() {
            let mut new_row = grid[j].iter().map(|x| if x + i > 9 { (x + i) % 10 + 1 } else { x + i }).collect::<Vec<usize>>();
            grid.push(new_row);
        }
    }

    let width = grid[0].len();
    let height = grid.len();
    let mut line = vec![];
    for row in &grid {
        for n in row { line.push(*n) }
    }

    let mut graph: Vec<Vec<Edge>> = vec![];
    for i in 0..line.len() {
        let mut node: Vec<Edge> = vec![];
        if i >= width { node.push(Edge { node: i - width, cost: line[i - width] })} //up neighbor
        if i < height * width - width { node.push(Edge { node: i + width, cost: line[i + width]})} //down neighbor
        if i % width > 0 { node.push(Edge { node: i - 1, cost: line[i - 1]})} //left neighbor
        if i %width < width - 1 { node.push(Edge { node: i + 1, cost: line[i + 1]})} //right neighbor
        graph.push(node);
    }
    println!("{}", find_shortest_path(&graph, 0, line.len()-1));
}