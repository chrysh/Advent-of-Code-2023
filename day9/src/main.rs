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

fn read_file_into_lines(
    filename: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut lines_vec: Vec<String> = Vec::new();

    let lines = read_lines(filename)?;

    for line in lines {
        let ip = line?;
        let parts: Vec<&str> = ip.split('\n').collect();
        lines_vec.push(parts[0].to_string());
    }

    Ok(lines_vec)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines_vec: Vec<String>;
    let mut results: Vec<i32> = Vec::new();

    lines_vec = read_file_into_lines("input")?;

    for l in lines_vec {
        dbg!("Input: {:?}", l);
        results.push(find_next_value(l));
    }

    let sum: i32 = results.into_iter().sum();
    println!("##SUM: {}", sum);
    Ok(())
}

fn next_diff(input_vec: Vec<i32>) -> i32 {
    dbg!("input_vec: {:?}, len {:?}", input_vec, input_vec.len());
    if input_vec.len() == 1 {
        return input_vec[0];
    } else if input_vec.iter().all(|&x| x == 0) {
        return 0;
    } else {
        let mut new_vec = Vec::new();
        for i in 0..input_vec.len()-1 {
            new_vec.push(input_vec[i+1] - input_vec[i]);
        }
        dbg!("new_vec: {:?}, len {:?}", new_vec, new_vec.len());
        return new_vec.last().expect("No last value")+next_diff(new_vec.clone())
    }
}


fn find_next_value(input: String) -> i32 {
    let next_val;
    let input_vec: Vec<i32> = input
    .split(' ')
    .map(|x| x.parse::<i32>().expect(&format!("Not a number: {}", x)))
    .collect();

    dbg!("input_vec: {:?}", input_vec);
    next_val = next_diff(input_vec.clone());
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
        assert_eq!(find_next_value("0   3   6   9  12  15".to_string()), 18);
        assert_eq!(find_next_value("1   3   6  10  15  21".to_string()), 28);
        assert_eq!(find_next_value("10  13  16  21  30  45".to_string()), 68);
    }
}
