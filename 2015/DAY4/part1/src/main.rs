use md5;
    
fn main() {
    let input = "iwrupvqb";
    let mut x: u32 = 0;  
    loop {
        let case = format!("{}{}", input.to_string(), x.to_string());
        let hash = format!("{:032x}", md5::compute(case.as_bytes()));
        if hash.starts_with("00000") {
            println!("{}", case);
            println!("{}", hash);
            break;
        }
        x += 1;
    }
    println!("{}", x);
}