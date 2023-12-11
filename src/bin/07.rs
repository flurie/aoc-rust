use std::{collections::HashMap, str::FromStr};

advent_of_code::solution!(7);

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct RankParseError {}

impl FromStr for Rank {
    type Err = RankParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut m: HashMap<u8, usize> = HashMap::new();
        let mut jokers = 0;
        for b in s.as_bytes() {
            if b == &b'j' {
                jokers += 1;
            } else {
                *m.entry(*b).or_default() += 1;
            }
        }
        let mut counts = m.values().clone().collect::<Vec<_>>();
        counts.sort_by(|a, b| b.cmp(&a));
        match (counts.as_slice(), jokers) {
            (&[5], _) | (&[4], 1) | (&[3], 2) | (&[2], 3) | (&[1], 4) | (_, 5) => {
                Ok(Self::FiveOfAKind)
            }
            (&[4, ..], _) | (&[3, ..], 1) | (&[2, ..], 2) | (&[1, ..], 3) | (_, 4) => {
                Ok(Self::FourOfAKind)
            }
            (&[3, 2], _) | (&[2, 2], 1) => Ok(Self::FullHouse),
            (&[3, ..], _) | (&[2, ..], 1) | (&[1, ..], 2) | (_, 3) => Ok(Self::ThreeOfAKind),
            (&[2, 2, ..], _) => Ok(Self::TwoPair),
            (&[2, ..], _) | (&[1, ..], 1) => Ok(Self::OnePair),
            (&[1, ..], _) => Ok(Self::HighCard),
            _ => Err(RankParseError {}),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    rank: Rank,
    cards: Vec<u8>,
    bid: u32,
}

struct HandParseError {}

impl FromStr for Hand {
    type Err = HandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards_s, bid) = s.split_once(" ").unwrap();
        let cards = cards_s
            .as_bytes()
            .iter()
            .map(|b| match b {
                b'j' => 0,
                b'1' => 1,
                b'2' => 2,
                b'3' => 3,
                b'4' => 4,
                b'5' => 5,
                b'6' => 6,
                b'7' => 7,
                b'8' => 8,
                b'9' => 9,
                b'T' => 10,
                b'J' => 11,
                b'Q' => 12,
                b'K' => 13,
                b'A' => 14,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        Ok(Self {
            rank: Rank::from_str(cards_s).ok().unwrap(),
            cards,
            bid: bid.parse().unwrap(),
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = input
        .lines()
        .map(|l| Hand::from_str(l).ok().unwrap())
        .collect::<Vec<Hand>>();
    hands.sort();
    Some(
        hands
            .iter()
            .enumerate()
            .map(|(i, h)| (i + 1) as u32 * h.bid)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands = input
        .lines()
        .map(|l| Hand::from_str(&l.replace("J", "j")).ok().unwrap())
        .collect::<Vec<Hand>>();
    hands.sort();
    Some(
        hands
            .iter()
            .enumerate()
            .map(|(i, h)| (i + 1) as u32 * h.bid)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
