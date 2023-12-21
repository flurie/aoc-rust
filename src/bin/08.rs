use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut paths = HashMap::new();
    let (route, path_str) = input.split_once("\n\n").unwrap();
    path_str.lines().for_each(|l| {
        let source = l.get(0..3).unwrap();
        let ldest = l.get(7..10).unwrap();
        let rdest = l.get(12..15).unwrap();
        paths.insert((source, &'L'), ldest);
        paths.insert((source, &'R'), rdest);
    });
    let mut loc = "AAA";
    let mut steps = 0;
    const DEST: &str = "ZZZ";
    while loc != DEST {
        route.chars().for_each(|c| {
            loc = paths.get(&(loc, &c)).unwrap();
            steps += 1;
        });
    }
    Some(steps)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut paths = HashMap::new();
    let (route, path_str) = input.split_once("\n\n").unwrap();
    path_str.lines().for_each(|l| {
        let source = l.get(0..3).unwrap();
        let ldest = l.get(7..10).unwrap();
        let rdest = l.get(12..15).unwrap();
        paths.insert((source, &'L'), ldest);
        paths.insert((source, &'R'), rdest);
    });
    let mut shortpaths = HashMap::new();
    let mut loc;
    for key in paths.keys() {
        loc = key.0;
        route.chars().for_each(|c| {
            loc = paths.get(&(loc, &c)).unwrap();
        });
        shortpaths.insert(key.0, loc);
    }
    let locs = paths
        .keys()
        .filter(|s| s.0.ends_with("A") && s.1 == &'L')
        .map(|s| s.0)
        .collect::<Vec<_>>();
    // turns out this works because these are all primes (including the path)
    // but if they were not it would be necessary to find the set of GCDs
    let soln = locs.iter().map(|l| {
        let mut iter = 0;
        let mut loc = l;
        while !loc.ends_with("Z") {
            loc = shortpaths.get(loc).unwrap();
            iter += 1;
        }
        iter as u64
    });
    // need to multiply by the path as well since we made a shortcode lookup
    Some(soln.product::<u64>() * route.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
