use std::collections::{HashMap, HashSet};

advent_of_code::solution!(4);

struct Game {
    winners: HashSet<u8>,
    played: HashSet<u8>,
}

impl Game {
    fn from_string(game: &str) -> Game {
        let winners_played = match game.split(": ").collect::<Vec<_>>()[..] {
            [_, w_p] => w_p,
            _ => "",
        };
        let winners_played_str = match winners_played.split(" | ").collect::<Vec<_>>()[..] {
            [w, p] => [w, p],
            _ => ["", ""],
        };
        let [winners, played] = winners_played_str.map(|s| {
            HashSet::from_iter(s.split_ascii_whitespace().map(|w| w.parse().unwrap_or(0)))
        });
        Game { winners, played }
    }

    fn score(&self) -> u32 {
        self.winners
            .intersection(&self.played)
            .count()
            .try_into()
            .unwrap_or(0)
    }

    fn power_score(&self) -> u32 {
        match self.score() {
            0 => 0,
            _ => u32::pow(2, self.score() - 1),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let soln = input
        .lines()
        .map(|l| Game::from_string(l).power_score())
        .sum();
    Some(soln)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards_for = HashMap::new();
    let soln = input
        .lines()
        .map(|l| Game::from_string(l))
        .enumerate()
        .collect::<Vec<_>>()
        .iter()
        .rev()
        .fold(0 as u32, |acc, (i, g)| {
            let it = *i as u32;
            let score = g.score();
            let new_cards = if score == 0 {
                cards_for.insert(it, 1);
                1
            } else {
                let total = (it + 1..it + score + 1).fold(1 as u32, |r_acc, e| {
                    let new_i = cards_for.get(&e).unwrap_or(&0).clone();
                    r_acc + new_i
                });
                cards_for.insert(it, total);
                total
            };
            acc + new_cards
        });
    Some(soln)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
