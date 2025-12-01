use std::{fs::File, io::{BufRead, BufReader}};

const INPUT_PATH: &str = "input.txt";

fn main() {
    let res: u32 = read_input().fold((50,0), |(prev, sum), instr| {
        //println!("({},{})", prev, sum);
        let prev = find_code(prev, &instr);
        let sum = if prev == 0 {
            sum + 1
        } else {
            sum
        };

        (prev, sum)
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
fn find_code(prev: u32, instruction: &str) -> u32 {
    let num: u32 = instruction[1..].parse::<u32>().expect("Failed to parse number from instruction") % 100;
    (match instruction.chars().next().unwrap() {
        'L' => prev + 100 - num,
        'R' => prev + num,
        _ =>panic!("instruction not L or R")
    }) % 100
}
