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
    println!("{}", sum_tuples(lines_vec));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_number() {
        assert_eq!(12, 12);
    }

}
