use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_number(left: Option<char>, right: Option<char>) -> Option<u32> {
    match (left, right) {
        (Some(l), Some(r)) => Some(format!("{}{}", l, r).parse::<u32>().unwrap()),
        (Some(l), None) => Some(format!("{}{}", l, l).parse::<u32>().unwrap()),
        _ => None
    }
}

fn find_number(s: &str) -> Option<u32> {
    let mut left: Option<char> = None;
    let mut right: Option<char> = None;
    for c in s.chars() {
        if c.is_numeric() && left.is_none() {
            left = Some(c);
        } else if c.is_numeric() {
            right = Some(c);
        }
    };
    parse_number(left, right)
}

fn extract_numbers(list: Vec<String>) -> impl Iterator<Item = u32> {
    list.into_iter().filter_map(|s| find_number(&s))
}

fn sum_tuples<I>(tuples: I) -> u32 where I: Iterator<Item = u32> {
    tuples.sum()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() -> io::Result<()> {
    let lines_vec: Vec<_> = read_lines("input")
        .unwrap()
        .filter_map(|line| line.ok())
        .inspect(|ip| { println!("{ip:?}"); })
        .collect();

    println!("{}", sum_tuples(extract_numbers(lines_vec)));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_number() {
        assert_eq!(find_number("1abc2"), Some(12));
    }

    #[test]
    fn test_extract_numbers() {
        assert_eq!(extract_numbers(vec![find_number("1abc2").unwrap().to_string()]).collect::<Vec<_>>(), vec![12]);
    }

    #[test]
    fn test_create_and_sum_tuples() {
        let input = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(sum_tuples(extract_numbers(input)), 142);
    }

}
