use std::{
    collections::{BTreeMap, BTreeSet},
    env::current_exe,
};

use itertools::Itertools;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let s_pos = grid[0].iter().find_position(|&c| *c == 'S').unwrap();

    let mut current_tachyons = BTreeSet::<usize>::new();

    current_tachyons.insert(s_pos.0);

    let mut sum_splits = 0;

    grid.iter().for_each(|line| {
        line.iter().enumerate().for_each(|(idx, &char)| {
            if current_tachyons.contains(&idx) && char == '^' {
                // println!("before remove: {current_tachyons:?}");
                current_tachyons.remove(&idx);
                // println!("after remove: {current_tachyons:?}");
                sum_splits += 1;
                current_tachyons.insert(idx - 1);
                current_tachyons.insert(idx + 1);
                // println!("after add: {current_tachyons:?}");
            }
        });
        // line.iter().enumerate().for_each(|(idx, char)| {
        //     if *char == 'S' {
        //         print!("S");
        //     } else if current_tachyons.contains(&idx) {
        //         print!("|");
        //     } else {
        //         print!("{char}");
        //     }
        // });

        // println!();
    });

    Some(sum_splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let s_pos = grid[0].iter().find_position(|&c| *c == 'S').unwrap();

    let mut current_tachyons = BTreeMap::<usize, u64>::new();

    current_tachyons.insert(s_pos.0, 1);

    grid.iter().for_each(|line| {
        line.iter().enumerate().for_each(|(idx, &char)| {
            if current_tachyons.contains_key(&idx) && char == '^' {
                // println!("before remove: {current_tachyons:?}");
                let prev_val = current_tachyons.remove(&idx).unwrap();
                // println!("after remove: {current_tachyons:?}");
                current_tachyons
                    .entry(idx - 1)
                    .and_modify(|v| *v += prev_val)
                    .or_insert(prev_val);
                current_tachyons
                    .entry(idx + 1)
                    .and_modify(|v| *v += prev_val)
                    .or_insert(prev_val);
                // println!("after add: {current_tachyons:?}");
            }
        });
        // line.iter().enumerate().for_each(|(idx, char)| {
        //     if *char == 'S' {
        //         print!("S");
        //     } else if let Some(val) = current_tachyons.get(&idx) {
        //         print!("{val:X}");
        //     } else {
        //         print!("{char}");
        //     }
        // });

        // println!();
    });

    let timelines = current_tachyons.values().sum::<u64>();

    Some(timelines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
