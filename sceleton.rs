use std::collections::HashMap;
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

fn read_file_into_lines(filename: &str) -> Result<(Vec<String>, HashMap<String, u32>), Box<dyn std::error::Error>> {
    let mut lines_vec: Vec<String> = Vec::new();
    let mut map: HashMap<String, u32> = HashMap::new();

    let lines = read_lines(filename)?;

    for line in lines {
        let ip = line?;
        let parts: Vec<&str> = ip.split(' ').collect();
        lines_vec.push(parts[0].to_string());
        let num = parts[1].parse::<u32>().map_err(|e| format!("Failed to parse number: {}", e))?;
        map.insert(parts[0].to_string(), num);
    }

    Ok((lines_vec, map))
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines_vec: Vec<String>;

    (lines_vec, _) = read_file_into_lines("input")?;

    println!("{}", sum_tuples(lines_vec));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_number() {
        let input = 12;
        assert_eq!(input, 12);
    }

}
