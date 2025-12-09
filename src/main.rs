use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_PATH: &str = "input.txt";

struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(line: &str) -> Self {
        let mut split = line.split('-');
        Self {
            start: split.next().unwrap().parse().unwrap(),
            end: split.next().unwrap().parse().unwrap(),
        }
    }

    pub fn contains(&self, num: usize) -> bool {
        self.start <= num && self.end >= num
    }

    pub fn merge(&mut self, other: &Self) -> bool {
        if self.end < other.start || self.start > other.end {
            false
        } else {
            self.start = if self.start < other.start {
                self.start
            } else {
                other.start
            };
            self.end = if self.end > other.end {
                self.end
            } else {
                other.end
            };

            true
        }
    }
}

fn main() {
    let mut input = read_input();

    let mut line = input.next().unwrap();
    let mut spans: Vec<Span> = Vec::new();

    while !line.is_empty() {
        let new_span = Span::new(&line);

        let mut overlap = false;
        for span in &mut spans {
            if span.merge(&new_span) {
                overlap = true;
                break;
            }
        }
        if !overlap {
            spans.push(new_span);
        }
        line = input.next().unwrap();
    }

    let res = input.filter(|l| {
        let ingredient: usize = l.parse().expect("failed to parse ingredient");
        spans.iter().any(|s| s.contains(ingredient))
    }).count();
    println!("result: {res}")
}

/// Reads the input into a iterator returning Strings
/// Panics if there is an issue with the file or a line
fn read_input() -> impl Iterator<Item = String> {
    let file = File::open(INPUT_PATH).expect("Failed to read file");
    let buf_read = BufReader::new(file);
    buf_read.lines().map(|r| r.expect("failed to read line"))
}
