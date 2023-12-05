use std::{collections::HashMap, u32};

advent_of_code::solution!(2);

struct Game {
    number: u32,
    plays: Vec<Play>,
}

impl Game {
    fn from_string(game: &str) -> Game {
        let (game_num_raw, plays_raw) = match game.split(": ").collect::<Vec<_>>()[..] {
            [x, y] => (x, y),
            _ => ("0", "none"),
        };
        let (_, number) = match game_num_raw.split(" ").collect::<Vec<_>>()[..] {
            [x, y] => (x, str::parse(y).unwrap_or(0)),
            _ => ("Game", 0),
        };
        let plays: Vec<Play> = plays_raw
            .split("; ")
            .map(|p| Play::from_string(p))
            .collect();
        Game { number, plays }
    }

    fn possible(&self, bag: Play) -> bool {
        self.plays.iter().all(|play| play.possible(bag))
    }

    fn max_each(&self) -> Play {
        Play {
            red: self.plays.iter().map(|p| p.red).max().unwrap_or(0),
            blue: self.plays.iter().map(|p| p.blue).max().unwrap_or(0),
            green: self.plays.iter().map(|p| p.green).max().unwrap_or(0),
        }
    }
}

#[derive(Clone, Copy)]
struct Play {
    green: u32,
    red: u32,
    blue: u32,
}

impl Play {
    fn from_string(play: &str) -> Play {
        let mut play_map = HashMap::new();
        play.split(", ").collect::<Vec<_>>().iter().for_each(|p| {
            let c = parse_num_color(p);
            play_map.insert(c.0, c.1);
        });
        Play {
            red: play_map.get("red").copied().unwrap_or(0),
            green: play_map.get("green").copied().unwrap_or(0),
            blue: play_map.get("blue").copied().unwrap_or(0),
        }
    }

    fn possible(&self, bag: Play) -> bool {
        self.red <= bag.red && self.blue <= bag.blue && self.green <= bag.green
    }

    fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

fn parse_num_color(color: &str) -> (&str, u32) {
    match color.split(" ").collect::<Vec<_>>()[..] {
        [num, color] => (color, str::parse(num).unwrap_or(0)),
        _ => ("white", 0),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let bag = Play {
        red: 12,
        green: 13,
        blue: 14,
    };
    let soln = input
        .lines()
        .map(|game_string| {
            let game = Game::from_string(game_string);
            if game.possible(bag) {
                game.number
            } else {
                0
            }
        })
        .sum();
    Some(soln)
}

pub fn part_two(input: &str) -> Option<u32> {
    let soln = input
        .lines()
        .map(|game_string| {
            let game = Game::from_string(game_string);
            game.max_each().power()
        })
        .sum();
    Some(soln)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
