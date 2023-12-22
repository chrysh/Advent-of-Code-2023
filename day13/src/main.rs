use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::thread;
use std::collections::HashSet;
use rayon::prelude::*;

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

fn read_file_into_lines(filename: &str) -> Result<Vec<Vec<Vec<char>>>, Box<dyn std::error::Error>> {
    let mut patterns: Vec<Vec<Vec<char>>> = Vec::new();

    let lines = read_lines(filename)?;
    let pattern: Vec<Vec<char>> = Vec::new();

    let mut pat = pattern;
    for line in lines {
        let ip = line?;
        if !ip.is_empty() {
            pat.push(ip.chars().collect());
        } else {
            patterns.push(pat.clone());
            pat.clear();
        }
    }
    patterns.push(pat.clone());

    Ok(patterns)
}

fn line_is_mirrored(col: usize, line: &Vec<char>) -> bool {
    let len = line.len();
    let mut k = 0;
    while col >= k && col + 1 + k < len {
        if line[col - k] != line[col + 1 + k] {
            // line not  mirrored for column j
            //dbg!("Failed for {:?}, col: {}, k: {}", line, col, k);
            dbg!("Failed for col: {}, k: {}", col, k);
            return false;
        }
        k += 1;
    }
    true
}

fn get_mirrored_cols(matrix: &Vec<Vec<char>>) -> Vec<usize> {
    let first_line = matrix.get(0).unwrap();
    let len = first_line.len();

    let mut col_candidates = Vec::new();
    // check next column
    for i in 0..len - 1 {
        // find mirrored candidates
        if line_is_mirrored(i, first_line) {
            col_candidates.push(i);
        }
    }
    dbg!("cand: {:?}", col_candidates);

    let mut res = Vec::new();
    for col in col_candidates {
        dbg!("Checking col: {}", col);
        let mut all_mirrored = true;
        for l in matrix {
            if !line_is_mirrored(col, l) {
                // line not  mirrored for column col
                all_mirrored = false;
                break;
            }
        }
        if all_mirrored {
            res.push(col + 1); // We count from 1, not 0
        }
    }

    res
}

fn get_mirrored_rows(matrix: &[Vec<char>]) -> Vec<usize> {
    let transposed: Vec<Vec<char>> = (0..matrix[0].len())
        .map(|i| matrix.iter().map(|row| row[i]).collect())
        .collect();

    get_mirrored_cols(&transposed)
}

fn find_sum(patterns: Vec<Vec<Vec<char>>>) -> usize {
    let mut sum = 0;
    let reflections = find_reflections(patterns);
    dbg!("Found reflections: {:?}", reflections);

    for (cols, rows) in reflections {
        sum += if cols.is_empty() { 0 } else { cols[0] };
        sum += if rows.is_empty() { 0 } else { 100 * rows[0] };
    }
    sum
}

fn find_reflection(patterns: Vec<Vec<Vec<char>>>) -> Vec<(Vec<usize>, Vec<usize>)> {
    for pattern in patterns.iter() {
        let cols = get_mirrored_cols(pattern);
        let rows = get_mirrored_rows(pattern);
        if !cols.is_empty() || !rows.is_empty() {
            return vec![(cols, rows)];
        }
    }
    vec![]
}

fn find_reflections(patterns: Vec<Vec<Vec<char>>>) -> Vec<(Vec<usize>, Vec<usize>)> {
    let mut sums: Vec<(Vec<usize>, Vec<usize>)> = Vec::new();
    for pattern in patterns.iter() {
        sums.push((get_mirrored_cols(pattern), get_mirrored_rows(pattern)));
    }
    sums
}

