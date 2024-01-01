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

fn read_file_into_lines(filename: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut lines_vec: Vec<String> = Vec::new();

    let lines = read_lines(filename)?;

    for line in lines {
        let ip = line?;
        lines_vec.push(ip);
    }

    Ok(lines_vec)
}

fn parse_seeds(lines_vec: String) -> Vec<u64> {
    let seeds: Vec<&str> = lines_vec.split_whitespace().collect::<Vec<_>>();
    let seeds_u64: Vec<u64> = seeds[1..]
        .iter()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();
    dbg!("seeds: {:?}", seeds);
    dbg!("seeds_u64: {:?}", seeds_u64);
    seeds_u64
}

fn parse_to_vec(lines_vec: &mut Vec<String>, nextbreakstr: &str, map: &mut Vec<(u64, u64, u64)>) {
    println!("Len of vec before parse: {:?}", lines_vec.clone().len());
    while let Some(line) = lines_vec.get(0).cloned() {
        let ip = line.clone();
        if ip == nextbreakstr {
            dbg!("Breaking up because found {:?}", ip);
            break;
        }
        lines_vec.remove(0);
        let elems = line
            .split(' ')
            .filter_map(|x| x.parse::<u64>().ok())
            .collect::<Vec<u64>>();

        if elems != [] {
            map.push((elems[0], elems[1], elems[2]));
        }
    }
    println!("Len of vec after parse: {:?}", lines_vec.len());
    ()
}

fn find_table_entry(table: &Vec<(u64, u64, u64)>, val: u64) -> u64 {
    let len: usize = table.len();
    let mut start: usize = 0usize;
    let mut stop: usize = len - 1;
    let mut middle: usize;
    println!("A{} E{} v{}", start, stop, val);

    loop {
        middle = start + (stop - start) / 2;
        println!("Loop: A{} E{} m{}", start, stop, middle);
        // FIXME: what if middle+1 == stop+1?
        // FIXME: What happens for val < table[0] ?
        if val < table[start].1 {
            middle = start;
            break;
        } else if val > table[len - 1].1 {
            middle = stop;
            break;
        } else if val >= table[middle].1 && val < table[middle + 1].1 {
            break;
        } else if val >= table[middle].1 && val <= table[stop].1 {
            start = middle
        } else if val >= table[start].1 && val < table[middle].1 {
            stop = middle
        }
    }

    println!("After: A{} E{} m{}", start, stop, middle);
    println!(
        "val: {} tabmid0: {} tabmid1: {} tabmid2: {}",
        val, table[middle].0, table[middle].1, table[middle].2
    );

    val - table[middle].1 + table[middle].0
}

fn parse_tables(mut lines_vec: Vec<String>) -> [Vec<(u64, u64, u64)>; 7] {
    let break_strings = [
        "seed-to-soil map:",
        "soil-to-fertilizer map:",
        "fertilizer-to-water map:",
        "water-to-light map:",
        "light-to-temperature map:",
        "temperature-to-humidity map:",
        "humidity-to-location map:",
        "",
    ];

    let seed2soil: Vec<(u64, u64, u64)> = Vec::new();
    let soil2fert: Vec<(u64, u64, u64)> = Vec::new();
    let fert2water: Vec<(u64, u64, u64)> = Vec::new();
    let water2light: Vec<(u64, u64, u64)> = Vec::new();
    let light2temp: Vec<(u64, u64, u64)> = Vec::new();
    let temp2hum: Vec<(u64, u64, u64)> = Vec::new();
    let hum2loc: Vec<(u64, u64, u64)> = Vec::new();

    let mut tables: [Vec<(u64, u64, u64)>; 7] = [
        seed2soil,
        soil2fert,
        fert2water,
        water2light,
        light2temp,
        temp2hum,
        hum2loc,
    ];

    for i in 0..tables.len() {
        parse_to_vec(&mut lines_vec, break_strings[i + 1], &mut tables[i]);
        println!("Len of table {} after parse: {:?}", i, tables[i].len());
        tables[i].sort_by(|a, b| a.1.cmp(&b.1));
        let first =  tables[i][0].1;
        if first > 0 {
            tables[i].insert(0, (0, 0, first));
        }
        let x = tables[i].len()-1;
        let y = tables[i][x].1 + tables[i][x].2;
        tables[i].push((y, y, 10000));
        println!("table: {:?}", tables[i]);
    }

    tables
}

