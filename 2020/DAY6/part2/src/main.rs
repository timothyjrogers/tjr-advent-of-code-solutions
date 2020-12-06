use std::fs::File;
use std::io::{BufReader, BufRead};

fn common_answers(answers: String, num_responses: i32) -> i32 {
    let mut common_answers = 0;
    let mut answers_chars: Vec<char> = answers.chars().collect();
    answers_chars.sort();
    let mut repetitions = 0;
    let mut cur_char = answers_chars[0];
    for answer in answers_chars {
        if answer == cur_char {
            repetitions = repetitions + 1;
        } else {
            if repetitions == num_responses { common_answers = common_answers + 1 };
            cur_char = answer;
            repetitions = 1;
        }
    }
    if repetitions == num_responses { common_answers = common_answers + 1};
    return common_answers;
}

fn main() {
    let fname = "../data/input.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut sum = 0;
    let mut start = 0;
    for i in 0..vec.len() {
        let mut answers: String = String::new();
        if vec[i] == "" {
            answers = vec[start..i].to_vec().join("");
            let res = common_answers(answers, (i-start) as i32);
            sum = sum + res;
            start = i+1;
        } else if i == vec.len()-1 {
            answers = vec[start..=i].to_vec().join("");
            let res = common_answers(answers, (i+1-start) as i32);
            sum = sum + res;
        } 
    }
    println!("{}", sum);
}