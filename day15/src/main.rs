use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead};
use std::path::Path;
use std::backtrace::Backtrace;

macro_rules! dbg {
    ($($args: expr),*) => {
        #[cfg(debug_assertions)] {
        print!("f:{}:l.{}", file!(), line!());
        $(
            print!(", {:?}: {:?}", stringify!($args), $args);
        )*
        println!(""); // to get a new line at the end
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_file_into_vec(filename: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let lines = read_lines(filename)?;
    let s: Vec<String> = lines
        .into_iter()
        .filter_map(Result::ok)
        .flat_map(|l| l.split(',').map(|s| s.to_string()).collect::<Vec<String>>())
        .collect();

    dbg!("{:?}", s);
    Ok(s)
}

fn get_hash(s: String) -> u64 {
    let input = Day15String {
        value: s,
    };
    let mut hasher = MyHasher::new();
    input.hash(&mut hasher);
    let hash = hasher.finish();
    hash
}

struct Day15String {
    value: String,
}

impl Hasher for MyHasher {
    fn write(&mut self, bytes: &[u8]) {
        for c in bytes {
            if *c == 0xff {
                break;
            }
            self.state = ((self.state + *c as u64) * 17) % 256;
        }
    }

    fn finish(&self) -> u64 {
        self.state.into()
    }
}

impl MyHasher {
    fn new() -> MyHasher {
        MyHasher { state: 0 }
    }
}

impl Hash for Day15String {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

struct MyHasher {
    state: u64,
}

fn get_hash_sum(input: Vec<String>) -> u64 {
    input.iter().map(|s| get_hash(s.to_string())).into_iter().sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_file_into_vec("input")?;

    println!("PART1: {:?}", get_hash_sum(input));
    // println!("PART2 Sum: {}", run_part2_threads());
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hash_simple() {
        assert_eq!(get_hash("HASH".to_string()), 52);
    }

    #[test]
    fn test_hashes_input() {
        let input = read_file_into_vec("example.input").unwrap();
        assert_eq!(input.iter().map(|s| get_hash(s.to_string())).into_iter().collect::<Vec<u64>>(),
        vec![30, 253, 97, 47, 14, 180, 9, 197, 48, 214, 231]);
    }
}
