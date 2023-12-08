use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


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

    println!("start_x: {}, stop_x: {}, start_y: {}, stop_y: {}", start_x, stop_x, start_y, stop_y);

    for i in start_x..stop_x+1 {
        for j in start_y..stop_y+1 {
            if char_vec[i][j] != '.' {
                num_not_dot += 1;
                println!("Found {}, {}++", char_vec[i][j], num_not_dot);
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

    println!("char_vec: {:?}", char_vec);
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
            println!("Character at ({}, {}): {}", i, j, ch);
            if ch.is_digit(10) {
                let len;
                if  (j + 1 == line.len()) || (j + 1 < line.len() && !line[j + 1].is_digit(10)) {
                    len = 1;
                } else if (j + 2 == line.len()) || (j + 2 < line.len() && !line[j + 2].is_digit(10)) {
                    len = 2;
                } else {
                    len = 3;
                }
                
                println!("i: {}, j: {}, len: {}", i, j, len);
                
                let s: String = line[j..j+len].iter().collect();
                println!("s: {}", s);
                let n =  s.parse::<u32>().expect(&format!("Line was: {}", line.iter().collect::<String>()));

                if has_neighbors(i, j, len, char_vec.clone()) {
                    nums_with_neighbors.push(n);
                }
                all_nums.push(n);
                j += len;
            }
            j += 1;
        }
    }
    println!("{:?}", nums_with_neighbors);
    (nums_with_neighbors, all_nums)
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
    let (res, all) = get_all_numbers_wneighbor(parse_lines(lines_vec));    
    let out = remove_elem(all.clone(), res.clone());

    println!("{:?} (len {})", res, res.len());
    println!("{:?} (len {})", all, all.len());
    println!("{:?} (len {})", out, out.len());
    
    println!("res: (len {})", res.len());
    println!("all: (len {})", all.len());
    println!("out: (len {})", out.len());

    println!("Sum: {}", sum_tuples(res));
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

}
