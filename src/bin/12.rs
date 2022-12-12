use std::{vec, collections::VecDeque};

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
    .lines()
    .map(|l| l.chars().collect())
    .collect()
}

fn bfs(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> Option<u32> {
    let mut q: VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mut visited = vec![vec![false;grid[0].len()];grid.len()];

    q.push_back((start.0, start.1, 0));
    visited[start.0][start.1] = true;

    while let Some((x, y, steps)) = q.pop_front() {
        if (x, y) == end {
            return Some(steps as u32);
        }

        for (dx, dy) in DIRECTIONS {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;


            if new_x < 0 || new_x >= grid.len() as i32 || new_y < 0 || new_y >= grid[0].len() as i32 {
                continue;
            }

            let val = grid[x][y] as i32;
            let new_val = grid[new_x as usize][new_y as usize] as i32;

            if !visited[new_x as usize][new_y as usize] && new_val - val <= 1{
                q.push_back((new_x as usize, new_y as usize, steps + 1));
                visited[new_x as usize][new_y as usize] = true;
            }
        }
    }

    None
}


pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = parse_input(input);

    let mut start = (0, 0);
    let mut end = (0, 0);

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            match grid[i][j] {
                'S' => {
                    start = (i,j);
                    grid[i][j] = 'a';
                },
                'E' => {
                    end = (i, j);
                    grid[i][j] = 'z';
                },
                _ => (),
            }
        }
    }
    
    bfs(&grid, start, end)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut min = u32::MAX;
    let mut grid = parse_input(input);
    let mut starts: Vec<(usize, usize)> = Vec::new();
    let mut end = (0, 0);

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            match grid[i][j] {
                'S' => {
                    starts.push((i,j));
                    grid[i][j] = 'a';
                },
                'E' => {
                    end = (i, j);
                    grid[i][j] = 'z';
                },
                'a' => {
                    starts.push((i,j));
                },
                _ => (),
            }
        }
    }
    
    for start in starts {
        if let Some(x) = bfs(&grid, start, end) {
            if x < min as u32 {
                min = x;
            }
        }
    }

    Some(min)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
