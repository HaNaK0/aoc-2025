use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_PATH: &str = "input.txt";

struct Pos<T> 
{
    pub x: T,
    pub y: T,
}

impl<T> Display for Pos<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pos(x:{},y:{})", self.x, self.y)
    }
}

fn main() {}

/// Reads the input into a iterator returning Strings
/// Panics if there is an issue with the file or a line
fn read_input() -> impl Iterator<Item = String> {
    let file = File::open(INPUT_PATH).expect("Failed to read file");
    let buf_read = BufReader::new(file);
    buf_read.lines().map(|r| r.expect("failed to read line"))
}
