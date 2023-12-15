use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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

fn read_file_into_lines(filename: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut lines_vec: Vec<String> = Vec::new();

    let lines = read_lines(filename)?;

    for line in lines {
        let ip = line?;
        let parts: Vec<&str> = ip.split('\n').collect();
        lines_vec.push(parts[0].to_string());
    }

    Ok(lines_vec)
}

fn find_sum_part1(lines_vec: Vec<String>) -> i32 {
    let mut results: Vec<i32> = Vec::new();

    for l in lines_vec {
        dbg!("Input: {:?}", l);
        results.push(find_next_value_tmp(l, false));
    }

    let sum: i32 = results.into_iter().sum();
    println!("##Part 1 SUM: {}", sum);
    sum
}

fn find_sum_part2(lines_vec: Vec<String>) -> i32 {
    let mut results: Vec<i32> = Vec::new();

    for l in lines_vec {
        dbg!("Input: {:?}", l);
        results.push(find_next_value_tmp(l, true));
    }

    let sum: i32 = results.into_iter().sum();
    println!("##Part 2 SUM: {}", sum);
    sum
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines_vec: Vec<String> = read_file_into_lines("input")?;

    find_sum_part1(lines_vec.clone());
    find_sum_part2(lines_vec);

    Ok(())
}

fn next_diff(input_vec: Vec<i32>) -> i32 {
    dbg!("input_vec: {:?}, len {:?}", input_vec, input_vec.len());
    if input_vec.len() == 1 {
        input_vec[0]
    } else if input_vec.iter().all(|&x| x == 0) {
        0
    } else {
        let mut new_vec = Vec::new();
        for i in 0..input_vec.len() - 1 {
            new_vec.push(input_vec[i + 1] - input_vec[i]);
        }
        dbg!("new_vec: {:?}, len {:?}", new_vec, new_vec.len());
        return new_vec.last().expect("No last value") + next_diff(new_vec.clone());
    }
}

#[allow(dead_code)]
fn find_next_value(input: String) -> i32 {
    find_next_value_tmp(input, false)
}

fn find_next_value_tmp(input: String, reverse: bool) -> i32 {
    let input_vec_t: Vec<&str> = input.split(' ').collect::<Vec<&str>>();

    let mut input_vec: Vec<i32> = input_vec_t
        .iter()
        .filter(|x| !x.trim().is_empty())
        .map(|x| x.parse::<i32>().unwrap_or_else(|_| panic!("Not a number: {}", x)))
        .collect();

    if reverse {
        input_vec.reverse();
    }
    dbg!("input_vec: {:?}", input_vec);
    let next_val = next_diff(input_vec.clone());
    let mut next = *input_vec.last().expect("No last value");
    next += next_val;
    dbg!("next: {}", next);
    next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_diff() {
        assert_eq!(next_diff(vec!(0, 3, 6, 9, 12, 15)), 3);
        assert_eq!(next_diff(vec!(1, 3, 6, 10, 15, 21)), 7);
        assert_eq!(next_diff(vec!(10, 13, 16, 21, 30, 45)), 23);
    }

    #[test]
    fn test_find_number() {
        //assert_eq!(find_next_value("0   3   6   9  12  15".to_string()), 18);
        //assert_eq!(find_next_value("1   3   6  10  15  21".to_string()), 28);
        assert_eq!(find_next_value("10  13  16  21  30  45".to_string()), 68);
    }

    #[test]
    fn test_find_sum_part1() {
        let lines_vec: Vec<String>;

        lines_vec = read_file_into_lines("example1.input").unwrap();
        assert_eq!(find_sum_part1(lines_vec), 114);
    }

    #[test]
    fn test_find_sum_part2() {
        let lines_vec: Vec<String>;

        lines_vec = read_file_into_lines("example1.input").unwrap();
        assert_eq!(find_sum_part2(lines_vec), 2);
    }
}
