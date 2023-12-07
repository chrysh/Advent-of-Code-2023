use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn remove_elem(first_ten: Vec<u32>, rest: Vec<u32>) -> Vec<u32> {
    use std::collections::HashSet;

    let set1: HashSet<_> = first_ten.into_iter().collect();
    let set2: HashSet<_> = rest.into_iter().collect();

    set1.difference(&set2).cloned().collect()
}

fn reduce_elems(numbers: Vec<u32>) -> u32 {
    let first_vec;
    let (first_ten, rest) = numbers.split_at(10);
    first_vec = remove_elem(first_ten.to_vec(), rest.to_vec());
    println!("reduced: {:?}", first_vec);

    match first_vec.len() {
        10 => 0,
        _ => 2_u32.pow(9 - first_vec.len() as u32),
    }
    
}

fn reduce_to_points(vec: Vec<Vec<u32>>) -> Vec<u32> {
    vec.iter().map(|x| reduce_elems(x.to_vec())).collect()
}


fn sum_points(points: Vec<u32>) -> u32 {
    points.iter().sum()
}

fn parse_lines(lines: Vec<String>) -> Vec<Vec<u32>> {
    let cards: Vec<Vec<u32>> = lines
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .skip(2)
                .filter_map(|s| s.parse::<u32>().ok())
                .collect()
        })
        .collect();

    println!("cards: {:?}", cards);
    cards
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> io::Result<()> {
    let mut lines_vec = Vec::new();

    if let Ok(lines) = read_lines("input") {
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
                lines_vec.push(ip);
            }
        }
    }
    println!("{:?}", sum_points(reduce_to_points(parse_lines(lines_vec))));
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input() {
        let input: Vec<String> = r#" Card   1:  4 16 87 61 11 37 43 25 49 17 | 54 36 14 55 83 58 43
                        Card   2: 15 53 49 30 36 68 76 12  2 73 |  3 18 33 27 34 75  5"#
                        .lines().map(|s| s.to_string()).collect();

        assert_eq!(parse_lines(input), vec!(vec!(4, 16, 87, 61, 11, 37, 43, 25, 49, 17, 54, 36, 14, 55, 83, 58, 43),
                                            vec!(15, 53, 49, 30, 36, 68, 76, 12, 2, 73, 3, 18, 33, 27, 34, 75, 5)));
    }

    #[test]
    fn test_input_parsing() {
        let input = vec!(vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 2, 36, 14, 55, 83, 58, 43),
                        vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 2, 73, 5, 18, 53, 27, 34, 75, 5));
        assert_eq!(reduce_to_points(input), vec!(1, 2));
    }
}
