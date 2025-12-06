use itertools::Itertools;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let ops = lines
        .next_back()
        .unwrap()
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect_vec();
    let nums = lines
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let x_len = nums[0].len();
    let y_len = nums.len();

    let ans = (0..x_len)
        .map(|col| match ops[col] {
            '*' => (0..y_len)
                .map(|row| nums[row][col])
                .reduce(|acc, e| acc * e)
                .unwrap(),
            '+' => (0..y_len)
                .map(|row| nums[row][col])
                .reduce(|acc, e| acc + e)
                .unwrap(),
            _ => panic!(),
        })
        .sum::<u64>();

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let ops = lines
        .next_back()
        .unwrap()
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect_vec();

    let nums = lines
        .map(|line| line.chars().pad_using(15, |_| ' ').collect_vec())
        .collect_vec();

    // println!("{nums:?}");

    // println!(
    //     "ops.len(): {}, nums.len(): {}, nums[0].len(): {}",
    //     ops.len(),
    //     nums.len(),
    //     nums[0].len()
    // );

    let cols = nums[0].len();
    let rows = nums.len();

    let transpose = (0..cols)
        .map(|col| {
            (0..rows).map(|row| nums[row][col]).fold(0u64, |acc, x| {
                if x == ' ' {
                    acc
                } else {
                    acc * 10 + x.to_digit(10).unwrap() as u64
                }
            })
        })
        .collect_vec();

    // println!("{transpose:?}");

    let transpose_nums = transpose.split(|&n| n == 0).collect_vec();

    let ans = (0..transpose_nums.len())
        .map(|col| match ops[col] {
            '*' => transpose_nums[col].iter().product::<u64>(),
            '+' => transpose_nums[col].iter().sum::<u64>(),
            _ => panic!(),
        })
        .sum::<u64>();

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
