enum Shape {
   Rock,
   Paper,
   Scissors
}

fn parse_shape(input: &str) -> Shape {
    match input {
        "A" | "X" => Shape::Rock,
        "B" | "Y" => Shape::Paper,
        "C" | "Z" => Shape::Scissors,
        _ => unreachable!()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;

    for line in input.lines() {
        let (opponent, me) = line.split_once(" ").unwrap();

        result += match (parse_shape(opponent), parse_shape(me)) {
            (Shape::Rock, Shape::Paper) => 2 + 6,
            (Shape::Rock, Shape::Scissors) => 3 + 0,
            (Shape::Paper, Shape::Rock) => 1 + 0,
            (Shape::Paper, Shape::Scissors) => 3 + 6,
            (Shape::Scissors, Shape::Rock) => 1 + 6,
            (Shape::Scissors, Shape::Paper) => 2 + 0,
            (Shape::Rock, Shape::Rock) => 1 + 3,
            (Shape::Paper, Shape::Paper) => 2 + 3,
            (Shape::Scissors, Shape::Scissors) => 3 + 3,
            _ => 0
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;

    for line in input.lines() {
        let (opponent, expected_result) = line.split_once(" ").unwrap();

        result += match (parse_shape(opponent), expected_result) {
            (Shape::Rock, "X") => 0 + 3,
            (Shape::Rock, "Y") => 3 + 1,
            (Shape::Rock, "Z") => 6 + 2,
            (Shape::Paper, "X") => 0 + 1,
            (Shape::Paper, "Y") => 3 + 2,
            (Shape::Paper, "Z") => 6 + 3,
            (Shape::Scissors, "X") => 0 + 2,
            (Shape::Scissors, "Y") => 3 + 3,
            (Shape::Scissors, "Z") => 6 + 1,
            _ => 0
        }

        
    }

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
