use hashbrown::HashSet;
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

    let ans = [("fft", "svr"), ("dac", "fft"), ("out", "dac")]
        .iter()
        .map(|&(target, start)| {
            let mut layers = Vec::new();

            layers.push(
                devices
                    .iter()
                    .filter(|device| device.connections.contains(&target.to_owned()))
                    .cloned()
                    .collect_vec(),
            );

            while !layers
                .last()
                .unwrap()
                .iter()
                .any(|device| device.label == start)
            {
                layers.push(
                    devices
                        .iter()
                        .filter(|device| {
                            device.connections.iter().any(|label| {
                                layers
                                    .last()
                                    .unwrap()
                                    .iter()
                                    .any(|last_layer_device| &last_layer_device.label == label)
                            })
                        })
                        .cloned()
                        .collect_vec(),
                );
            }

            let all_labels = layers
                .iter()
                .flat_map(|layer| layer.iter().map(|dev| dev.label.clone()))
                .collect::<HashSet<_>>();

            layers.iter_mut().flatten().for_each(|dev| {
                dev.connections
                    .retain(|label| label == target || all_labels.contains(label))
            });

            // println!("{layers:?}");

            let pruned_devices = layers.iter().flatten().collect_vec();

            let mut pos_devices = Vec::<PosDevice>::new();

            let mut start_idx: usize = 0;
            let target_idx: usize = usize::MAX;

            for idx in 0..pruned_devices.len() {
                let dev = &pruned_devices[idx];

                if dev.label == start {
                    start_idx = idx;
                }

                let connections = dev
                    .connections
                    .iter()
                    .map(|label| {
                        if label == target {
                            // usize::MAX used as magical number to represent target
                            usize::MAX
                        } else {
                            #[allow(clippy::let_and_return)]
                            let pos = pruned_devices
                                .iter()
                                .position(|find_label| &find_label.label == label)
                                .unwrap();

                            pos
                        }
                    })
                    .collect_vec();
                pos_devices.push(PosDevice { connections });
            }

            // println!("{pos_devices:?}");

            // println!("start: {start_idx}, target: {target_idx}");
            #[allow(clippy::let_and_return)]
            let start_to_target = visit_freestanding_p2(&pos_devices, start_idx, target_idx);
            // println!("{start_to_target}");

            start_to_target
        })
        .product::<u64>();

    Some(ans)
}

#[derive(Debug, Clone)]
struct Device {
    pub label: String,
    pub connections: Vec<String>,
}

#[derive(Debug)]
struct PosDevice {
    pub connections: Vec<usize>,
}

// static mut TOTAL_VISITED: u64 = 0;

fn visit_freestanding_p2(all: &[PosDevice], idx: usize, target: usize) -> u64 {
    // unsafe {
    //     TOTAL_VISITED += 1;
    //     let val = TOTAL_VISITED;
    //     if val.is_multiple_of(100_000_000) {
    //         println!("{}", val);
    //     }
    // }

    all[idx]
        .connections
        .iter()
        .map(|&next_idx| {
            // print!("{next} ");

            if next_idx == target {
                1
            } else if next_idx == usize::MAX {
                0
            } else {
                visit_freestanding_p2(all, next_idx, target)
            }
        })
        .sum::<u64>()
}

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
