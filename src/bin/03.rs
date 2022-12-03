use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let mut acc: u32 = 0;
    for rucksack in input.lines() {
        let idx = rucksack.len()/2;

        let first: HashSet<char> = rucksack[0..idx].chars().collect();
        let second: HashSet<char> = rucksack[idx..].chars().collect();
        
        if let Some(&x) = first.intersection(&second).nth(0) {
            acc += if x.is_lowercase() {
                x as u32 - 96
            } else {
                x as u32 - 38
            }
        }
    }

    Some(acc)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut acc: u32 = 0;

    for group in input.lines().chunks(3).into_iter() {
        let chars = group
            .map(|g| g
                .chars()
                .collect::<HashSet<char>>())
            .collect::<Vec<HashSet<char>>>();

        let res = chars[0]
            .iter()
            .filter(|a| chars[1..]
                .iter()
                .all(|s| s.contains(a))
            ).nth(0);

        if let Some(&x) = res {
            acc += if x.is_lowercase() {
                x as u32 - 96
            } else {
                x as u32 - 38
            }
        }
    }

    Some(acc)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
