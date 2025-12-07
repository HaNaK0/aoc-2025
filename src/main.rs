use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_PATH: &str = "input.txt";

fn main() {
    let res: u128 = read_input().map(|l| handle_bank(&l)).sum();

    println!("{res}");
}

/// Reads the input into a iterator returning Strings
/// Panics if there is an issue with the file or a line
fn read_input() -> impl Iterator<Item = String> {
    let file = File::open(INPUT_PATH).expect("Failed to read file");
    let buf_read = BufReader::new(file);
    buf_read.lines().map(|r| r.expect("failed to read line"))
}

fn handle_bank(bank: &str) -> u128 {
    let mut full_bank = ['0'; 12];
    let mut current_index = 0;

    for (i, current_battery) in full_bank.iter_mut().enumerate() {
        for (index, battery) in bank
            .char_indices()
            .take(bank.len() - (11 - i))
            .skip(current_index)
        {
            if battery > *current_battery {
                *current_battery = battery;
                current_index = index + 1;
            }
            if battery == '9' {
                break;
            }
        }
    }

    let full_bank = full_bank.iter().collect::<String>().parse().unwrap();
    println!("{full_bank}");
    full_bank
}