fn solve_part1(mut lines_vec: Vec<String>) -> Vec<u64> {
    let first_line = lines_vec.remove(0);
    let seeds = parse_seeds(first_line.to_string());
    lines_vec.remove(0);

    let tables = parse_tables(lines_vec);

    seed2locations(seeds, &tables)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines_vec = read_file_into_lines("input").expect("Something is wrong with input!");

    println!("PART1: {:?}", solve_part1(lines_vec).iter().min());
    Ok(())
}

fn seed2locations(seeds: Vec<u64>, tables: &[Vec<(u64, u64, u64)>; 7]) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();
    for i in seeds {
        let soil = find_table_entry(&tables[0], i);
        let fert = find_table_entry(&tables[1], soil);
        let water = find_table_entry(&tables[2], fert);
        let light = find_table_entry(&tables[3], water);
        let temp = find_table_entry(&tables[4], light);
        let hum = find_table_entry(&tables[5], temp);
        let loc = find_table_entry(&tables[6], hum);
        println!(
            "#### {} {} {} {} {} {} {} {}",
            i, soil, fert, water, light, temp, hum, loc
        );
        if loc != 0 {
            println!("r: {}", loc);
            res.push(loc);
        }
    }
    println!("{:?}", res);
    println!("{:?}", res.iter().min());
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_seeds() {
        let seeds = parse_seeds("seeds: 1 2 3 4 5\n".to_string());
        assert_eq!(seeds, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_parse_soil() {
        let mut seed2soil: Vec<(u64, u64, u64)> = Vec::new();
        let input = "1 2 3\n4 5 6\n10 2 5\nFoo".to_string();
        let mut lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();

        parse_to_vec(&mut lines, "Foo", &mut seed2soil);
        assert_eq!(seed2soil, [(1, 2, 3), (4, 5, 6), (10, 2, 5)]);
    }

    #[test]
    fn test_parse_soil2() {
        let mut vec_foo: Vec<(u64, u64, u64)> = Vec::new();
        let mut vec_bar: Vec<(u64, u64, u64)> = Vec::new();
        let input = "1 2 3\n4 5 6\n10 2 5\nFoo\n1 1 1\n5 5 5\nBar".to_string();
        let mut lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();

        parse_to_vec(&mut lines, "Foo", &mut vec_foo);
        parse_to_vec(&mut lines, "Bar", &mut vec_bar);
        assert_eq!(vec_foo, [(1, 2, 3), (4, 5, 6), (10, 2, 5)]);
        assert_eq!(vec_bar, [(1, 1, 1), (5, 5, 5)]);
    }

    #[test]
    fn test_find_table_entry() {
        let table: Vec<(u64, u64, u64)> = vec![(0, 0, 2), (1, 2, 3), (4, 5, 6), (10, 11, 4), (15, 15, 100)];
        assert_eq!(find_table_entry(&table, 1), 1);
        assert_eq!(find_table_entry(&table, 3), 2);
        assert_eq!(find_table_entry(&table, 4), 3);
        assert_eq!(find_table_entry(&table, 9), 8);
        assert_eq!(find_table_entry(&table, 11), 10);
        assert_eq!(find_table_entry(&table, 14), 13);
        // FIXME: assert_eq!(find_table_entry(&table, 15), None);
    }

    #[test]
    fn test_find_table_entry2() {
        let lines_vec = read_file_into_lines("example1.input").expect("Something is wrong with input!");

        assert_eq!(solve_part1(lines_vec), vec![82, 43, 86, 35])
    }

    #[test]
    fn test_seed2location() {
        let mut lines_vec = read_file_into_lines("example1.input").expect("Something is wrong with input!");
        lines_vec.remove(0);
        lines_vec.remove(0);
        lines_vec.remove(0);
        let tables = parse_tables(lines_vec);

        println!("{:?}", tables);
        assert_eq!(seed2locations(vec![79], &tables), vec![82]);
        assert_eq!(seed2locations(vec![14], &tables), vec![43]);
        assert_eq!(seed2locations(vec![55], &tables), vec![86]);
        assert_eq!(seed2locations(vec![13], &tables), vec![35]);
    }
}
