use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead};
use std::path::Path;
use scan_fmt::scan_fmt_some;
use std::collections::HashMap;

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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Lens {
    label: String,
    num: Option<u8>,
}

#[derive(Debug, Clone, PartialEq)]
struct LensBox {
    lenses: Vec<Lens>,
}

impl LensBox {
    fn new() -> Self {
        Self { lenses: Vec::new(), }
    }

    fn push(&mut self, lens: Lens) {
        if self.contains(&lens.label) {
            if lens.num.is_none() {
                if let Some(pos) = self.lenses.iter().position(|l| l.label == lens.label) {
                    self.lenses.remove(pos);
                }
            } else if let Some(pos) = self.lenses.iter().position(|l| l.label == lens.label) {
                    self.lenses[pos] = Lens { label: lens.label, num: lens.num };
            }
        } else {
            if lens.num.is_none() {
                return;
            }
            self.lenses.push(lens);
        }
    }

    fn contains(&self, label: &str) -> bool {
        self.lenses.iter().any(|l| l.label == label)
    }

    pub fn is_empty(&self) -> bool {
        self.lenses.is_empty()
    }
    fn get_focusing_power(&self, idx: u8) -> u64 {
        let mut pow = 0;
        for (i, lens) in self.lenses.iter().enumerate() {
            dbg!(idx, i+1, lens.num);
            pow += (idx as u64 +1) * (i+1) as u64 * lens.num.unwrap_or(1) as u64;
            dbg!(lens, pow);
        }
        pow
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input(input: &str) -> Result<Lens, Box<dyn std::error::Error>> {
    dbg!(input);
    let (label, num)  = scan_fmt_some!(input, "{[a-z]}{*[^a-z0-9]}{}", String, u8);
    dbg!(label, num);
    Ok(Lens { label: label.expect("Parsing label failed"), num })
}

fn read_file_into_vec(filename: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let lines = read_lines(filename)?;
    let s: Vec<String> = lines
        .into_iter()
        .map_while(Result::ok)
        .flat_map(|l| l.split(',').map(|s| s.to_string()).collect::<Vec<String>>())
        .collect();

    dbg!("{:?}", s);
    Ok(s)
}

fn get_hash(s: &str) -> u64 {
    let input = Day15String {
        value: s.to_string(),
    };
    let mut hasher = MyHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
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
        self.state
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


fn insert_lenses(input: Vec<String>) -> HashMap<u8, LensBox> {
    let mut map: HashMap<u8, LensBox> = HashMap::new();

    for s in input {
        let lens = parse_input(&s).unwrap_or_else(|e| {
            eprintln!("Failed to parse '{}, {}'", s, e);
            Lens { label: "ERRR".to_string(), num: None }
        });
        let idx = get_hash(&lens.label) as u8;
        dbg!(idx, &lens);
        map.entry(idx).or_insert(LensBox::new()).push(lens);
    }
    map.retain(|_, lens_box| !lens_box.is_empty());
    dbg!(&map);
    map
}

fn get_hash_sum(input: &[String]) -> u64 {
    input.iter().map(|s| get_hash(s)).sum()
}

fn get_all_focussing_powers(input: Vec<String>) -> u64 {
    let map = insert_lenses(input);
    map.iter().map(|(&k, v)| v.get_focusing_power(k)).sum::<u64>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_file_into_vec("input")?;

    println!("PART1: {:?}", get_hash_sum(&input));
    println!("PART2: {:?}", get_all_focussing_powers(input));
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_box() {
        let input = read_file_into_vec("example.input").unwrap();

        assert_eq!(get_all_focussing_powers(input), 145);
    }

    #[test]
    fn test_insert_lenses() {
        let input = vec!["rn=1".to_string()];
        let mut map = HashMap::new();
        let lens = Lens { label: "rn".to_string(), num: Some(1) };
        map.insert(0 as u8, LensBox { lenses: vec![lens] });

        assert_eq!(insert_lenses(input), map);
    }

    #[test]
    fn test_insert_lenses2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4".split(",").map(|s| s.to_string()).collect();
        dbg!(&input);
        let mut map = HashMap::new();
        let lens1 = Lens { label: "rn".to_string(), num: Some(1) };
        let lens2 = Lens { label: "cm".to_string(), num: Some(2) };
        map.insert(0 as u8, LensBox { lenses: vec![lens1, lens2] });
        let lens3 = Lens { label: "pc".to_string(), num: Some(4) };
        map.insert(3 as u8, LensBox { lenses: vec![lens3] });

        assert_eq!(insert_lenses(input), map);
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse_input("rn=1").unwrap(), Lens { label: "rn".to_string(), num: Some(1) });
        assert_eq!(parse_input("qnfg-").unwrap(), Lens { label: "qnfg".to_string(), num: None });
    }

    #[test]
    fn test_hash_simple() {
        assert_eq!(get_hash("HASH"), 52);
        assert_eq!(get_hash("pc"), 3);
    }

    #[test]
    fn test_hashes_input() {
        let input = read_file_into_vec("example.input").unwrap();
        assert_eq!(input.iter().map(|s| get_hash(s)).into_iter().collect::<Vec<u64>>(),
        vec![30, 253, 97, 47, 14, 180, 9, 197, 48, 214, 231]);
    }
}
