use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
        .lines()
        .map(|l|
            l
            .split(['-', ','])
            .map(|s| s.parse::<u32>().unwrap())
            .next_tuple()
            .unwrap()
        )
        .filter(|(a, b, c, d)| a >= c && b <= d || c >= a && d <= b)
        .count() as u32
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
        .lines()
        .map(|l|
            l
            .split(['-', ','])
            .map(|s| s.parse::<u32>().unwrap())
            .next_tuple()
            .unwrap()
        )
        .filter(|(a, b, c, d)| a <= c && c <= b || c <= a && a <= d)
        .count() as u32
    )
}


fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
