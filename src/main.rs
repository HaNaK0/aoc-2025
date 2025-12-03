use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_PATH: &str = "input.txt";
const PRIMES: [usize; 9] = [2, 3, 5, 7, 11, 13, 17, 19, 23];

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
    let mut res = 0;
    assert!(end.len() < *PRIMES.last().unwrap());
    let mut prev_checked: HashSet<u128> = HashSet::new();

    for prime in PRIMES {
        if prime > end.len() {
            break;
        }
        if !begining.len().is_multiple_of(prime) && begining.len() == end.len() {
            continue;
        }

        let mut current_half_number = if begining.len().is_multiple_of(prime) {
            begining[0..begining.len() / prime].parse::<u128>().unwrap()
        } else {
            10_u128.pow((begining.len() / prime) as u32)
        };

        let end: u128 = end.parse().unwrap();
        let begining: u128 = begining.parse().unwrap();

        let mut current_number = create_number(current_half_number, prime);

        while current_number <= end {
            if current_number >= begining && !prev_checked.contains(&current_number) {
                // println!("{current_number}");
                res += current_number;
                prev_checked.insert(current_number);
            }

            current_half_number += 1;
            current_number = create_number(current_half_number, prime)
        }
    }

    res
}

fn create_number(current_num: u128, prime: usize) -> u128 {
    (0..prime)
        .map(|_| current_num.to_string())
        .collect::<String>()
        .parse()
        .unwrap()
}