fn search_smudges(pattern: &[Vec<char>]) ->  (Vec<usize>, Vec<usize>){
    let mut reflections: Vec<(Vec<usize>, Vec<usize>)> = Vec::new();
    let reference = find_reflection([pattern.to_owned()].to_vec());

    for (i, line) in pattern.iter().enumerate() {
        for (j, &c) in line.iter().enumerate() {
            let mut pattern_clone = pattern.to_owned();
            pattern_clone[i][j] = if c == '#' { '.' } else { '#' }; // Correct smudge
            dbg!("Checking smudge where {i}.{j} = {:?}", pattern_clone[i][j]);
            let refref = find_reflection([pattern_clone.clone()].to_vec());
            if !refref.is_empty() {
                dbg!("Found reflections: {:?}", refref);
                reflections.push(refref[0].clone());
            } else {
                pattern_clone[i][j] = if c == '#' { '.' } else { '#' }; // Change back smudge
            }
        }
    }
    let mut first_elements: HashSet<usize> = HashSet::new();
    let mut second_elements: HashSet<usize> = HashSet::new();
    
    for (first, second) in &reflections {
        first_elements.extend(first);
        second_elements.extend(second);
    }
    let (reference_first, reference_second) = &reference[0];

    let combined: (Vec<usize>, Vec<usize>) = (
        first_elements.into_iter().filter(|e| !reference_first.contains(e)).collect(),
        second_elements.into_iter().filter(|e| !reference_second.contains(e)).collect(),
    );

    dbg!("All combined: {:?}", combined);
    dbg!("All reference: {:?}", reference);

    combined
}

fn sum_smudges(reflections: (Vec<usize>, Vec<usize>)) -> usize {
    let mut sum = 0;
    let(cols, rows) = reflections;
    sum += if cols.is_empty() { 0 } else { cols[0] };
    sum += if rows.is_empty() { 0 } else { 100 * rows[0] };
    sum
}

fn run_part2_rayon_threads() -> usize {
    let patterns = read_file_into_lines("input").expect("Failed to read file");

    patterns.par_iter().map(|pat| sum_smudges(search_smudges(pat))).sum()
}

fn run_part2_threads() -> usize {
    let patterns = read_file_into_lines("input").expect("Failed to read file");

    let mut handles = Vec::new();

    for pat in patterns.into_iter() {
        let handle = thread::spawn(move || {
            let sum = sum_smudges(search_smudges(&pat));
            dbg!("Sum from thread: {}", sum);
            sum
        });
        handles.push(handle);
    }

    let mut sum = 0;
    for handle in handles {
        sum += handle.join().unwrap();
        dbg!("Sum from handle: {}", sum);
    }
    sum
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let patterns = read_file_into_lines("input")?;

    println!("PART1: {:?}", find_sum(patterns));
    println!("PART2 Sum: {}", run_part2_threads());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_pattern() {
        let patterns = read_file_into_lines("example1.input").expect("Failed to read file");
        let pattern_string: String = patterns[0][0].iter().collect();
        assert_eq!(pattern_string, "#.##..##.");
        let pattern_string2: String = patterns[1][0].iter().collect();
        assert_eq!(pattern_string2, "#...##..#");
    }

    #[test]
    fn get_vert_lines() {
        let patterns = read_file_into_lines("example1.input").expect("Failed to read file");
        assert_eq!(get_mirrored_cols(&patterns[0]), [5]);
        assert_eq!(get_mirrored_rows(&patterns[1]), [4]);
    }

    #[test]
    fn get_sum_one() {
        let patterns = read_file_into_lines("example1.input").expect("Failed to read file");
        assert_eq!(find_sum(patterns), 405);
    }

    #[test]
    fn get_sum_part2() {
        let patterns = read_file_into_lines("example1.input").expect("Failed to read file");
        assert_eq!(search_smudges(&patterns[0]), (vec![], vec![3]));
        assert_eq!(search_smudges(&patterns[1]), (vec![], vec![1]));
    }


    #[test]
    fn get_full_sum_part2() {
        let patterns = read_file_into_lines("example1.input").expect("Failed to read file");
        assert_eq!(sum_smudges(search_smudges(&patterns[0])), 300);
        assert_eq!(sum_smudges(search_smudges(&patterns[1])), 100);
    }    
}
