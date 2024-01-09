use scan_fmt::scan_fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rayon::prelude::*;

macro_rules! dbg {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            println!($($arg)*);
        }
    };
}

#[derive(Debug, Clone)]
struct Spring {
    str: String,
    states: Vec<i8>,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_file_into_vec(
    filename: &str,
) -> Result<Vec<Spring>, Box<dyn std::error::Error>> {
    let lines = read_lines(filename)?;
    let mut res: Vec<Spring> = Vec::new();
    for line in lines {
        let str;
        let states;
        let ip = line?;
        (str, states) = parse_input(&ip).unwrap_or_else(|_| {
            dbg!("Failed parsing input: {}", ip);
            panic!("Exiting due to parse error");
        });
        res.push(Spring{str, states});
    }

    Ok(res)
}

fn parse_input(input: &str) -> Result<(String, Vec<i8>), Box<dyn std::error::Error>> {
    dbg!("Parsing: {:?}", input);
    let (str, rest) = scan_fmt!(input, "{[.#?]} {[,0-9]}", String, String)?;
    dbg!("Parsed: {:?}, {:?}", str, rest);

    let states: Vec<i8> = rest
        .split(',')
        .filter_map(|x| x.parse::<i8>().ok())
        .collect::<Vec<i8>>();

    Ok((str, states))
}

fn generate_permutations(str: String) -> Vec<String> {
    if let Some(index) = str.find('?') {
        let mut str1 = str.clone();
        str1.replace_range(index..index + 1, "#");
        let mut str2 = str.clone();
        str2.replace_range(index..index + 1, ".");
        let mut result = generate_permutations(str1);
        result.extend(generate_permutations(str2));
        result
    } else {
        vec![str]
    }
}

fn check_permutations(vec: Vec<String>, pattern: Vec<i8>) -> Vec<String> {
    let mut result = Vec::new();
    'outer: for str in vec {
        if str.matches('#').count() != pattern.iter().sum::<i8>() as usize {
            continue;
        }
        let mut start = 0;
        dbg!("Check for {:?}", str);

        //if str == "#.#.###" {let mut start = 0;
            for (i, n) in pattern.iter().enumerate() {
                dbg!("Check for pattern {:?}", n);
                if start >= str.len() {
                    dbg!("No match found for {:?}", str);
                    continue 'outer;
                }
                if let Some(index) = str[start..].find('#') {
                    start += index;
                    let segment_end = start + *n as usize;

                    // Check if the segment is n long and followed by a '.' or end of string
                    dbg!("segment_end > str.len() : {}", segment_end > str.len());
                    if segment_end > str.len() {
                        dbg!("No match found for {:?}", str);
                        continue 'outer;
                    } else if str[start..segment_end] == "#".repeat(*n as usize) {
                        dbg!("(segment_end == str.len() || str.chars().nth(segment_end) == Some('.')) : {}", (segment_end == str.len() || str.chars().nth(segment_end) == Some('.')));
                        if segment_end == str.len() || str.chars().nth(segment_end) == Some('.') {
                                dbg!("Found pattern match at {:?}", start);
                                start = segment_end + 1; // Skip the '.' character
                                dbg!("i == pattern.len(): {}, i {}", i == (pattern.len()-1), i);
                                dbg!("start >= str.len(): {}", start >= str.len() );
                                dbg!("str[start..].chars().all(|c| c == '.'): {}",  start <str.len() && str[start..].chars().all(|c| c == '.'));
                                if i == (pattern.len()-1)
                                && (start >= str.len() 
                                || str[start..].chars().all(|c| c == '.'))   {
                                    dbg!("++++ Found match: {:?}", str);
                                    result.push(str);
                                    continue 'outer;
                                }
                                continue;
                        }
                    }
                    dbg!("No match found for {:?}", str);
                    continue 'outer;
            }
        }
    }
    result
}

fn get_arrangement_cnts(data: Vec<Spring>) -> Vec<usize> {
    let res: Vec<usize> = data
        .par_iter()
        .map(|s| check_permutations(generate_permutations(s.str.clone()), s.states.clone()).len())
        .collect();
    dbg!("Result: {:?}", res);
    res
}

#[derive(PartialEq, Clone)]
enum States {
    ParsingInitial,
    ParsingBang,
    ParsingRest,
}

fn generate_words(mut s: Spring, mut idx: usize, mut bang_count: i8, mut state: States) -> i16 {
    let mut ret = 0;
    while idx < s.str.len() && s.states.len() > 0 {
        if state == States::ParsingBang {

        while idx < s.str.len() && s.str.get(idx..idx+1) == Some("#") {
            if bang_count > s.states[0] {
                // Abort generation, this word is not in my alphabet
                return 0;
            }
            idx += 1;
            bang_count += 1;
        }
        if s.str.get(idx..idx+1) == Some("?") {
            if bang_count < s.states[0] {
                s.str.replace_range(idx..idx+1, "#");
            } else { // bang_count == state[0]
                s.str.replace_range(idx..idx+1, ".");
            }
        } else {  // Encountered .
            if bang_count != s.states[0] {
                // Abort generation, this word is not in my alphabet
                return 0;
            }
            s.states.remove(0);
            state = States::ParsingRest;
            idx += 1;
        }
    } else if state == States::ParsingRest {
        // FIXME: Transform to match
        while idx < s.str.len() {
        while idx < s.str.len() && s.str.get(idx..idx+1) == Some(".") {
            idx += 1;
        }
        if s.str.get(idx..idx+1) == Some("#") {
            bang_count = 0;
            state = States::ParsingBang;
            idx += 1;
        } else if s.str.get(idx..idx+1) == Some("?") {
            let new_spring = Spring{ str: s.str.clone(), states: s.states.clone()};
            ret += generate_words(new_spring, idx, bang_count, state.clone());
            ret += generate_words(s, idx, bang_count, state);
            return ret;
        }

    }
    } else if state == States::ParsingInitial {
        if s.str.get(idx..idx+1) == Some("#") {
            state = States::ParsingBang;
        } else {
            state = States::ParsingRest;    
        }
        idx += 1;

    }
}
    if s.states.len() > 0 {
        return 0;
    }
    1
}

