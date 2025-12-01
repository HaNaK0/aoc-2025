use std::{fs::File, io::{BufRead, BufReader}};

const INPUT_PATH: &str = "input.txt";

fn main() {
    let res: u32 = read_input().fold((50,0), |(prev, sum), instr| {
        let (prev, laps) = find_code(prev, &instr);
        // println!("{}({},{})",instr ,prev, laps);
        (prev, sum + laps)
    }).1;

    println!("the code is {}", res)
}

/// Reads the input into a iterator returning Strings
/// Panics if there is an issue with the file or a line
fn read_input() -> impl Iterator<Item = String> {
    let file = File::open(INPUT_PATH).expect("Failed to read file");
    let buf_read = BufReader::new(file);
    buf_read.lines().map(|r| r.expect("failed to read line"))
}

/// Read a line from the input and apply it to the prevoius state of safe knob
/// Returns the result of the rotation and the number of times it passes zero
fn find_code(prev: u32, instruction: &str) -> (u32, u32) {
    let num: u32 = instruction[1..].parse::<u32>().expect("Failed to parse number from instruction");

    let full_laps = num / 100;
    let num = num % 100;
    match instruction.chars().next().unwrap() {
        'L' => {
            (
                (prev + 100 - num) % 100,
                if prev != 0 && num != 0 && num >= prev {full_laps + 1} else {full_laps}
            )
        },
        'R' => {
            (
                (prev + num) % 100, 
                if prev != 0 && num != 0 && num >= 100 - prev {full_laps + 1} else {full_laps}
            )
        },
        _ =>panic!("instruction not L or R")
    }
}
