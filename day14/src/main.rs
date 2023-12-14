use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::thread;
use std::time::Duration;

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
        lines_vec.push(line?);
    }

    Ok(lines_vec)
}


fn float_up(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut float_matrix = vec![vec![0; matrix[0].len()]; matrix.len()];
    let mut last_empty = 0;

    for i in 0..matrix.len() {
        last_empty = 0;
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 1 {
                float_matrix[i][j] = 0;
                float_matrix[i][last_empty] = 1;
                last_empty = last_empty + 1;
            } else if matrix[i][j] == -66 {
                float_matrix[i][j] = -66;
                last_empty = j+1;
            } else if matrix[i][j] == 0 {
                float_matrix[i][j] = 0;
            }
        }
    }
    float_matrix
}

fn create_matrix(lines: Vec<&str>) -> Vec<Vec<i32>> {
    let rows = lines.len();
    let cols = lines[0].len();
    let mut matrix: Vec<Vec<i32>> =  vec![vec![0; rows]; cols];
    let mut i = 0;
    let mut j = 0;

    for l in lines {
        println!("{}", l);
        for c in l.chars() {
            let val = 0;
            match c {
                'O' => matrix[j][i] = 1,
                '#' => matrix[j][i] = -66,
                _ => matrix[j][i] = 0,
            }
            j += 1;
        }
        i += 1;
        j = 0;
    }
    matrix
}

fn calc_col_sum(col: Vec<i32>) -> i32 {
    let mut i = col.len() as i32;
    let mut sum = 0;

    for c in col {
        if c > 0 {
            sum += c * i;
        }
        i -= 1;
    }
    sum
}

fn find_weight(input: Vec<Vec<i32>>) -> i32 {
    float_up(input)
        .iter()
        .map(|col| calc_col_sum(col.to_vec()))
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines_vec: Vec<String>;

    lines_vec = read_file_into_lines("input")?;

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    let lines_str = lines_vec.iter().map(AsRef::as_ref).collect();
    println!("{}", find_weight(create_matrix(lines_str)));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_number2() {
        let input=r"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....".lines().collect::<Vec<&str>>();
        assert_eq!(find_weight(create_matrix(input)), 136);
    }

    #[test]
    fn test_create_matrix() {
        let input=vec![ r"OOOO.#", 
                        r"OO..#.", 
                        r"OO..O#", 
                        r"O..#.O"];
        assert_eq!(create_matrix(input), 
        [[1, 1, 1, 1],
        [1, 1, 1, 0],
        [1, 0, 0, 0],
        [1, 0, 0, -66],
        [0, -66, 1, 0],
        [-66, 0, -66, 1]]);
    }


    #[test]
    fn test_float_up() {
        let input=vec![ r"O....#", 
                        r".O..#.", 
                        r".O....", 
                        r"O.O#OO"];
        assert_eq!(float_up(create_matrix(input)), 
        [[1, 1, 0, 0],
        [1, 1, 0, 0],
        [1, 0, 0, 0],
        [0, 0, 0, -66],
        [0, -66, 1, 0],
        [-66, 1, 0, 0]]);
    }

    #[test]
    fn test_calc_col_sum() {
        assert_eq!(calc_col_sum(vec![1, 1, 0, -66]), 7);
        assert_eq!(calc_col_sum(vec![0, 1, 0, -66]), 3);
    }
}
