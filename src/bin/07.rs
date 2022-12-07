use std::{collections::HashMap, vec};

use itertools::Itertools;

pub fn calculate_sizes(input: &str) -> HashMap<String, u32> {
    let mut directories: HashMap<String, u32> = HashMap::new();
    let mut path: Vec<String>= Vec::new();

    for line in input.lines() {
        if line.starts_with("$") {
            let parts = line.split(" ").collect::<Vec<_>>();

            if parts.len() == 3 {
                match parts[2] {
                    ".." => { path.pop(); },
                    "/" => { path = vec![String::from("/")]; },
                    _ => path.push(String::from(parts[2])),
                };
            }
        } else {
            if line.starts_with("dir") {
                continue;
            }

            let mut path_builder = String::new();
            for path_part in path.iter() {
                path_builder.push_str(path_part.as_str());
                *directories
                    .entry(path_builder.to_owned())
                    .or_insert(0) += line.split(" ")
                    .flat_map(|x| x.parse::<u32>())
                    .nth(0)
                    .unwrap()
            }
        }
    }

    directories
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        calculate_sizes(input)
        .values()
        .filter(|&&val| val <= 100000)
        .sum()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let directories = calculate_sizes(input);
    let available_space = 70000000 - directories.values().max().unwrap();
    let limit = 30000000;

    Some(
        *(directories
            .values()
            .sorted()
            .find(|&&val| available_space + val >= limit)
            .unwrap()
        )
    )
}


fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
