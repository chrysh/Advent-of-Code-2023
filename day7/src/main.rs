use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[allow(unused_macros)]
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

trait Cardgame {
    fn find_rank(&self, cards: &str) -> Rank;
    fn order() -> &'static [char];
    fn cmp_cards(a: &&str, b: &&str) -> std::cmp::Ordering;
    fn compare_card(a: char, b: char) -> std::cmp::Ordering;
}

struct Part1;
struct Part2;

impl Cardgame for Part1 {
    fn order() -> &'static [char] {
        &[
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ]
    }

    fn cmp_cards(a: &&str, b: &&str) -> std::cmp::Ordering {
        let a_chars = (*a).chars();
        let b_chars = (*b).chars();

        a_chars
            .zip(b_chars)
            .find_map(|(a, b)| match Self::compare_card(a, b) {
                std::cmp::Ordering::Equal => None,
                other => Some(other),
            })
            .unwrap_or(std::cmp::Ordering::Equal)
    }

    fn compare_card(a: char, b: char) -> std::cmp::Ordering {
        let a_value = Self::order().iter().position(|&c| c == a).unwrap();
        let b_value = Self::order().iter().position(|&c| c == b).unwrap();
        a_value.cmp(&b_value)
    }

    fn find_rank(&self, cards: &str) -> Rank {
        let card_count;
        let ids;

        (ids, card_count) = get_card_count(cards, self);

        get_rank(ids, card_count)
    }
}

impl Cardgame for Part2 {
    fn order() -> &'static [char] {
        &[
            'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
        ]
    }

    fn compare_card(a: char, b: char) -> std::cmp::Ordering {
        let a_value = Self::order().iter().position(|&c| c == a).unwrap();
        let b_value = Self::order().iter().position(|&c| c == b).unwrap();
        a_value.cmp(&b_value)
    }

    fn cmp_cards(a: &&str, b: &&str) -> std::cmp::Ordering {
        let a_chars = (*a).chars();
        let b_chars = (*b).chars();

        a_chars
            .zip(b_chars)
            .find_map(|(a, b)| match Self::compare_card(a, b) {
                std::cmp::Ordering::Equal => None,
                other => Some(other),
            })
            .unwrap_or(std::cmp::Ordering::Equal)
    }
    fn find_rank(&self, cards: &str) -> Rank {
        let mut card_count;
        let mut ids;

        (ids, card_count) = get_card_count(cards, self);

        ids.sort();

        // if joker in ids, then add number of jokers to the highest card_count
        let joker_id = Self::order().iter().position(|&c| c == 'J').unwrap();

        if ids.contains(&joker_id) {
            // find the max card_count from back of the list, add J to it
            let mut max = 0;
            let mut idx = None;
            for i in ids.iter().rev() {
                if card_count[*i] > max && *i != joker_id {
                    max = card_count[*i];
                    idx = Some(*i);
                }
            }
            if let Some(index) = idx {
                card_count[index] += card_count[joker_id];
                card_count[joker_id] = 0;
                ids.remove(joker_id);
            } else {
                card_count[CARDS_IN_SUIT - 1] += card_count[ids[joker_id]];
                card_count[ids[joker_id]] = 0;
                ids.remove(joker_id);
                ids.push(CARDS_IN_SUIT - 1);
            }
        }

        get_rank(ids, card_count)
    }
}

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
    }
    match state {
        State::Uninitialized => Rank::HighCard,
        State::CouldBeFullhouse => Rank::Three,
        State::CouldBeTwoPair => Rank::OnePair,
    }
}

fn get_card_count<T: Cardgame>(cards: &str, _game: &T) -> (Vec<usize>, [u32; CARDS_IN_SUIT]) {
    let mut card_count = [0; CARDS_IN_SUIT];
    let mut ids = Vec::new();

    for card in cards.chars() {
        let card_value = T::order().iter().position(|&c| c == card).unwrap();
        card_count[card_value] += 1;
        if card_count[card_value] == 1 {
            ids.push(card_value);
        }
    }
    (ids, card_count)
}

