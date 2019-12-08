use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}
 
fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1]).unwrap();
    let v = read(f).unwrap();
    let mut fuel = 0;
    for module in &v {
        let mut weight = *module;
        while weight > 0 {
            let new_fuel = weight/3 as i64 -2;
            if new_fuel > 0 {
                fuel = fuel + new_fuel
            }
            weight = weight/3 as i64 -2;
        }
    }
    println!("{}", fuel);
}