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
const RANKS_IN_DECK: usize = 7;

#[derive(Debug, PartialEq)]
enum State {
    CouldBeFullhouse,
    CouldBeTwoPair,
    Uninitialized,
}
const CARDS_IN_SUIT: usize = 13;
static ORDER: &'static [char; CARDS_IN_SUIT] = &[
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

impl From<Rank> for usize {
    fn from(rank: Rank) -> usize {
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
}

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

fn get_rank(ids: Vec<usize>, card_count: [u32; CARDS_IN_SUIT]) -> Rank {
    let mut state = State::Uninitialized;
    for id in ids {
        dbg!("{} {} {:?}", id, card_count[id], state);

        match card_count[id] {
            5 => return Rank::Five,
            4 => return Rank::Four,
            3 => {
                if state == State::CouldBeTwoPair {
                    return Rank::FullHouse;
                } else {
                    state = State::CouldBeFullhouse;
                    continue;
                }
            }
            2 => {
                if state == State::CouldBeFullhouse {
                    return Rank::FullHouse;
                } else if state == State::CouldBeTwoPair {
                    return Rank::TwoPair;
                } else {
                    state = State::CouldBeTwoPair;
                    continue;
                }
            }
            _ => continue,
        }

        // if state == State::CouldBeFullhouse {
        //     if card_count[id] == 2 {
        //         return Rank::FullHouse;
        //     }
        // } else if state == State::CouldBeTwoPair {
        //     if card_count[id] == 3 {
        //         return Rank::FullHouse;
        //     } else if card_count[id] == 2 {
        //         return Rank::TwoPair;
        //     }
        // } else if card_count[id] == 5 {
        //     return Rank::Five;
        // } else if card_count[id] == 4 {
        //     return Rank::Four;
        // } else if card_count[id] == 3 {
        //     state = State::CouldBeFullhouse;
        //     continue;
        // } else if card_count[id] == 2 {
        //     state = State::CouldBeTwoPair;
        //     continue;
        // }
    }
    match state {
        State::Uninitialized => Rank::HighCard,
        State::CouldBeFullhouse => Rank::Three,
        State::CouldBeTwoPair => Rank::OnePair,
        _ => Rank::HighCard,
    }
}

fn get_card_count(cards: &str) -> (Vec<usize>, [u32; CARDS_IN_SUIT]) {
    let mut card_count = [0; CARDS_IN_SUIT];
    let mut ids = Vec::new();

    for card in cards.chars() {
        let card_value = ORDER.iter().position(|&c| c == card).unwrap();
        card_count[card_value] += 1;
        if card_count[card_value] == 1 {
            ids.push(card_value);
        }
    }
    (ids, card_count)
}

fn find_rank(cards: &str) -> Rank {
    let card_count;
    let ids;

    dbg!("{}", cards);

    (ids, card_count) = get_card_count(cards);

    dbg!("{:?}", ids);
    dbg!("{:?}", card_count);

    get_rank(ids, card_count)
}

fn sort_rank_piles(cards: Vec<&str>) -> Vec<Vec<&str>> {
    let mut piles: Vec<Vec<&str>> = vec![Vec::new(); RANKS_IN_DECK];
    for card in cards {
        dbg!("card: {} rank: {}", card, rank_to_usize(find_rank(card)));
        let rank = find_rank(card);
        let rn = rank_to_usize(rank);
        piles[rn].push(card);
    }
    piles
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
    let map: HashMap<String, u32>;

    (lines_vec, map) = read_file_into_lines("input")?;

    println!("{}", sum_all(lines_vec, map));
    Ok(())
}

fn sum_all(lines_vec: Vec<String>, map: HashMap<String, u32>) -> u32 {
    let vec_of_str: Vec<&str> = lines_vec.iter().map(|s| s.as_str()).collect();
    let mut piles = sort_rank_piles(vec_of_str);

    for p in &mut piles {
        p.sort_by(cmp_cards);
    }

    dbg!("{:?}", piles);

    let mut sum = 0;
    let mut i: u32 = 1;
    for p in piles {
        for card in p {
            let x = *map.get(card).unwrap() * i;
            dbg!("{} = {} * {}", x, x / i, i);
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

    a_chars
        .zip(b_chars)
        .find_map(|(a,b)| match compare_card(a,b) {
                std::cmp::Ordering::Equal => None,
                other => Some(other),
        })
        .unwrap_or_else(|| std::cmp::Ordering::Equal)
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
        let mut piles: Vec<Vec<&str>> = vec![Vec::new(); RANKS_IN_DECK];
        let cards: [&str; 16] = [
            "AAAAA", "AA8AA", "TTT98", "23332", "23432", "A23A4", "23456", "33332", "2AAAA",
            "77888", "77788", "32T3K", "KTJJT", "KK677", "T55J5", "T55J5",
        ];

        for card in cards {
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
        let mut piles: Vec<Vec<&str>> = vec![Vec::new(); RANKS_IN_DECK];
        let cards: [&str; 15] = [
            "AA8AA", "2AAAA", "TTT98", "23332", "23432", "A23A4", "23456", "33332", "77888",
            "77788", "32T3K", "KTJJT", "KK677", "T55J5", "AQ68J",
        ];

        for card in cards {
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
            dbg!("{:?}", parts[0]);
            lines_vec.push(parts[0].to_string());
            map.insert(parts[0].to_string(), parts[1].parse::<u32>().unwrap());
        }
        assert_eq!(sum_all(lines_vec, map), 6440);
    }
}
