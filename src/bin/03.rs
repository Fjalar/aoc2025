use itertools::Itertools;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let banks = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8));

    Some(
        banks
            .map(|bank| {
                let bank_v = bank.collect_vec();

                let mut accumulated = 0u64;

                let length = bank_v.len();

                let mut lower_idx = 0;

                for i in 0..2 {
                    // find the i:th digit

                    let upper_idx = length - 2 + i;

                    let mut max_val = 0;
                    let mut max_idx = 0;

                    (lower_idx..=upper_idx).for_each(|j| {
                        if bank_v[j] > max_val {
                            max_val = bank_v[j];
                            max_idx = j;
                        }
                    });
                    accumulated = accumulated * 10 + max_val as u64;
                    lower_idx = max_idx + 1;
                }

                accumulated
            })
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let banks = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8));

    Some(
        banks
            .map(|bank| {
                let bank_v = bank.collect_vec();

                let mut accumulated = 0u64;

                let length = bank_v.len();

                let mut lower_idx = 0;

                for i in 0..12 {
                    // find the i:th digit

                    let upper_idx = length - 12 + i;

                    let mut max_val = 0;
                    let mut max_idx = 0;

                    (lower_idx..=upper_idx).for_each(|j| {
                        if bank_v[j] > max_val {
                            max_val = bank_v[j];
                            max_idx = j;
                        }
                    });
                    accumulated = accumulated * 10 + max_val as u64;
                    lower_idx = max_idx + 1;
                }

                accumulated
            })
            .sum::<u64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
