use std::ops::RangeInclusive;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (fresh, available) = input.split_once("\n\n").unwrap();
    let fresh_set = fresh
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("-").unwrap();
            left.parse::<u64>().unwrap()..=right.parse::<u64>().unwrap()
        })
        .collect::<Vec<RangeInclusive<_>>>();

    let ans = available
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .filter(|num| fresh_set.iter().any(|range| range.contains(num)))
        .count() as u64;

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (fresh, _) = input.split_once("\n\n").unwrap();
    let fresh_set = fresh
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("-").unwrap();
            (left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap())
        })
        .collect::<Vec<(u64, u64)>>();

    let mut ans = 0u64;

    let mut counted_ranges = Vec::<(u64, u64)>::new();

    (0..fresh_set.len()).for_each(|i| {
        ans += check_and_add(&mut counted_ranges, fresh_set[i]);
    });

    // let alt_ans = counted_ranges
    //     .iter()
    //     .map(|(lower, upper)| upper - lower + 1)
    //     .sum::<u64>();

    counted_ranges.sort();

    for row in counted_ranges {
        println!("{row:?}");
    }

    Some(ans)
}

fn check_and_add(counted_ranges: &mut Vec<(u64, u64)>, new: (u64, u64)) -> u64 {
    let mut lower_bound = new.0;
    let mut upper_bound = new.1;

    for prev in counted_ranges.clone() {
        let lower_contained = (lower_bound..=upper_bound).contains(&prev.0);
        let upper_contained = (lower_bound..=upper_bound).contains(&prev.1);
        if lower_contained && upper_contained {
            let lower_sum = if prev.0 > lower_bound {
                check_and_add(counted_ranges, (lower_bound, prev.0 - 1))
            } else {
                0
            };
            let upper_sum = if upper_bound > prev.1 {
                check_and_add(counted_ranges, (prev.1 + 1, upper_bound))
            } else {
                0
            };
            print!("lower: {lower_sum}, upper: {upper_sum}\t");
            return lower_sum + upper_sum;
        } else if lower_contained {
            upper_bound = prev.0 - 1;
        } else if upper_contained {
            lower_bound = prev.1 + 1;
        }
        if lower_bound > upper_bound {
            return 0;
        }
    }
    if lower_bound <= upper_bound {
        // println!(
        //     "{new:?}: found {lower_bound}-{upper_bound} resulting in {}",
        //     upper_bound - lower_bound + 1
        // );
        println!();
        counted_ranges.push((lower_bound, upper_bound));
        upper_bound - lower_bound + 1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
