use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let mut boxes = input
        .lines()
        .map(|line| {
            let (x, y, z) = line.split(',').collect_tuple().unwrap();
            (
                x.parse::<u64>().unwrap(),
                y.parse::<u64>().unwrap(),
                z.parse::<u64>().unwrap(),
            )
        })
        .collect::<BTreeSet<_>>();

    // example and real problem have different parameters
    let number_of_connections = if boxes.len() == 20 { 10 } else { 1000 };

    let mutual_distances = boxes
        .iter()
        .combinations(2)
        .map(|v| {
            let one = v[0];
            let two = v[1];
            (
                (*one, *two),
                one.0.abs_diff(two.0).pow(2)
                    + one.1.abs_diff(two.1).pow(2)
                    + one.2.abs_diff(two.2).pow(2),
            )
        })
        .collect::<BTreeMap<((u64, u64, u64), (u64, u64, u64)), u64>>();

    let mutual_distances = mutual_distances
        .iter()
        .sorted_by_key(|(_, dist)| **dist)
        .take(number_of_connections)
        .collect_vec();

    // mutual_distances
    //     .iter()
    //     .for_each(|entry| println!("{entry:?}"));

    let mut circuits = Vec::<BTreeSet<(u64, u64, u64)>>::new();

    'outer: for smallest_entry in mutual_distances {
        // println!("\nConnection: {smallest_entry:?}");

        let (left, right) = smallest_entry.0;

        for circuit in &mut circuits {
            if circuit.contains(left)
            // && boxes.contains(right) {
            {
                // println!("Found left box in existing circuit");
                boxes.remove(right);
                circuit.insert(*right);
                continue 'outer;
            }
            if circuit.contains(right)
            // boxes.contains(left) {
            {
                // println!("Found right box in existing circuit");
                boxes.remove(left);
                circuit.insert(*left);
                continue 'outer;
            }
        }
        // println!("Found no box in existing circuit");

        let mut new_circuit = BTreeSet::<(u64, u64, u64)>::new();
        if boxes.contains(left) {
            new_circuit.insert(*left);
            boxes.remove(left);
        }
        if boxes.contains(right) {
            new_circuit.insert(*right);
            boxes.remove(right);
        }
        if !new_circuit.is_empty() {
            circuits.push(new_circuit);
        }
    }

    for remaining_box in boxes {
        circuits.push(BTreeSet::from([remaining_box]));
    }

    // println!("--- BEFORE MERGE ---");

    // for circuit in &circuits {
    //     println!("{circuit:?}");
    // }

    // println!("len: {}", circuits.len());
    // println!("len of first: {}", circuits[0].len());

    'outer2: loop {
        let len = circuits.len();
        for i in 0..len {
            for j in (i + 1)..len {
                if !circuits[i].is_disjoint(&circuits[j]) {
                    // println!(
                    //     "found intersection with {:?}",
                    //     circuits[i].intersection(&circuits[j])
                    // );
                    circuits[i] = circuits[i]
                        .union(&circuits[j])
                        .copied()
                        .collect::<BTreeSet<_>>();
                    circuits.swap_remove(j);
                    continue 'outer2;
                }
            }
        }
        break;
    }

    // println!("--- AFTER MERGE ---");

    // for circuit in &circuits {
    //     println!("{circuit:?}");
    // }

    // println!("len: {}", circuits.len());
    // println!("len of first: {}", circuits[0].len());

    let three_largest_multiplied = circuits
        .iter()
        .map(|circuit| circuit.len())
        .k_largest(3)
        .product::<usize>();

    Some(three_largest_multiplied as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut boxes = input
        .lines()
        .map(|line| {
            let (x, y, z) = line.split(',').collect_tuple().unwrap();
            (
                x.parse::<u64>().unwrap(),
                y.parse::<u64>().unwrap(),
                z.parse::<u64>().unwrap(),
            )
        })
        .collect::<BTreeSet<_>>();

    let mutual_distances = boxes
        .iter()
        .combinations(2)
        .map(|v| {
            let one = v[0];
            let two = v[1];
            (
                (*one, *two),
                one.0.abs_diff(two.0).pow(2)
                    + one.1.abs_diff(two.1).pow(2)
                    + one.2.abs_diff(two.2).pow(2),
            )
        })
        .sorted_by_key(|(_, dist)| *dist)
        .collect::<Vec<(((u64, u64, u64), (u64, u64, u64)), u64)>>();

    let mut circuits = Vec::<BTreeSet<(u64, u64, u64)>>::new();

    for connection in mutual_distances {
        let (left, right) = connection.0;

        // Add new connection
        let mut i = 0;
        while i < circuits.len() {
            if circuits[i].contains(&left)
            // && boxes.contains(right) {
            {
                // println!("Found left box in existing circuit");
                boxes.remove(&right);
                circuits[i].insert(right);
            } else if circuits[i].contains(&right)
            // boxes.contains(left) {
            {
                // println!("Found right box in existing circuit");
                boxes.remove(&left);
                circuits[i].insert(left);
            } else {
                i += 1;
                continue;
            }

            // Merge
            for j in 0..circuits.len() {
                if circuits[i] != circuits[j] && !circuits[j].is_disjoint(&circuits[i]) {
                    // println!(
                    //     "found intersection with {:?}",
                    //     circuits[i].intersection(&circuits[j])
                    // );
                    circuits[i] = circuits[i]
                        .clone()
                        .union(&circuits[j])
                        .copied()
                        .collect::<BTreeSet<_>>();
                    circuits.swap_remove(j);
                    break;
                }
            }

            i += 1;
        }

        let mut new_circuit = BTreeSet::<(u64, u64, u64)>::new();
        if boxes.contains(&left) {
            new_circuit.insert(left);
            boxes.remove(&left);
        }
        if boxes.contains(&right) {
            new_circuit.insert(right);
            boxes.remove(&right);
        }
        if !new_circuit.is_empty() {
            circuits.push(new_circuit);
        }
        // println!("{}", circuits.len());

        if boxes.is_empty() && circuits.len() == 1 {
            return Some(connection.0.0.0 * connection.0.1.0);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
