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

fn map_word2num(s: &str) -> String {
    let number_words = [("zero", "0"), ("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9")];
    let mut result = s.to_string();

    loop {
        let mut res_pos = None;
        let mut res_word = "";
        let mut res_num = "";

        for (word, num) in number_words.iter() {
            if let Some(pos) = result.find(word) {
                if res_pos.map_or(true, |p| pos < p) {
                    res_pos = Some(pos);
                    res_word = word;
                    res_num = num;
                } 
            }
        }
        if let Some(_pos) = res_pos {
            result = result.replacen(&res_word[0..res_word.len()-1], res_num, 1);
        } else {
            break;
        }
    }

    result
}

fn find_number(s: &str) -> Option<u32> {
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;
    for c in s.chars() {
        if c.is_numeric() {
            if first.is_none() {
                first = Some(c);
            }
            last = Some(c);
        }
    };
    let ret = parse_number(first, last);
    ret
}

fn extract_numbers(list: Vec<String>) -> impl Iterator<Item = u32> {
    list.into_iter().filter_map(|s| find_number(&s))
}

fn extract_numbers_part2(list: Vec<String>) -> impl Iterator<Item = u32> {

    list.into_iter().map(|s| map_word2num(&s)).filter_map(|s| find_number(&s))
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

fn read_lines_vec(filename: &str) -> Vec<String> {
    let lines_vec: Vec<_> = read_lines(filename)
        .unwrap()
        .filter_map(|line| line.ok())
        //.inspect(|ip| { println!("{ip:?}"); })
        .collect();
    lines_vec
}

fn main() -> io::Result<()> {
    let lines_vec = read_lines_vec("input");

    println!("PART1: {}", sum_tuples(extract_numbers(lines_vec.clone())));
    println!("PART2: {}", sum_tuples(extract_numbers_part2(lines_vec)));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word2num_part2() {
        assert_eq!(sum_tuples(extract_numbers_part2(vec!["one5qdtnrpcmrnnsbeighttwotwoninegtpv".to_string()])), 19);
        assert_eq!(sum_tuples(extract_numbers_part2(vec!["eightwothree".to_string()])), 83);
        assert_eq!(sum_tuples(extract_numbers_part2(vec!["oneight".to_string()])), 18);
        assert_eq!(sum_tuples(extract_numbers_part2(vec!["twone".to_string()])), 21);
        assert_eq!(sum_tuples(extract_numbers_part2(vec!["twone".to_string()])), 21);
        assert_eq!(sum_tuples(extract_numbers_part2(vec!["nineight".to_string()])), 98);
        assert_eq!(sum_tuples(extract_numbers_part2(vec!["fiveight".to_string()])), 58);
        assert_eq!(sum_tuples(extract_numbers_part2(vec!["sevenine".to_string()])), 79);
        assert_eq!(sum_tuples(extract_numbers_part2(vec!["twone".to_string()])), 21);
        assert_eq!(sum_tuples(extract_numbers_part2(vec!["9abc".to_string()])), 99);
        assert_eq!(sum_tuples(extract_numbers_part2(vec!["9".to_string()])), 99);
    }

    #[test]
    fn test_example_part22() {
        let lines_vec = read_lines_vec("input");
        assert_eq!(sum_tuples(extract_numbers_part2(lines_vec)), 53592);
    }

    #[test]
    fn test_example_part2() {
        let lines_vec = read_lines_vec("example2.input");
        assert_eq!(sum_tuples(extract_numbers_part2(lines_vec)), 281);
    }

    #[test]
    fn test_example_part1() {
        let lines_vec = read_lines_vec("example1.input");
        assert_eq!(sum_tuples(extract_numbers(lines_vec)), 142);
    }

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
