use std::fmt;

use itertools::Itertools;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    // let mut elements = input.split("\n\n");
    let (presents, regions) = input.rsplit_once("\n\n").unwrap();

    let presents = presents
        .split("\n\n")
        .map(|block| {
            let mut lines = block.lines();
            let idx = lines
                .next()
                .unwrap()
                .strip_suffix(':')
                .unwrap()
                .parse::<u8>()
                .unwrap();
            let line1 = lines
                .next()
                .unwrap()
                .chars()
                .fold(0, |acc, c| if c == '#' { (acc << 1) + 1 } else { acc << 1 });
            let line2 = lines
                .next()
                .unwrap()
                .chars()
                .fold(0, |acc, c| if c == '#' { (acc << 1) + 1 } else { acc << 1 });
            let line3 = lines
                .next()
                .unwrap()
                .chars()
                .fold(0, |acc, c| if c == '#' { (acc << 1) + 1 } else { acc << 1 });
            Present {
                idx,
                line1,
                line2,
                line3,
            }
        })
        .collect_array::<6>()
        .unwrap();

    presents.iter().for_each(|present| println!("{present}"));

    let regions = regions
        .lines()
        .map(|line| {
            let (dims, presents) = line.split_once(": ").unwrap();
            let (x, y) = dims.split_once("x").unwrap();
            let (x, y) = (x.parse::<u8>().unwrap(), y.parse::<u8>().unwrap());
            let presents = presents
                .split_whitespace()
                .map(|num| num.parse::<u8>().unwrap())
                .collect_array::<6>()
                .unwrap();
            Region { x, y, presents }
        })
        .collect_vec();

    regions.iter().for_each(|region| println!("{region}"));

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

struct Present {
    idx: u8,
    line1: u8,
    line2: u8,
    line3: u8,
}

impl fmt::Display for Present {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}:", self.idx)?;
        [self.line1, self.line2, self.line3]
            .into_iter()
            .for_each(|line| {
                for bit in 0..3 {
                    if ((line >> bit) & 1) == 1 {
                        _ = write!(f, "#");
                    } else {
                        _ = write!(f, ".");
                    }
                }
                _ = writeln!(f);
            });

        Ok(())
    }
}

struct Region {
    x: u8,
    y: u8,
    presents: [u8; 6],
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}:", self.x, self.y)?;

        self.presents
            .iter()
            .for_each(|present| _ = write!(f, " {present}"));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
