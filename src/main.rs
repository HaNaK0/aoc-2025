use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_PATH: &str = "input.txt";

#[derive(Clone)]
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

    #[allow(dead_code)]
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

    pub fn count(&self) -> usize {
        self.end - self.start + 1
    }
}

fn main() {
    let mut input = read_input();

    let mut line = input.next().unwrap();
    let mut spans: Vec<Span> = Vec::new();

    while !line.is_empty() {
        let mut new_spans = vec![];
        let mut new_span = Span::new(&line);

        for span in &spans {
            if !new_span.merge(span) {
                new_spans.push(span.clone());
            }
        }
        new_spans.push(new_span);
        spans = new_spans;
        line = input.next().unwrap();
    }

    let res: usize = spans.iter().map(|s| s.count()).sum();
    println!("result: {}", res)
}

/// Reads the input into a iterator returning Strings
/// Panics if there is an issue with the file or a line
fn read_input() -> impl Iterator<Item = String> {
    let file = File::open(INPUT_PATH).expect("Failed to read file");
    let buf_read = BufReader::new(file);
    buf_read.lines().map(|r| r.expect("failed to read line"))
}
