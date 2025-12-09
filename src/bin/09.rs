use std::{collections::BTreeSet, ops::RangeInclusive};

use hashbrown::HashMap;
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
    let red = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(',').unwrap();
            (left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap())
        })
        .collect_vec();

    let mut horizontal_segments = Vec::<(u64, (u64, u64))>::new();
    let mut vertical_segments = Vec::<(u64, (u64, u64))>::new();

    red.iter().circular_tuple_windows().for_each(|(a, b)| {
        if a.0 == b.0 {
            horizontal_segments.push((a.0, (a.1.min(b.1), (a.1.max(b.1)))));
        } else {
            vertical_segments.push((a.1, (a.0.min(b.0), (a.0.max(b.0)))));
        }
    });

    println!("{horizontal_segments:?}");

    let (y_min, y_max) = red.iter().map(|(_, y)| *y).minmax().into_option().unwrap();
    let (x_min, x_max) = red.iter().map(|(x, _)| *x).minmax().into_option().unwrap();

    let y_ranges = (y_min..=y_max).map(|y| {
        let vert = vertical_segments
            .iter()
            .filter(|(_, (y_low, y_high))| (y_low..=y_high).contains(&&y))
            .sorted_by_key(|(x, _)| x)
            .collect_vec();

        let mut total_tiles = 0;
        let mut count_next = false;

        vert.iter().combinations(2).map(|v| {
            let left = v[0];
            let right = v[1];
        });

        for segment in &vert {
            if (min_x..=max_x).contains(&segment.0) {
                if count_next {
                    let segment_min = segment.1.0;
                    let segment_max = segment.1.1;
                    total_tiles += segment_max.max(max_x) - segment_min.min(min_x);
                    count_next = false;
                }
            }
        }
    });

    let ans = red
        .iter()
        .combinations(2)
        .map(|v| {
            let a = v[0];
            let b = v[1];

            let min_x = a.0.min(b.0);
            let min_y = a.1.min(b.1);
            let max_x = a.0.max(b.0);
            let max_y = a.1.max(b.1);

            // number of tiles inside polygon for each line
            (min_y..=max_y).for_each(|y| {
                let left_side = vert.iter().filter(|(x, _)| *x >= min_x).count();
                let right_side = vert.iter().filter(|(x, _)| *x >= max_x).count();
            });

            1
        })
        .max()
        .unwrap();

    println!("{ans}");

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
        assert_eq!(result, Some(24));
    }
}
