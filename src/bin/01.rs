advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let answer = input.lines().map(|line| {
        let mut c = line.chars();
        let sign = if c.next().unwrap() == 'L' { -1 } else { 1 };
        let rest = c.collect::<String>();
        rest.parse::<i32>().unwrap() * sign
    });

    let mut rotation = 50;

    let mut times_at_zero = 0;

    for r in answer {
        // println!("{r}");
        rotation += r;

        while rotation < 0 {
            rotation += 100;
        }

        while rotation > 99 {
            rotation -= 100;
        }

        if rotation == 0 {
            times_at_zero += 1;
        }

        // println!("{rotation}");
    }

    Some(times_at_zero)
}

pub fn part_two(input: &str) -> Option<u64> {
    let answer = input.lines().map(|line| {
        let mut c = line.chars();
        let sign = if c.next().unwrap() == 'L' { -1 } else { 1 };
        let rest = c.collect::<String>();
        rest.parse::<i32>().unwrap() * sign
    });

    let mut position = 50;

    let mut times_at_zero = 0;

    let mut prev_was_zero = false;

    for r in answer {
        // print!("{position} {r:+}: ");

        times_at_zero += (r / 100).unsigned_abs() as u64;
        position += r % 100;

        if !prev_was_zero && position == 0 {
            // print!("(*)");
            times_at_zero += 1;
        } else if position < 0 {
            // print!("(*)");
            position += 100;
            if !prev_was_zero {
                times_at_zero += 1;
            }
        } else if position > 99 {
            // print!("(*)");
            position -= 100;
            if !prev_was_zero {
                times_at_zero += 1;
            }
        }

        // println!("{position}");

        prev_was_zero = position == 0;
    }

    Some(times_at_zero)
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
        assert_eq!(result, Some(6));
    }
}
