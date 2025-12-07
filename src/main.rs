use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_PATH: &str = "input.txt";

fn main() {
    let res: u32 = read_input().map(|l| handle_bank(&l)).sum();

    println!("{res}");
}

/// Reads the input into a iterator returning Strings
/// Panics if there is an issue with the file or a line
fn read_input() -> impl Iterator<Item = String> {
    let file = File::open(INPUT_PATH).expect("Failed to read file");
    let buf_read = BufReader::new(file);
    buf_read.lines().map(|r| r.expect("failed to read line"))
}

fn handle_bank(bank: &str) -> u32 {
    let mut first_battery = bank.chars().next().unwrap();
    let mut current_index = 0;

    for (index, battery) in bank.char_indices().take(bank.len() - 1).skip(1) {
        if battery > first_battery {
            first_battery = battery;
            current_index = index;
        }
        if battery == '9' {
            break;
        }
    }

    let mut second_battery = '0';

    for battery in bank[current_index + 1..].chars() {
        if battery > second_battery {
            second_battery = battery;
        }
    }

    let full_bank = [first_battery, second_battery]
        .iter()
        .collect::<String>()
        .parse()
        .unwrap();
    println!("{full_bank}");
    full_bank
}
