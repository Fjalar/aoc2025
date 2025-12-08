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
        .flat_map(|line| {
            line.trim().split(",").map(|range| {
                // println!("\n\n{range}:\n");
                let (left, right) = range.split_once("-").unwrap();

                let (left_num, right_num) =
                    (left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap());

                let num_range = left_num..=right_num;

                let left_digits = left.len();
                let right_digits = right.len();

                (2..=(left_digits).max(right_digits))
                    .flat_map(|parts| {
                        // println!("divided into {parts} parts");

                        let num_range = num_range.clone();

                        let part_length = if left_digits % parts == 0 {
                            left_digits / parts
                        } else if right_digits % parts == 0 {
                            right_digits / parts
                        } else {
                            0
                        };

                        let min;
                        let max;

                        if part_length == 0 {
                            min = 0;
                            max = 0;
                        } else {
                            let min_left = left
                                .chars()
                                .chunks(part_length)
                                .into_iter()
                                .map(|chunk| String::from_iter(chunk).parse::<u64>().unwrap())
                                .min()
                                .unwrap();

                            let min_right = right
                                .chars()
                                .chunks(part_length)
                                .into_iter()
                                .map(|chunk| String::from_iter(chunk).parse::<u64>().unwrap())
                                .min()
                                .unwrap();

                            min = min_left.min(min_right);
                            max = '9'.to_string().repeat(part_length).parse::<u64>().unwrap();
                        }

                        (min..=max).filter_map(move |part: u64| {
                            let s = part.to_string();
                            let concat = s.trim().repeat(parts);
                            // println!("{concat}");
                            let new_num = concat.parse::<u64>().unwrap();
                            if num_range.contains(&new_num) {
                                // print!("{new_num},");
                                Some(new_num)
                            } else {
                                None
                            }
                        })
                    })
                    .sorted()
                    .dedup()
                    .sum::<u64>()
            })
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
