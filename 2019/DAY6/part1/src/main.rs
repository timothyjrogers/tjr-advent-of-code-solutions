use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::cmp;

struct Node {
    val: String,
    children: Vec<Node>,
}
impl Node {
    fn new(val: String) -> Self {
        let children: Vec<Node> = Vec::new();
        Node { val: val, children: children }
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
    
    let root: Node = Node::new(String::from("COM"));
    for (idx, line) in lines.iter().enumerate() {

    }
}