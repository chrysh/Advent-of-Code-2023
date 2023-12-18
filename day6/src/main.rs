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

fn read_file_into_lines(
    filename: &str,
) -> Result<(Vec<String>, HashMap<String, u32>), Box<dyn std::error::Error>> {
    let mut lines_vec: Vec<String> = Vec::new();
    let mut map: HashMap<String, u32> = HashMap::new();

    let lines = read_lines(filename)?;

    for line in lines {
        let ip = line?;
        let parts: Vec<&str> = ip.split(' ').collect();
        lines_vec.push(parts[0].to_string());
        let num = parts[1]
            .parse::<u32>()
            .map_err(|e| format!("Failed to parse number: {}", e))?;
        map.insert(parts[0].to_string(), num);
    }

    Ok((lines_vec, map))
}

fn calc_possiblities_efficient(time: u32, dist: u32) -> u32 {
    let mut pos = 0;

    for i in 0..time / 2 {
        if i * (time * 2 - i) > dist {
            pos += 2;
        }
    }
    if time % 2 == 0 && pos > 0 {
        pos -= 1;
    }

    pos
}

fn calc_possiblities(time: u32, dist: u32) -> u32 {
    (0..time)
        .filter(|i|  i * (time - i) > dist)
        .count()
        .try_into()
        .unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Time:        54     94     65     92
    //Distance:   302   1476   1029   1404

    let times: [u32; 4] = [54, 94, 65, 92];
    let dist: [u32; 4] = [302, 1476, 1029, 1404];

    // for t in times {
    //     pos *= calc_possiblities(t, dist[i]);
    //     i += 1;
    // }
    // println!("{}", pos);

    // pos = 1;

    let pos = times
        .iter()
        .zip(dist)
        .map(|(t, d)| calc_possiblities(*t, d))
        .product::<u32>();

    println!("{}", pos);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_number() {
        //Time:      7  15   30
        //Distance:  9  40  200
        //4* 8 * 9 = 288

        assert_eq!(12, 12);
    }
}
