use itertools::Itertools;

pub fn first_distinct_substring(input: &str, count: usize) -> Option<u32>{
    let chars = input.chars().collect::<Vec<char>>();

    for (i, part) in chars.windows(count).enumerate() {
        if part.into_iter().unique().count() == count {
            return Some((i + count) as u32);
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    return first_distinct_substring(input, 4);
}

pub fn part_two(input: &str) -> Option<u32> {
    return first_distinct_substring(input, 14);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
