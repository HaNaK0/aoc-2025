use std::{fs::File, io::{BufRead, BufReader}};

const INPUT_PATH: &str = "input.txt";

fn main() {
    let lines: Vec<String> = read_input().collect();

    let operators: Vec<&str> = lines.last().unwrap().split_whitespace().collect();

    let results: Vec<u128> = lines[..lines.len() - 1].iter()
        .map(|l| l.split_whitespace().map(|s| s.parse::<u128>().unwrap()).collect::<Vec<u128>>())
        .reduce(|res, next| {
            res.iter().zip(next.iter()).zip(operators.iter()).map(|((p, n), o)| {
                match *o {
                    "+" => p + n,
                    "*" => p * n,
                    _ => panic!("invalid operator")
                }
            }).collect()
        }).unwrap();

    println!("{:?}", results.iter().sum::<u128>())
}

/// Reads the input into a iterator returning Strings
/// Panics if there is an issue with the file or a line
fn read_input() -> impl Iterator<Item = String> {
    let file = File::open(INPUT_PATH).expect("Failed to read file");
    let buf_read = BufReader::new(file);
    buf_read.lines().map(|r| r.expect("failed to read line"))
}

