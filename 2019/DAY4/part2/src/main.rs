fn check_double_digit(i: &i64) -> bool {
    let s = i.to_string();
    let dups = ["11", "22", "33", "44", "55", "66", "77", "88", "99", "00"];
    for d in dups.iter() {
        if s.contains(d) { return true; }
    }
    false
}

fn check_double_digit_two(i: &i64) -> bool {
    let s = i.to_string();
    let dups = ["11", "22", "33", "44", "55", "66", "77", "88", "99", "00"];
    for d in dups.iter() {
        if s.contains(d) && s.find(d) == s.rfind(d) { return true; }
    }
    false
}

fn check_digits_increasing(i: &i64) -> bool {
    let s = i.to_string();
    let v: Vec<_> = s.chars().collect();
    for i in 0..v.len()-1 {
        if v[i].to_digit(10).unwrap() > v[i+1].to_digit(10).unwrap() { return false; }
    }
    true
}

fn main() {
    let puzzle_min = 153517;
    let puzzle_max = 630395;

    let mut possibilities: Vec<i64> = (puzzle_min..=puzzle_max).collect();
    possibilities.retain(|x| check_digits_increasing(x) && check_double_digit(x));
    possibilities.retain(|x| check_double_digit_two(x));
    println!("{}", possibilities.len());
}