use std::{
    collections::HashSet,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT_PATH: &str = "input.txt";

#[derive(PartialEq, Eq, Hash)]
struct Pos<T> {
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

fn main() {
    // read the position of the paper rolls into a set
    let rolls: HashSet<Pos<i32>> = read_input()
        .enumerate()
        .flat_map(move |(y, l)| {
            l.char_indices()
                .map(|(x, c)| {
                    (
                        c,
                        Pos {
                            x: x as i32,
                            y: y as i32,
                        },
                    )
                })
                .collect::<Vec<(char, Pos<i32>)>>()
        })
        .filter_map(|(c, p)| if c == '@' { Some(p) } else { None })
        .collect();

    let res = rolls
        .iter()
        .map(|r| {
            (-1..2)
                .flat_map(|x| (-1..2).map(move |y| (x, y)))
                .filter_map(|(x, y)| {
                    if x != 0 || y != 0 {
                        Some(Pos {
                            x: r.x + x,
                            y: r.y + y,
                        })
                    } else {
                        None
                    }
                })
                .filter(|p| rolls.contains(p))
                .count()
        })
        .filter(|n| *n < 4)
        .count();

    println!("amount of rolls:{}", res)
}

/// Reads the input into a iterator returning Strings
/// Panics if there is an issue with the file or a line
fn read_input() -> impl Iterator<Item = String> {
    let file = File::open(INPUT_PATH).expect("Failed to read file");
    let buf_read = BufReader::new(file);
    buf_read.lines().map(|r| r.expect("failed to read line"))
}
