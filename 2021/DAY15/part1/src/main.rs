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

    let width = lines[0].len();
    let height = lines.len();
    let line = lines.join("").chars().map(|x| x.to_string().parse::<usize>().unwrap()).collect::<Vec<usize>>();

    let mut graph: Vec<Vec<Edge>> = vec![];
    for i in 0..line.len() {
        let mut node: Vec<Edge> = vec![];
        if i >= width { node.push(Edge { node: i - width, cost: line[i - width] })} //up neighbor
        if i < height * width - width { node.push(Edge { node: i + width, cost: line[i + width]})} //down neighbor
        if i % width > 0 { node.push(Edge { node: i - 1, cost: line[i - 1]})} //left neighbor
        if i %width < width - 1 { node.push(Edge { node: i + 1, cost: line[i + 1]})} //right neighbor
        graph.push(node);
    }
    println!("{}", find_shortest_path(&graph, 0, graph.len()-1));
}