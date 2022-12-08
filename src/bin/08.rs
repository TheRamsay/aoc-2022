pub fn check_line(start_tree: usize, row_span: (usize, usize), column_span: (usize, usize), trees: &Vec<Vec<usize>>) -> bool {
    let prev = start_tree;

    for i in row_span.0..row_span.1 {
        for j in column_span.0..column_span.1 {
            if trees[i][j] >= prev {
                return false;
            }
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let trees: Vec<Vec<usize>> = input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>()).collect();
    let mut count = 0;

    for i in 0..trees.len() {
        for j in 0..trees[i].len() {
            if
                check_line(trees[i][j], (i, i + 1), (0, j), &trees) ||
                check_line(trees[i][j], (i, i + 1), (j + 1, trees[i].len()), &trees) ||
                check_line(trees[i][j], (0, i), (j, j + 1), &trees) ||
                check_line(trees[i][j], (i + 1 , trees.len()), (j, j + 1), &trees)
             {
                count += 1;
            }
        }

    }

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let trees: Vec<Vec<usize>> = input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>()).collect();
    let mut max = 0;

    for i in 0..trees.len() {
        for j in 0..trees[i].len() {
            let mut distances = vec![0, 0, 0, 0];

            for idx in i + 1..trees.len() {
                distances[0] += 1;
                if trees[idx][j] >= trees[i][j] {
                    break;
                }
            }

            for idx in j + 1..trees[0].len() {
                distances[1] += 1;
                if trees[i][idx] >= trees[i][j] {
                    break;
                }
            }

            for idx in (0..i).rev() {
                distances[2] += 1;
                if trees[idx][j] >= trees[i][j] {
                    break;
                }
            }

            for idx in (0..j).rev() {
                distances[3] += 1;
                if trees[i][idx] >= trees[i][j] {
                    break;
                }
            }

            let distance = distances.iter().fold(1, |acc, x| acc * x);
            
            if distance > max {
                max = distance;
            }

        }

    }
    Some(max as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
