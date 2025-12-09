use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let points = input.lines().map(|line| {
        let (left, right) = line.split_once(',').unwrap();
        (left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap())
    });

    let ans = points
        .combinations(2)
        .map(|v| (v[0].0.abs_diff(v[1].0) + 1) * (v[0].1.abs_diff(v[1].1) + 1))
        .max()
        .unwrap();

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
