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
                    let num_range =
                        (left.parse::<u64>().unwrap())..=(right.parse::<u64>().unwrap());
                    num_range
                        .filter(|num| {
                            let s = num.to_string();
                            let l = s.len();
                            if l % 2 == 0 {
                                let (left_s, right_s) = s.split_at(l / 2);
                                left_s == right_s
                            } else {
                                false
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
                            }
                            if c.iter().all_equal() {
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
