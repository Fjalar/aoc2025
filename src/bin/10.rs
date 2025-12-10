use std::fmt;

use itertools::Itertools;
use ndarray::{ArcArray2, prelude::*};
use ndarray_linalg::{self, Solve};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    let subprobs = input.lines().map(SubProbPart1::from).collect_vec();

    // subprobs.iter().for_each(|s| println!("{s}"));

    let ans = subprobs
        .iter()
        .map(|sub| {
            let mut max_depth = 1;
            // print!("Testing depth...");
            loop {
                // print!(" {max_depth}...");
                if p1recurse(sub.lights, &sub.buttons, 0, max_depth) {
                    break;
                }
                max_depth += 1;
            }
            // println!();
            max_depth
        })
        .sum::<u16>() as u64;

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let subprobs = input.lines().map(SubProbPart2::from).collect_vec();

    let ans = subprobs
        .iter()
        .map(|sub| {
            println!("{}", sub.a);
            println!("{}", sub.b);

            let x = sub.a.solve_into(sub.b.clone()).unwrap();

            println!("{x}");

            0
        })
        .sum::<u16>() as u64;

    Some(ans)
}

fn p1recurse(lights: u16, buttons: &[u16], depth: u16, max_depth: u16) -> bool {
    // print!("{:016b} at depth {depth}: ", lights.reverse_bits());
    if lights == u16::MAX {
        true
    } else if depth >= max_depth {
        // println!(" false");
        // println!("{:b} ", lights.reverse_bits());
        false
    } else {
        buttons.iter().enumerate().any(|(idx, button)| {
            // println!("pressed button {}", idx + 1);
            p1recurse(lights ^ button, buttons, depth + 1, max_depth)
        })
    }
}

#[derive(Debug)]
struct SubProbPart1 {
    lights: u16,
    buttons: Vec<u16>,
    length: u8,
}

impl fmt::Display for SubProbPart1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..self.length {
            if (self.lights >> i) & 1 == 1 {
                write!(f, ".")?;
            } else {
                write!(f, "#")?;
            }
        }
        write!(f, "]")?;

        for button in &self.buttons {
            write!(f, " (")?;
            write!(
                f,
                "{:0width$b}",
                (button.reverse_bits()) >> (16 - self.length),
                width = self.length as usize
            )?;
            write!(f, ")")?;
        }

        Ok(())
    }
}

impl From<&str> for SubProbPart1 {
    fn from(value: &str) -> Self {
        let (lights, value) = value.split_once(" ").unwrap();
        let (buttons, _) = value.rsplit_once(" ").unwrap();
        let len = lights.len() - 2;
        SubProbPart1 {
            lights: lights
                .strip_prefix('[')
                .unwrap()
                .strip_suffix(']')
                .unwrap()
                .chars()
                .enumerate()
                .fold(
                    u16::MAX,
                    |acc, (idx, x)| {
                        if x == '#' { acc ^ (1 << idx) } else { acc }
                    },
                ),
            buttons: buttons
                .split_whitespace()
                .map(|button| {
                    button
                        .strip_prefix('(')
                        .unwrap()
                        .strip_suffix(')')
                        .unwrap()
                        .split(",")
                        .map(|s| s.parse::<u16>().unwrap())
                        .fold(0u16, |acc, x| acc | (1u16 << x))
                })
                .collect_vec(),
            length: len as u8,
        }
    }
}

struct SubProbPart2 {
    a: Array2<f64>,
    b: Array1<f64>,
}

impl From<&str> for SubProbPart2 {
    fn from(value: &str) -> Self {
        let (_, value) = value.split_once(" ").unwrap();
        let (buttons, joltage) = value.rsplit_once(" ").unwrap();

        let joltage = joltage
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .map(|c| c.parse::<f64>().unwrap())
            .collect_vec();

        let buttons = buttons
            .split_whitespace()
            .map(|button| {
                button
                    .strip_prefix('(')
                    .unwrap()
                    .strip_suffix(')')
                    .unwrap()
                    .split(",")
                    .map(|s| s.parse::<u16>().unwrap())
                    .collect_vec()
            })
            .collect_vec();

        let mut constraints = Vec::new();

        (0..joltage.len()).for_each(|lamp_idx| {
            constraints.push(
                buttons
                    .iter()
                    .map(|button| {
                        if button.contains(&(lamp_idx as u16)) {
                            1f64
                        } else {
                            0f64
                        }
                    })
                    .collect_vec(),
            );
        });

        let mut a = Array2::zeros((constraints.len(), constraints.first().unwrap().len()));

        for row in 0..constraints.len() {
            for col in 0..constraints[row].len() {
                a[[row, col]] = constraints[row][col];
            }
        }

        let b = Array1::from_vec(joltage);

        SubProbPart2 { a, b }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
