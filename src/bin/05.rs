use std::{collections::VecDeque, vec};

pub fn solve(input: &str, reverse: bool) -> Option<String> {
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new();9];
    let (cargos, moves) = input.split_once("\n\n").unwrap();

    for line in cargos.lines().rev() {
        let mut stack_idx = 0;
        for i in (1..line.len()).step_by(4){
            let c  = line.chars().nth(i).unwrap();
            
            if c.is_numeric() || c == ' ' {
                stack_idx += 1;
                continue;
            }

            stacks[stack_idx].push_back(c);
            stack_idx += 1;
        }
    }

    for cargo_move in moves.lines() {
        let parts = cargo_move.split(" ").collect::<Vec<&str>>();
        let count = parts[1].parse::<usize>().unwrap();
        let from = parts[3].parse::<usize>().unwrap() - 1;
        let to = parts[5].parse::<usize>().unwrap() - 1;

        let mut buffer: VecDeque<char> = VecDeque::new();

        for _ in 0..count {
            let c = stacks[from].pop_back().unwrap();

            if reverse { 
                buffer.push_front(c);
            } else { 
                buffer.push_back(c);
            }
        }

        buffer.iter().for_each(|x| stacks[to].push_back(*x));

    }

    let mut out = String::new();

    for i in 0..stacks.len() {
        if let Some(x) = stacks[i].pop_back() {
            out.push(x);
        }
    }

    Some(out)
}

pub fn part_one(input: &str) -> Option<String> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<String> {
    solve(input, true)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from_str("CMZ").unwrap()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from_str("MCD").unwrap()));
    }
}
