use std::iter::zip;
use std::str::FromStr;

advent_of_code::solution!(6);

// time is first line
struct Val(Vec<u64>);

struct ParseValError;

impl FromStr for Val {
    type Err = ParseValError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals = s
            .split_ascii_whitespace()
            .filter_map(|t| t.parse::<u64>().ok())
            .collect::<Vec<_>>();
        Ok(Val(vals))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    if let [time_s, distance_s] = input.lines().collect::<Vec<_>>()[..] {
        let time = Val::from_str(time_s).ok().unwrap().0;
        let distance = Val::from_str(distance_s).ok().unwrap().0;
        let t_d = zip(time, distance);
        let soln = t_d
            .map(|(t, d)| {
                let mut z: Vec<u64> = (0..=t).collect();
                z.reverse();
                let r = z
                    .iter()
                    .enumerate()
                    .filter_map(move |(but, dis)| {
                        let button = but.clone() as u64;
                        let run = button * dis;
                        if run > d {
                            Some(run)
                        } else {
                            None
                        }
                    })
                    .count();
                r as u64
            })
            .product();
        Some(soln)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    if let [time_s, distance_s] = input.lines().collect::<Vec<_>>()[..] {
        let time = Val::from_str(time_s).ok().unwrap().0;
        let distance = Val::from_str(distance_s).ok().unwrap().0;
        let t_d = zip(time, distance);
        let soln = t_d
            .map(|(t, d)| {
                let mut z: Vec<u64> = (0..=t).collect();
                z.reverse();
                let r = z
                    .iter()
                    .enumerate()
                    .filter_map(move |(but, dis)| {
                        let button = but.clone() as u64;
                        let run = button * dis;
                        if run > d {
                            Some(run)
                        } else {
                            None
                        }
                    })
                    .count();
                r as u64
            })
            .product();
        Some(soln)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
