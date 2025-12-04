use itertools::Itertools;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect_vec())
        .collect_vec();

    // for row in &grid {
    //     for c in row {
    //         if *c {
    //             print!("@");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    let x_len = grid[0].len();
    let y_len = grid.len();

    // println!("lengths: {x_len}, {y_len}");

    Some(
        (0..y_len)
            .cartesian_product(0..x_len)
            .filter(|&(x, y)| {
                if grid[y][x] {
                    let lower_x;
                    let lower_y;
                    let higher_x;
                    let higher_y;
                    // println!("{x},{y} was @");
                    if let Some(x_sub_one) = x.checked_sub(1) {
                        lower_x = x_sub_one;
                    } else {
                        lower_x = x;
                    }
                    if let Some(y_sub_one) = y.checked_sub(1) {
                        lower_y = y_sub_one;
                    } else {
                        lower_y = y;
                    }

                    if let Some(x_add_one) = x.checked_add(1)
                        && x_add_one < x_len
                    {
                        higher_x = x_add_one;
                    } else {
                        higher_x = x;
                    }
                    if let Some(y_add_one) = y.checked_add(1)
                        && y_add_one < y_len
                    {
                        higher_y = y_add_one;
                    } else {
                        higher_y = y;
                    }

                    // println!("bounds: {lower_x}..{higher_x}, {lower_y}..{higher_y}");

                    let at_count = (lower_x..=higher_x)
                        .cartesian_product(lower_y..=higher_y)
                        .filter(|&(x, y)| grid[y][x])
                        .count();
                    // println!("{at_count}");

                    at_count < 5
                } else {
                    false
                }
            })
            // .inspect(|v| println!("{v:?}"))
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect_vec())
        .collect_vec();

    let x_len = grid[0].len();
    let y_len = grid.len();

    let mut total_removed = 0;

    loop {
        let to_remove = (0..y_len)
            .cartesian_product(0..x_len)
            .filter(|&(x, y)| {
                if grid[y][x] {
                    let lower_x;
                    let lower_y;
                    let higher_x;
                    let higher_y;
                    // println!("{x},{y} was @");
                    if let Some(x_sub_one) = x.checked_sub(1) {
                        lower_x = x_sub_one;
                    } else {
                        lower_x = x;
                    }
                    if let Some(y_sub_one) = y.checked_sub(1) {
                        lower_y = y_sub_one;
                    } else {
                        lower_y = y;
                    }
                    if let Some(x_add_one) = x.checked_add(1)
                        && x_add_one < x_len
                    {
                        higher_x = x_add_one;
                    } else {
                        higher_x = x;
                    }
                    if let Some(y_add_one) = y.checked_add(1)
                        && y_add_one < y_len
                    {
                        higher_y = y_add_one;
                    } else {
                        higher_y = y;
                    }

                    let at_count = (lower_x..=higher_x)
                        .cartesian_product(lower_y..=higher_y)
                        .filter(|&(x, y)| grid[y][x])
                        .count();

                    at_count < 5
                } else {
                    false
                }
            })
            .collect_vec();

        if to_remove.is_empty() {
            break;
        }

        for coord in to_remove {
            total_removed += 1;
            grid[coord.1][coord.0] = false;
        }
    }

    Some(total_removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