fn get_number_perms(data: Vec<Spring>) -> Vec<i16> {
    let mut res : Vec<i16> = Vec::new();
    for s in data {
        res.push(generate_words(s, 0, 0, States::ParsingInitial));
    }
    res
}

fn get_arrangement_sum(res: Vec<Spring>) -> usize {
    get_arrangement_cnts(res).par_iter().sum()
}

fn generate_input_part2(data: Vec<Spring>) -> Vec<Spring> {
    let mut result: Vec<Spring> = Vec::new();
    for spring in data {
        let new_str = (spring.str.clone() + "?").repeat(5);
        let new_states = spring.states.clone().repeat(5);
        // FIXME: decide already here on values?
        result.push(Spring { str: new_str, states: new_states});
    }
    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_file_into_vec("input")?;
    let input_part2 = generate_input_part2(input.clone());

    println!("PART1: {:?}", get_arrangement_sum(input));
    println!("PART2: {:?}", get_arrangement_sum(input_part2));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_arrangement_cnts() {
        let _data = read_file_into_vec("example1.input").expect("Failed to read file");
        assert_eq!(get_arrangement_cnts(_data), vec![1, 4, 1, 1, 4, 10]);
    }

    #[test]
    fn test_get_arrangement_cnts_part2() {
        let data = read_file_into_vec("example1.input").expect("Failed to read file");
        let data2 = generate_input_part2(data);
        dbg!("{:?}", data2);
        assert_eq!(get_arrangement_cnts(data2), vec![1, 16384, 1, 16, 2500, 506250]);
        //assert!(false);
    }

    #[test]
    fn test_generate_permutations() {
        let perms = generate_permutations("???.###".to_string());

        dbg!("{:?}", perms);
        assert_eq!(perms[0], "###.###");
        assert_eq!(perms.len(), 8);
    }

    #[test]
    fn test_check_permutations() {
        let perms = generate_permutations("???.###".to_string());

        dbg!("{:?}", perms);
        let mut result = check_permutations(perms, vec![1, 1, 3]);
        dbg!("{:?}", result);
        assert_eq!(result.len(), 1);
    }
    
    macro_rules! test_arrangement {
        ($str:expr, $states:expr, $expected:expr) => {
            assert_eq!(get_arrangement_cnts(vec![Spring{str: $str, states: $states}])[0], $expected);
        };
    }
    
        #[test]
        fn test_statemachine() {
            test_arrangement!("???.###".to_string(), vec![1,1,3], 1);
            test_arrangement!(".??..??...?##.".to_string(), vec![1,1,3], 4);
        }
    
    #[test]
    fn test_nom_next_input() {
        test_arrangement!("???.###".to_string(), vec![1,1,3], 1);
        test_arrangement!(".??..??...?##.".to_string(), vec![1,1,3], 4);
        test_arrangement!("?#?#?#?#?#?#?#?".to_string(), vec![1,3,1,6], 1);
        test_arrangement!("????.".to_string(), vec![4], 1);
        test_arrangement!("????.#...#...".to_string(), vec![4,1,1], 1);
        test_arrangement!("????.######..#####.".to_string(), vec![1,6,5], 4);
        test_arrangement!("????.#####..#####.".to_string(), vec![1,6,5], 0);
    }
    
    #[test]
    fn test_nom_next_input_basic1() {
        test_arrangement!("?###????????".to_string(), vec![3,2,1], 10);
        test_arrangement!("???????".to_string(), vec![2,1], 10);
        test_arrangement!("?????".to_string(), vec![2, 1], 3);
        test_arrangement!(".?#??#.#".to_string(), vec![1,2,1], 1);
        test_arrangement!("#??#".to_string(), vec![1,2], 1);
        test_arrangement!("#??".to_string(), vec![1,1], 1);
    }

    #[test]
    fn test_nom_next_input_basic2() {
    /*     test_arrangement!(('#'.to_string(), vec![1], 1);
        test_arrangement!((".#".to_string(), vec![1], 1);
        test_arrangement!((".#.".to_string(), vec![1], 1);
        test_arrangement!(("##.".to_string(), vec![1], 0);
        test_arrangement!(("..".to_string(), vec![1], 0);
        test_arrangement!(("..#.##.#".to_string(), vec![1,2,1], 1);
        test_arrangement!(("..#.##.#".to_string(), vec![1,1,2], 0);
        */
    }
}
