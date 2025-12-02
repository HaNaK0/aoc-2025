use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_PATH: &str = "input.txt";

fn main() {
    let input = read_input().next().unwrap();
    let result: u128 = input
        .split(',')
        .map(|s| {
            let mut iter = s.split('-');
            check_range(
                iter.next().expect("Failed to get range"),
                iter.next().expect("Failed to get range"),
            )
        })
        .sum();
    println!("result: {}", result);
}

/// Reads the input into a iterator returning Strings
/// Panics if there is an issue with the file or a line
fn read_input() -> impl Iterator<Item = String> {
    let file = File::open(INPUT_PATH).expect("Failed to read file");
    let buf_read = BufReader::new(file);
    buf_read.lines().map(|r| r.expect("failed to read line"))
}

fn check_range(begining: &str, end: &str) -> u128 {
    if begining.len() % 2 == 1 && end.len() == begining.len() {
        return 0;
    }
    println!("{}-{}", begining, end);

    let end: u128 = end.parse().unwrap();
    let begin_int: u128 = begining.parse().unwrap();
    let upper_half = if begin_int < 10 {"1"} else {&begining[0..begining.len() / 2]};
    let mut current_number_string = [upper_half, upper_half].join("");
    let mut res = 0;

    while current_number_string
        .parse::<u128>()
        .expect("failed to parse current number")
        <= end
    {
        let current_number = current_number_string.parse::<u128>().unwrap();
        if current_number >= begining.parse::<u128>().unwrap() {
            res += current_number;
        }
        let new_half = current_number_string[0..current_number_string.len() / 2]
            .parse::<u128>()
            .unwrap()
            + 1;
        let new_half = new_half.to_string();
        current_number_string = [new_half.as_str(), new_half.as_str()].join("");
    }
    res
}
