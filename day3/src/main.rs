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


fn sum_tuples(tuples: Vec<u32>) -> u32 {
    tuples.iter().sum()
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn has_neighbors(x: usize, y: usize, len: usize, char_vec: Vec<Vec<char>>) -> bool { 
    let (start_x, stop_x);
    let mut num_not_dot = 0;

    if x > 0 {
        start_x = x -1;
    } else {
        start_x = x;
    }
    if x+1 == char_vec.len() {
        stop_x = x;
    } else {
        stop_x = x+1;
    }

    let (start_y, stop_y);
    if y > 0 {
        start_y = y - 1;
    } else {
        start_y = y;
    }
    if y + len == char_vec[0].len() {
        stop_y = y + len - 1;
    } else {
        stop_y = y + len ;
    }

    dbg!("start_x: {}, stop_x: {}, start_y: {}, stop_y: {}", start_x, stop_x, start_y, stop_y);

    //for i in start_x..stop_x+1 {
    for cv in char_vec.iter().take(stop_x+1).skip(start_x) {
        //for j in start_y..stop_y+1 {
        for c in cv.iter().take(stop_y+1).skip(start_y) {
            if *c != '.' {
                num_not_dot += 1;
                dbg!("Found {}, {}++", c, num_not_dot);
            }
        }
    }

    if num_not_dot > len {
        return true;
    }
    false
}

fn parse_lines(lines: Vec<String>) -> Vec<Vec<char>> {
    let char_vec: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    dbg!("char_vec: {:?}", char_vec);
    char_vec
}

fn remove_elem(first_ten: Vec<u32>, rest: Vec<u32>) -> Vec<u32> {
    use std::collections::HashSet;

    let set1: HashSet<_> = first_ten.into_iter().collect();
    let set2: HashSet<_> = rest.into_iter().collect();

    set1.difference(&set2).cloned().collect()
}

fn get_all_numbers_wneighbor(char_vec: Vec<Vec<char>>) -> (Vec<u32>, Vec<u32>) {
    let  mut nums_with_neighbors : Vec<u32> = Vec::new();
    let  mut all_nums : Vec<u32> = Vec::new();

    for (i, line) in char_vec.iter().enumerate() {
        let mut j = 0;
        while j < line.len() {
            let ch = line[j];
            dbg!("Character at ({}, {}): {}", i, j, ch);
            if ch.is_ascii_digit() {
                let len;
                if  (j + 1 == line.len()) || (j + 1 < line.len() && !line[j + 1].is_ascii_digit()) {
                    len = 1;
                } else if (j + 2 == line.len()) || (j + 2 < line.len() && !line[j + 2].is_ascii_digit()) {
                    len = 2;
                } else {
                    len = 3;
                }
                
                dbg!("i: {}, j: {}, len: {}", i, j, len);
                
                let s: String = line[j..j+len].iter().collect();
                dbg!("s: {}", s);
                let n =  s.parse::<u32>()
                            .unwrap_or_else(|_| panic!("Line was: {}", line.iter().collect::<String>()));
                            //.expect(&format!("Line was: {}", line.iter().collect::<String>()));

                if has_neighbors(i, j, len, char_vec.clone()) {
                    nums_with_neighbors.push(n);
                }
                all_nums.push(n);
                j += len;
            }
            j += 1;
        }
    }
    dbg!("{:?}", nums_with_neighbors);
    (nums_with_neighbors, all_nums)
}

fn part1() -> u32 {
    let mut lines_vec = Vec::new();

    if let Ok(lines) = read_lines("input") {
        for line in lines.flatten() {
                lines_vec.push(line);
        }
    }
    let (res, all) = get_all_numbers_wneighbor(parse_lines(lines_vec));    
    let _out = remove_elem(all.clone(), res.clone());

    dbg!("{:?} (len {})", &res, &res.len());
    dbg!("{:?} (len {})", &all, &all.len());
    dbg!("{:?} (len {})", &_out, &_out.len());

    let result = sum_tuples(res);
    dbg!("Sum: {}", result);
    result
}

fn get_full_part_numer(char_vec: &[Vec<char>], row: usize, col: usize) -> u32 {
    // start at (row, col)
    let mut number_str: String = char_vec[row][col].to_string();
    
    // Fan out to the left
    let mut left_col = col - 1;
    while char_vec[row][left_col].is_ascii_digit() {
        number_str = char_vec[row][left_col].to_string() + &number_str;
        if left_col == 0 {
            break;
        } else {
            left_col -= 1;
        }
    }

    // Fan out to the right
    let mut right_col = col + 1;
    while right_col < char_vec[row].len() && char_vec[row][right_col].is_ascii_digit() {
        number_str = number_str + &char_vec[row][right_col].to_string();
        right_col += 1;
    } 

    dbg!("#### Number: {}", &number_str);
    number_str.parse::<u32>().unwrap()
}

fn get_all_symb_with_2neighbors(char_vec: Vec<Vec<char>>) -> Vec<u32> {
    let  mut nums_with_neighbors : Vec<u32> = Vec::new();

    for (i, line) in char_vec.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if !ch.is_ascii_digit() && *ch != '.' {
                dbg!("Found anker {}", ch);
                let mut neighbors: Vec<u32> = Vec::new();
                for m in 0..3 {
                    let row = i-1+m;
                    for n in 0..3 {
                        let col = j-1+n;
                        if char_vec[row][col].is_ascii_digit() {
                            let neighb = get_full_part_numer(&char_vec, row, col);
                            if !neighbors.contains(&neighb) {
                                neighbors.push(neighb);
                            }
                        }
                    }
                }
                if neighbors.len() == 2 {
                    nums_with_neighbors.push(neighbors[0]*neighbors[1]);
                }
            }
        }
    }
    
    dbg!("{:?}", nums_with_neighbors);
    nums_with_neighbors
}

fn part2() -> u32 {
    let mut lines_vec = Vec::new();

    if let Ok(lines) = read_lines("input") {
        for line in lines.flatten() {
                lines_vec.push(line);
        }
    }
    let res = get_all_symb_with_2neighbors(parse_lines(lines_vec));

    let result = sum_tuples(res);
    dbg!("Sum: {}", result);
    result
}

fn main() -> io::Result<()> {
    println!("{}", part1());
    println!("{}", part2());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_number() {
        let input : &str = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.";
        let inp_str = input.lines().map(|l| l.to_string()).collect();
        let (nums, _ignore) = get_all_numbers_wneighbor(parse_lines(inp_str));

        assert_eq!(nums, [467, 35, 633, 617]);
    }

    #[test]
    fn test_find_number2() {
        let input : &str = 
        r#"467..114.3
5..*......
..35..633.
......#...
617*....45
55...+.584"#;
        let inp_str = input.lines().map(|l| l.to_string()).collect();
        let (nums, _ignore) = get_all_numbers_wneighbor(parse_lines(inp_str));
        let n =  println!("Nums was: {:?}", nums);

        assert_eq!(nums, [467, 5, 35, 633, 617, 45, 55, 584]);
    }

    #[test]
    fn test_find_number_part2() {
        let mut lines_vec = Vec::new();

        if let Ok(lines) = read_lines("input.test") {
            for line in lines {
                if let Ok(ip) = line {
                    println!("###{}", ip);
                    lines_vec.push(ip);
                }
            }
        }
        let res = get_all_symb_with_2neighbors(parse_lines(lines_vec));
        assert_eq!(sum_tuples(res), 467835);
    }
}
