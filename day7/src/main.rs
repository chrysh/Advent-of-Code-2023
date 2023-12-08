use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq)]
enum Rank {
    HighCard,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

#[derive(Debug, PartialEq)]
enum State {
    CouldBeFullhouse,
    CouldBeTwoPair,
    Uninitialized,
}

static ORDER: &'static [char; 13] = &[
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

fn rank_to_usize(rank: Rank) -> usize {
    match rank {
        Rank::HighCard => 0,
        Rank::OnePair => 1,
        Rank::TwoPair => 2,
        Rank::Three => 3,
        Rank::FullHouse => 4,
        Rank::Four => 5,
        Rank::Five => 6,
    }
}

fn find_rank(cards: &str) -> Rank {
    let mut card_count = [0; 13];
    let mut ids = Vec::new();

    println!("{}", cards);

    for card in cards.chars() {
        let card_value = ORDER.iter().position(|&c| c == card).unwrap();
        card_count[card_value] += 1;
        if card_count[card_value] == 1 {
            ids.push(card_value);
        }
    }

    println!("{:?}", ids);
    println!("{:?}", card_count);

    let mut state = State::Uninitialized;
    for id in ids {
        println!("{} {} {:?}", id, card_count[id], state);
        if state == State::CouldBeFullhouse {
            if card_count[id] == 2 {
                return Rank::FullHouse;
            }
        } else if state == State::CouldBeTwoPair {
            if card_count[id] == 3 {
                return Rank::FullHouse;
            } else if card_count[id] == 2 {
                return Rank::TwoPair;
            }
        } else if card_count[id] == 5 {
            return Rank::Five;
        } else if card_count[id] == 4 {
            return Rank::Four;
        } else if card_count[id] == 3 {
            state = State::CouldBeFullhouse;
            continue;
        } else if card_count[id] == 2 {
            state = State::CouldBeTwoPair;
            continue;
        }
    }

    if state == State::Uninitialized {
        return Rank::HighCard;
    }

    if state == State::CouldBeFullhouse {
        return Rank::Three;
    }

    if state == State::CouldBeTwoPair {
        return Rank::OnePair;
    }
    Rank::HighCard
}

fn sort_rank_piles(cards: Vec<&str>) -> Vec<Vec<&str>> {
    let mut piles: Vec<Vec<&str>> = vec![Vec::new(); 7];
    for card in cards {
        println!("card: {} rank: {}", card, rank_to_usize(find_rank(card)));
        let rank = find_rank(card);
        let rn = rank_to_usize(rank);
        piles[rn].push(card);
    }
    for pile in &piles {
        println!("{:?}", pile);
    }
    piles
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

fn main() -> io::Result<()> {
    let mut lines_vec: Vec<String> = Vec::new();
    let mut map: HashMap<String, u32> = HashMap::new();

    if let Ok(lines) = read_lines("input") {
        for line in lines {
            if let Ok(ip) = line {
                let parts: Vec<&str> = ip.split(' ').collect();
                println!("{:?}", parts[0]);
                lines_vec.push(parts[0].to_string());
                map.insert(parts[0].to_string(), parts[1].parse::<u32>().unwrap());
            }
        }
    }

    println!("{}", sum_all(lines_vec, map));
    Ok(())
}

fn sum_all(lines_vec: Vec<String>, map: HashMap<String, u32>) -> u32 {
    let vec_of_str: Vec<&str> = lines_vec.iter().map(|s| s.as_str()).collect();
    let mut piles = sort_rank_piles(vec_of_str);

    for p in &mut piles {
        p.sort_by(cmp_cards);
    }

    println!("{:?}", piles);

    let mut sum = 0;
    let mut i: u32 = 1;
    for p in piles {
        for card in p {
            let x = (*map.get(card).unwrap() * i);
            println!("{} = {} * {}", x, x / i, i);
            sum += x;
            i += 1;
        }
    }
    sum
}

fn compare_card(a: char, b: char) -> std::cmp::Ordering {
    let a_value = ORDER.iter().position(|&c| c == a).unwrap();
    let b_value = ORDER.iter().position(|&c| c == b).unwrap();
    a_value.cmp(&b_value)
}

fn cmp_cards(a: &&str, b: &&str) -> std::cmp::Ordering {
    let mut a_chars = (*a).chars();
    let mut b_chars = (*b).chars();

    loop {
        match (a_chars.next(), b_chars.next()) {
            (Some(a), Some(b)) => match compare_card(a, b) {
                std::cmp::Ordering::Equal => continue,
                std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
            },
            (None, None) => return std::cmp::Ordering::Equal,
            (None, Some(_)) => return std::cmp::Ordering::Less,
            (Some(_), None) => return std::cmp::Ordering::Greater,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_basic() {
        assert_eq!(compare_card('A', 'A'), std::cmp::Ordering::Equal);
        assert_eq!(compare_card('A', 'T'), std::cmp::Ordering::Greater);
        assert_eq!(compare_card('2', '3'), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_compare_types() {
        assert_eq!(find_rank("AAAAA"), Rank::Five);
        assert_eq!(find_rank("AA8AA"), Rank::Four);
        assert_eq!(find_rank("TTT98"), Rank::Three);
        assert_eq!(find_rank("23332"), Rank::FullHouse);
        assert_eq!(find_rank("23432"), Rank::TwoPair);
        assert_eq!(find_rank("A23A4"), Rank::OnePair);
        assert_eq!(find_rank("23456"), Rank::HighCard);
        assert_eq!(find_rank("33332"), Rank::Four);
        assert_eq!(find_rank("2AAAA"), Rank::Four);
        assert_eq!(find_rank("77888"), Rank::FullHouse);
        assert_eq!(find_rank("77788"), Rank::FullHouse);
        assert_eq!(find_rank("32T3K"), Rank::OnePair);
        assert_eq!(find_rank("KTJJT"), Rank::TwoPair);
        assert_eq!(find_rank("KK677"), Rank::TwoPair);
        assert_eq!(find_rank("T55J5"), Rank::Three);
        assert_eq!(find_rank("T55J5"), Rank::Three);
    }

    #[test]
    fn test_check_piles() {
        let mut piles: Vec<Vec<&str>> = vec![Vec::new(); 7];
        let cards: [&str; 16] = [
            "AAAAA", "AA8AA", "TTT98", "23332", "23432", "A23A4", "23456", "33332", "2AAAA",
            "77888", "77788", "32T3K", "KTJJT", "KK677", "T55J5", "T55J5",
        ];

        for card in cards {
            println!("card: {} rank: {}", card, rank_to_usize(find_rank(card)));
            let rank = find_rank(card);
            let rn = rank_to_usize(rank);
            piles[rn].push(card);
        }

        assert_eq!(piles[rank_to_usize(Rank::Five)].len(), 1);
        assert_eq!(piles[rank_to_usize(Rank::TwoPair)].len(), 3);
        assert_eq!(piles[rank_to_usize(Rank::Four)].len(), 3);
        assert_eq!(piles[rank_to_usize(Rank::FullHouse)].len(), 3);
        assert_eq!(piles[rank_to_usize(Rank::Three)].len(), 3);
        assert_eq!(piles[rank_to_usize(Rank::OnePair)].len(), 2);
        assert_eq!(piles[rank_to_usize(Rank::HighCard)].len(), 1);
    }

    #[test]
    fn test_check_sort_ranked_piles() {
        let mut piles: Vec<Vec<&str>> = vec![Vec::new(); 7];
        let cards: [&str; 15] = [
            "AA8AA", "2AAAA", "TTT98", "23332", "23432", "A23A4", "23456", "33332", "77888",
            "77788", "32T3K", "KTJJT", "KK677", "T55J5", "AQ68J",
        ];

        for card in cards {
            println!("card: {} rank: {}", card, rank_to_usize(find_rank(card)));
            let rank = find_rank(card);
            let rn = rank_to_usize(rank);
            piles[rn].push(card);
        }

        for p in &mut piles {
            p.sort_by(cmp_cards);
        }
        assert_eq!(
            piles[rank_to_usize(Rank::Four)],
            ["2AAAA", "33332", "AA8AA"]
        );
        assert_eq!(
            piles[rank_to_usize(Rank::FullHouse)],
            ["23332", "77788", "77888"]
        );
        assert_eq!(piles[rank_to_usize(Rank::Three)], ["T55J5", "TTT98"]);
        assert_eq!(
            piles[rank_to_usize(Rank::TwoPair)],
            ["23432", "KTJJT", "KK677"]
        );
        assert_eq!(piles[rank_to_usize(Rank::OnePair)], ["32T3K", "A23A4"]);
        assert_eq!(piles[rank_to_usize(Rank::HighCard)], ["23456", "AQ68J"]);
    }

    #[test]
    fn test_sum() {
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
        let mut lines_vec: Vec<String> = Vec::new();
        let mut map = HashMap::new();

        for line in input.lines() {
            let parts: Vec<&str> = line.split(' ').collect();
            println!("{:?}", parts[0]);
            lines_vec.push(parts[0].to_string());
            map.insert(parts[0].to_string(), parts[1].parse::<u32>().unwrap());
        }
        assert_eq!(sum_all(lines_vec, map), 6440);
    }
}
