use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use scan_fmt::scan_fmt;

macro_rules! dbg {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            println!($($arg)*);
        }
    };
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
enum Direction {
    U,
    D,
    L,
    R,
}

impl Direction {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'U' => Some(Self::U),
            'D' => Some(Self::D),
            'L' => Some(Self::L),
            'R' => Some(Self::R),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Step {
    dir: Direction,
    steps: u32,
    rgb: String,
}

fn read_file_into_vec(filename: &str) -> Result<Vec<Step>, Box<dyn std::error::Error>> {
    let lines = read_lines(filename)?;
    let steps: Vec<Step> = lines.into_iter().filter_map(|l|
        { match l {
            Ok(l) => match parse_input(&l) {
                Ok(direction) => Some(direction),
                Err(e) => {
                    eprintln!("Failed to parse line: {}", e);
                    None
                }
            },
            Err(e) => {
                eprintln!("Failed to read line: {}", e);
                None
            },
        }
        }).collect();

    dbg!("{:?}", steps);
    Ok(steps)
}

fn parse_input(input: &str) -> Result<Step, Box<dyn std::error::Error>> {
    let (character, steps, rgb) = scan_fmt!(input, "{[a-zA-Z]} {d} (#{[a-zA-Z0-9]})", char, u32, String)?;
    let dir = Direction::from_char(character).ok_or("Failed to parse direction")?;
    Ok(Step { dir, steps, rgb })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_file_into_vec("input")?;

    // println!("PART1: {:?}", find_sum(patterns));
    // println!("PART2 Sum: {}", run_part2_threads());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_number() {
        let input = read_file_into_lines("example1.input").expect("Failed to read file");
        assert_eq!(input, 12);
    }

}
