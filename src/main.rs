use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_PATH: &str = "input.txt";

fn main() {
    let lines: Vec<String> = read_input().collect();

    let operators: Vec<char> = lines.last().unwrap().chars().collect();
    let mut results: Vec<u128> = vec![];

    let mut curent_numbers = vec![];
    for i in (0..lines[0].len()).rev() {
        let current_num = lines[0..lines.len() - 1]
            .iter()
            .map(|l| l.chars().nth(i).unwrap())
            .filter(|c| c.is_ascii_digit())
            .collect::<String>();

        if !current_num.is_empty() {
            curent_numbers.push(current_num.parse::<u128>().expect("failed to parse"));
        } 

        match operators[i] {
            '+' => {
                results.push(curent_numbers.iter().sum());
                curent_numbers = vec![];
            }
            '*' => {
                results.push(curent_numbers.into_iter().reduce(|p, n| p * n).unwrap());
                curent_numbers = vec![];
            }
            _ => {}
        }
    }

    println!("{:?}", results);
    println!("{}", results.iter().sum::<u128>())
}

/// Reads the input into a iterator returning Strings
/// Panics if there is an issue with the file or a line
fn read_input() -> impl Iterator<Item = String> {
    let file = File::open(INPUT_PATH).expect("Failed to read file");
    let buf_read = BufReader::new(file);
    buf_read.lines().map(|r| r.expect("failed to read line"))
}
