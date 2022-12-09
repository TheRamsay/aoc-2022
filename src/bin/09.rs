use std::{collections::HashSet, vec};

const TOUCHING_POSITIONS: [(i32, i32); 9] = [
    (0, 0),
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1)
];

fn play(input: &str, knots_count: usize) -> Option<u32> {
    let mut visited: HashSet<(i32, i32)> = HashSet::from_iter([(0, 0)]);
    let mut knots: Vec<(i32, i32)> = vec![(0, 0); knots_count];

    input.lines().for_each(|line| {
        let (dir, val_raw) = line.split_once(" ").unwrap();
        let val = val_raw.parse::<u32>().unwrap();

        let pattern = match dir {
            "R" => (1, 0),
            "U" => (0, 1),
            "L" => (-1, 0),
            "D" => (0, -1),
            _ => unreachable!()
        };

        make_moves(&mut knots, pattern, &mut visited, val)
    });

    Some(visited.len() as u32)
}


fn make_moves(knots: &mut Vec<(i32, i32)>, pattern: (i32, i32), visited: &mut HashSet<(i32, i32)>, move_count: u32) {
    let last_idx = knots.len() - 1;

    for _ in 0..move_count {
        knots[0].0 += pattern.0;
        knots[0].1 += pattern.1;
        
        for i in 1..knots.len() {
            let (x, y) = (knots[i - 1].0 - knots[i].0, knots[i - 1].1 - knots[i].1);

            if !TOUCHING_POSITIONS.iter().any(|pos| pos.0 == x && pos.1 == y) {
                knots[i].0 += if x.abs() == 1 { x } else { x / 2 };
                knots[i].1 += if y.abs() == 1 { y } else { y / 2 };

                if i == last_idx {
                    visited.insert((knots[i].0, knots[i].1));
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    play(input, 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    play(input, 10)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
