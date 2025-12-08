use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let ans = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .split(",")
                .map(|range| {
                    // println!("{range}");
                    let (left, right) = range.split_once("-").unwrap();
                    let (left_num, right_num) =
                        (left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap());

                    let num_range = (left_num)..=(right_num);
                    // println!("num_range: {num_range:?}");

                    let (left_one, left_two) = left.split_at(left.len() / 2);
                    let (right_one, right_two) = right.split_at(right.len() / 2);

                    let lower_bound = if left_one.is_empty() {
                        left_two.parse::<u64>().unwrap()
                    } else {
                        left_one.parse::<u64>().unwrap()
                    };

                    let upper_bound = [left_one, left_two, right_one, right_two]
                        .into_iter()
                        .filter(|s| !s.is_empty())
                        .map(|s| s.parse::<u64>().unwrap())
                        .max()
                        .unwrap();

                    // println!("bounds: {lower_bound}{lower_bound}-{upper_bound}{upper_bound}");

                    (lower_bound..=upper_bound)
                        .filter_map(|num| {
                            let s = num.to_string();
                            let concat = s.repeat(2);
                            let new_num = concat.parse::<u64>().unwrap();
                            // println!("{new_num}");
                            if num_range.contains(&new_num) {
                                Some(new_num)
                            } else {
                                None
                            }
                        })
                        .sum::<u64>()
                })
                .sum::<u64>()
        })
        .sum::<u64>();

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ans = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .split(",")
                .map(|range| {
                    // println!("\n{range}:");
                    let (left, right) = range.split_once("-").unwrap();
                    let num_range =
                        (left.parse::<u64>().unwrap())..=(right.parse::<u64>().unwrap());
                    num_range
                        .filter(|num| {
                            let s = num.to_string();
                            let length = s.len();
                            let c = s.as_bytes();
                            // println!("{num}");
                            //
                            if length == 1 {
                                return false;
                            } else if c.iter().all_equal() {
                                // println!("skip because all were same {c:?}");
                                return true;
                            }

                            (2..length).any(|split_len| {
                                if length % split_len == 0 {
                                    let split_count = length / split_len;
                                    // println!("split_len: {split_len}, split_count: {split_count}",);

                                    (0..(split_count))
                                        // .inspect(|n| println!("{n}"))
                                        .map(|split_count| {
                                            // println!("split_count inside: {split_count}");
                                            let start = split_count * split_len;
                                            let end = start + split_len;
                                            &c[start..end]
                                        })
                                        .all_equal()
                                } else {
                                    false
                                }
                            })
                        })
                        // .inspect(|num| print!("{num},"))
                        .sum::<u64>()
                })
                .sum::<u64>()
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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
