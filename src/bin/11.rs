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

    let ans = start.visit_recursively_p1(&visited, &devices);

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
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
        .filter(|device| device.label.eq("svr"))
        .exactly_one()
        .unwrap();

    let visited = Vec::<String>::new();

    let ans = start.visit_recursively_p2(&visited, &devices, false, false);

    Some(ans)
}

#[derive(Debug)]
struct Device {
    pub label: String,
    pub connections: Vec<String>,
}

static mut TOTAL_VISITED: u64 = 0;

impl Device {
    fn visit_recursively_p1(&self, visited: &[String], all: &[Device]) -> u64 {
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
                        .visit_recursively_p1(&new_visited, all)
                }
            })
            .sum::<u64>()
    }

    fn visit_recursively_p2(
        &self,
        visited: &[String],
        all: &[Device],
        visited_fft: bool,
        visited_dac: bool,
    ) -> u64 {
        // println!("{}", self.label);
        // print!("\t");'
        unsafe {
            TOTAL_VISITED += 1;
            let val = TOTAL_VISITED;
            println!("{}", val);
        }
        let mut new_visited = visited.to_vec();
        new_visited.push(self.label.clone());
        self.connections
            .iter()
            .map(|next| {
                // print!("{next} ");
                if visited.contains(next) {
                    0
                } else if next == "out" {
                    if visited_fft && visited_dac { 1 } else { 0 }
                } else if next == "fft" && !visited_fft {
                    all.iter()
                        .filter(|device| device.label.eq(next))
                        .exactly_one()
                        .unwrap()
                        .visit_recursively_p2(&new_visited, all, true, visited_dac)
                } else if next == "dac" && !visited_dac {
                    all.iter()
                        .filter(|device| device.label.eq(next))
                        .exactly_one()
                        .unwrap()
                        .visit_recursively_p2(&new_visited, all, visited_fft, true)
                } else {
                    all.iter()
                        .filter(|device| device.label.eq(next))
                        .exactly_one()
                        .unwrap()
                        .visit_recursively_p2(&new_visited, all, visited_fft, visited_dac)
                }
            })
            .sum::<u64>()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            &fs::read_to_string("data/examples/11p1.txt").expect("could not open input file"),
        );
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            &fs::read_to_string("data/examples/11p2.txt").expect("could not open input file"),
        );
        assert_eq!(result, Some(2));
    }
}
