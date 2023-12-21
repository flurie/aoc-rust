advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let soln = input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));
            let first_num = digits.next().unwrap();
            let second_num = digits.next_back().unwrap_or(first_num);
            10 * first_num + second_num
        })
        .sum();
    Some(soln)
}

pub fn part_two(input: &str) -> Option<u32> {
    let soln = input
        .lines()
        .map(|line| {
            // need to pad the window with unmatchable chars
            let chars = [line.as_bytes(), &[b'z', b'z', b'z', b'z']].concat();
            let nums: Vec<_> = chars
                .windows(5)
                .map(|w| match w {
                    &[b'o', b'n', b'e', _, _] | &[b'1', _, _, _, _] => Some(&1),
                    &[b't', b'w', b'o', _, _] | &[b'2', _, _, _, _] => Some(&2),
                    &[b't', b'h', b'r', b'e', b'e'] | &[b'3', _, _, _, _] => Some(&3),
                    &[b'f', b'o', b'u', b'r', _] | &[b'4', _, _, _, _] => Some(&4),
                    &[b'f', b'i', b'v', b'e', _] | &[b'5', _, _, _, _] => Some(&5),
                    &[b's', b'i', b'x', _, _] | &[b'6', _, _, _, _] => Some(&6),
                    &[b's', b'e', b'v', b'e', b'n'] | &[b'7', _, _, _, _] => Some(&7),
                    &[b'e', b'i', b'g', b'h', b't'] | &[b'8', _, _, _, _] => Some(&8),
                    &[b'n', b'i', b'n', b'e', _] | &[b'9', _, _, _, _] => Some(&9),
                    _ => None,
                })
                .filter(|x| x.is_some())
                .collect();
            let first_num = nums.first().unwrap().unwrap();
            let second_num = nums.last().unwrap().unwrap_or(first_num);
            10 * first_num + second_num
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
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
