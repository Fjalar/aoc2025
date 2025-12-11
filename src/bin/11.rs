use itertools::Itertools;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let mut all_labels = Vec::new();

    let devices = input
        .lines()
        .map(|line| {
            let (label, connections) = line.split_once(':').unwrap();
            let label = label.to_owned();
            let connections = connections
                .split_whitespace()
                .map(|connection| connection.trim().to_owned())
                .collect_vec();

            all_labels.push(label.clone());

            Device { label, connections }
        })
        .collect_vec();

    let start = devices
        .iter()
        .filter(|device| device.label.eq("you"))
        .exactly_one()
        .unwrap();

    let visited = Vec::<String>::new();

    let ans = start.visit_recursively(&visited, &devices);

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[derive(Debug)]
struct Device {
    pub label: String,
    pub connections: Vec<String>,
}

impl Device {
    fn visit_recursively(&self, visited: &[String], all: &[Device]) -> u64 {
        let mut new_visited = visited.to_vec();
        new_visited.push(self.label.clone());
        self.connections
            .iter()
            .filter(|next| !new_visited.contains(*next))
            .map(|next| {
                if next == "out" {
                    1
                } else {
                    all.iter()
                        .filter(|device| device.label.eq(next))
                        .exactly_one()
                        .unwrap()
                        .visit_recursively(&new_visited, all)
                }
            })
            .sum::<u64>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
