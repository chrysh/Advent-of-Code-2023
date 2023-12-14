use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
enum state {
    U, D, L, R, N
}

macro_rules! dbg {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            println!($($arg)*);
        }
    };
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

fn steps(inp: Vec<Vec<char>>) -> i32 {
    42
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_file_into_lines(filename: &str) -> Result<(Vec<Vec<char>>, HashMap<String, u32>), Box<dyn std::error::Error>> {
    let mut lines_vec: Vec<Vec<char>> = Vec::new();
    let mut map: HashMap<String, u32> = HashMap::new();

    let lines = read_lines(filename)?;

    for line in lines {
        let ip = line?;
        lines_vec.push(ip.chars().collect());
    }

    Ok((lines_vec, map))
}

fn get_next_state(state: state, c: char) -> state {
    match state {       
        state::R => match c {
            'J' => state::U,
            '7' => state::D,   
            '-' => state::R,
            _ => state::N,
        },        
        state::L => match c {
            'L' => state::U,
            'F' => state::D,   
            '-' => state::L,   
            _ => state::N,

        },        
        state::D => match c {
            'J' => state::L,
            'L' => state::R,   
            '|' => state::D,   
            _ => state::N,

        },        
        state::U => match c {
            'F' => state::R,
            '7' => state::L,   
            '|' => state::U,   
            _ => state::N,
        },    
        state::N => state::N,
    }
}

fn walk_all(input: Vec<Vec<char>>) -> i32 {
    let mut x = 0;
    let mut y = 0;

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            
            if input[i][j] == 'S' {
                x = i;
                y = j;
                break;
            }
        }
    }
    let mut cnt = 0;
    
    let mut s = get_next_state(state::R, input[x][y+1]);
    println!("S: {} {} {:?}", x, y, s);
    if s != state::N {
        s = state::R;
        y = y + 1;
    } else {
        s = get_next_state(state::L, input[x][y-1]);
        println!("S: {} {} {:?}", x, y, s);
        if s != state::N {
            s = state::L;
            y = y - 1;
        } else {
            s = get_next_state(state::D, input[x+1][y]);
            println!("S: {} {} {:?}", x, y, s);
            if s != state::N {
                s = state::D;
                x = x + 1;
            } else {
                s = get_next_state(state::U, input[x-1][y]);
                println!("S: {} {} {:?}", x, y, s);
                if s != state::N {
                    s = state::U;
                    x = x - 1;
                } else {
                    panic!("No state");
                    
                }
            }
        }
    }

    println!("Next S: {:?}", s);
    let mut state = s;

    while input[x][y] != 'S' {
        dbg!("{} {} {} s: {:?}", x, y, input[x][y], state);
        let s = get_next_state(state, input[x][y]);
        dbg!("{} {} {} s: {:?}", x, y, input[x][y], &s);
        match s {
            state::R => y += 1,
            state::L => y -= 1,
            state::D => x += 1,
            state::U => x -= 1,
            state::N => panic!("No state"),
        }
        state = s;
        dbg!("{} {} {} s: {:?}", x, y, input[x][y], &state);
        cnt += 1;
    }
    println!("{}", cnt);
    cnt
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines_vec: Vec<Vec<char>>;

    (lines_vec, _) = read_file_into_lines("input")?;

    println!("Distance: {}", walk_all(lines_vec)/2+1);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_number() {
        let input_lines = "7-F7-
        .FJ|7
        FSLL7
        |F--J
        LJ.LJ".replace(" ", "");
        let inp_vec = input_lines.lines().map(|x| x.chars().collect()).collect();

        for line in &inp_vec {
            println!("{:?}", line);
        }
        // FIXME: Initial step is to the left, so it's not generic
        assert_eq!(walk_all(inp_vec), 15);
    }

    #[test]
    fn test_next_state() {

        assert_eq!(get_next_state(state::R, 'J'), state::U);
        assert_eq!(get_next_state(state::D, 'J'), state::L);
        assert_eq!(get_next_state(state::U, 'F'), state::R);
        assert_eq!(get_next_state(state::L, 'F'), state::D);
        assert_eq!(get_next_state(state::R, '7'), state::D);
        assert_eq!(get_next_state(state::U, '7'), state::L);
        assert_eq!(get_next_state(state::D, 'L'), state::R);
        assert_eq!(get_next_state(state::L, 'L'), state::U);
        assert_eq!(get_next_state(state::U, '|'), state::U);
        assert_eq!(get_next_state(state::D, '|'), state::D);
        assert_eq!(get_next_state(state::R, '-'), state::R);
        assert_eq!(get_next_state(state::L, '-'), state::L);
    }


    #[test]
    fn test_replace() {
        let input_lines = "7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ".replace(" ", "");
        let inp_vec: Vec<Vec<char>>  = input_lines.lines().map(|x| x.chars().collect()).collect();

        let output_lines = "..45.
        .236.
        01.78
        14567
        23...".replace(" ", "");
        let out_vec: Vec<Vec<char>> = input_lines.lines().map(|x| x.chars().collect()).collect();

        assert_eq!(inp_vec, out_vec);
    }
}