fn sort_rank_piles(cards: Vec<&str>, cardgame: impl Cardgame) -> Vec<Vec<&str>> {
    let mut piles: Vec<Vec<&str>> = vec![Vec::new(); RANKS_IN_DECK];
    for card in cards {
        let rank = cardgame.find_rank(card);
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

type LineVec = Vec<String>;
type LineMap = HashMap<String, u32>;

fn read_file_into_lines(filename: &str) -> Result<(LineVec, LineMap), Box<dyn std::error::Error>> {
    let mut lines_vec: LineVec = Vec::new();
    let mut map: LineMap = HashMap::new();

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines_vec: LineVec;
    let map: LineMap;

    (lines_vec, map) = read_file_into_lines("input")?;

    println!("PART1: {}", sum_all(rank_cards(&lines_vec, Part1), &map));
    println!("PART2: {}", sum_all(rank_cards(&lines_vec, Part2), &map));
    Ok(())
}

fn rank_cards<T: Cardgame>(lines_vec: &[String], cardgame: T) -> Vec<Vec<&str>> {
    let vec_of_str: Vec<&str> = lines_vec.iter().map(|s| s.as_str()).collect();
    let mut piles = sort_rank_piles(vec_of_str, cardgame);

    for p in &mut piles {
        p.sort_by(|a, b| T::cmp_cards(a, b));
    }
    piles
}

fn sum_all(piles: Vec<Vec<&str>>, map: &LineMap) -> u32 {
    let mut sum = 0;
    let mut i: u32 = 1;
    for p in piles {
        for card in p {
            let x = *map.get(card).unwrap() * i;
            //dbg!("{} = {} * {}", x, x / i, i);
            sum += x;
            i += 1;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_find_rank {
        ($cardgame:expr, $cards:expr, $rank:expr) => {
            assert_eq!($cardgame.find_rank($cards), $rank);
        };
    }

    #[test]
    fn test_example_part2() {
        let lines_vec: LineVec;
        let map: LineMap;

        (lines_vec, map) = read_file_into_lines("example.input").unwrap();
        assert_eq!(sum_all(rank_cards(&lines_vec, Part2), &map), 5905);
    }

    #[test]
    fn test_compare_types_part2_joker() {
        test_find_rank!(Part2, "JJJJJ", Rank::Five);
        test_find_rank!(Part2, "JJJTA", Rank::Four);
        test_find_rank!(Part2, "JJJ98", Rank::Four);
        test_find_rank!(Part2, "TJJ98", Rank::Three);
        test_find_rank!(Part2, "2J332", Rank::FullHouse);
        test_find_rank!(Part2, "A23J4", Rank::OnePair);
    }

    #[test]
    fn test_compare_types_part2() {
        test_find_rank!(Part2, "AAAAA", Rank::Five);
        test_find_rank!(Part2, "AA8AA", Rank::Four);
        test_find_rank!(Part2, "TTJ98", Rank::Three);
        test_find_rank!(Part2, "2J332", Rank::FullHouse);
        test_find_rank!(Part2, "2J432", Rank::Three);
        test_find_rank!(Part2, "A23J4", Rank::OnePair);
        test_find_rank!(Part2, "23456", Rank::HighCard);
        test_find_rank!(Part2, "33J32", Rank::Four);
        test_find_rank!(Part2, "2AAJA", Rank::Four);
        test_find_rank!(Part2, "778J8", Rank::FullHouse);
        test_find_rank!(Part2, "777J8", Rank::Four);
        test_find_rank!(Part2, "32T3K", Rank::OnePair);
        test_find_rank!(Part2, "KTJJT", Rank::Four);
        test_find_rank!(Part2, "KK677", Rank::TwoPair);
        test_find_rank!(Part2, "T55JA", Rank::Three);
    }

    #[test]
    fn test_compare_types() {
        test_find_rank!(Part1, "AAAAA", Rank::Five);
        test_find_rank!(Part1, "AA8AA", Rank::Four);
        test_find_rank!(Part1, "TTT98", Rank::Three);
        test_find_rank!(Part1, "23332", Rank::FullHouse);
        test_find_rank!(Part1, "23432", Rank::TwoPair);
        test_find_rank!(Part1, "A23A4", Rank::OnePair);
        test_find_rank!(Part1, "23456", Rank::HighCard);
        test_find_rank!(Part1, "33332", Rank::Four);
        test_find_rank!(Part1, "2AAAA", Rank::Four);
        test_find_rank!(Part1, "77888", Rank::FullHouse);
        test_find_rank!(Part1, "77788", Rank::FullHouse);
        test_find_rank!(Part1, "32T3K", Rank::OnePair);
        test_find_rank!(Part1, "KTTTT", Rank::Four);
        test_find_rank!(Part1, "KK677", Rank::TwoPair);
        test_find_rank!(Part1, "T55J5", Rank::Three);
    }

    #[test]
    fn test_example_part1() {
        let lines_vec: LineVec;
        let map: LineMap;

        (lines_vec, map) = read_file_into_lines("example.input").unwrap();
        assert_eq!(sum_all(rank_cards(&lines_vec, Part1), &map), 6440);
    }

    #[test]
    fn test_compare_basic() {
        assert_eq!(Part1::compare_card('A', 'A'), std::cmp::Ordering::Equal);
        assert_eq!(Part1::compare_card('A', 'T'), std::cmp::Ordering::Greater);
        assert_eq!(Part1::compare_card('2', '3'), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_check_piles() {
        let mut piles: Vec<Vec<&str>> = vec![Vec::new(); RANKS_IN_DECK];
        let game = Part1;

        let cards: [&str; 16] = [
            "AAAAA", "AA8AA", "TTT98", "23332", "23432", "A23A4", "23456", "33332", "2AAAA",
            "77888", "77788", "32T3K", "KTJJT", "KK677", "T55J5", "T55J5",
        ];

        for card in cards {
            let rank = game.find_rank(card);
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

        let game = Part1;
        for card in cards {
            let rank = game.find_rank(card);
            let rn = rank_to_usize(rank);
            piles[rn].push(card);
        }

        for p in &mut piles {
            p.sort_by(|a, b| Part1::cmp_cards(a, b));
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
        let lines_vec: LineVec;
        let map: LineMap;

        (lines_vec, map) = read_file_into_lines("example.input").unwrap();

        assert_eq!(sum_all(rank_cards(&lines_vec, Part1), &map), 6440);
    }
}
