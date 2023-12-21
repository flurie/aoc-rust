advent_of_code::solution!(9);

fn diff(v: Vec<i64>) -> Vec<i64> {
    v.windows(2)
        .map(|w| match w {
            &[w0, w1] => w1 - w0,
            _ => 0,
        })
        .collect()
}

// don't need to do the final diff, can just diff until constant
fn is_constant(v: Vec<i64>) -> bool {
    match v.first() {
        None => false,
        Some(f) => v.iter().all(|i| i == f),
    }
}

// val is just sum of all the last vals we find
fn find_next(v: Vec<i64>) -> i64 {
    let mut differential: Vec<i64>;
    let mut lasts = vec![*v.last().unwrap()];
    differential = v;
    while !is_constant(differential.clone()) {
        let diff_tmp = diff(differential);
        let last_val = diff_tmp.clone().last().unwrap().clone();
        lasts.push(last_val);
        differential = diff_tmp.clone();
    }
    lasts.iter().sum()
}

pub fn part_one(input: &str) -> Option<i64> {
    let soln: i64 = input
        .lines()
        .map(|l| {
            let seq = l
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            find_next(seq)
        })
        .sum();
    Some(soln)
}

pub fn part_two(input: &str) -> Option<i64> {
    let soln: i64 = input
        .lines()
        .map(|l| {
            let mut seq = l
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            // literally just reverse the sequences and do same thing
            seq.reverse();
            find_next(seq)
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
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
