const MEASURED_CYCLES: [u32; 6] = [20, 60, 100, 140, 180, 220];

pub fn run_cpu(total_cycles: &mut u32, result: &mut u32, count: u32, register_value: &mut i32) {
    for _ in 0..count{
        *total_cycles += 1;

        if MEASURED_CYCLES.contains(total_cycles) {
            *result += *total_cycles * (*register_value) as u32;
        }
    }
}

pub fn draw_screen(total_cycles: &mut i32, sprite: &mut i32, count: u32, screen: &mut [[u32; 40];6]) {
    for _ in 0..count {
        let current_row = ((*total_cycles / 40) as f32).ceil() as i32;
        let current_column = *total_cycles - (current_row) * 40;

        if current_column == (*sprite - 1) || current_column == *sprite || current_column == (*sprite + 1) {
            screen[current_row as usize][current_column as usize] = 1;
        }

        *total_cycles += 1;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut register_value = 1;
    let mut cycle = 0;
    let mut result = 0;

    for line in input.lines() {
        match line.split_once(" ") {
            None => { run_cpu(&mut cycle, &mut result, 1, &mut register_value); },
            Some((_, value)) => {
                run_cpu(&mut cycle, &mut result, 2, &mut register_value);
                register_value += value.parse::<i32>().unwrap();
            }
        };
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut register_value = 1;
    let mut cycle = 0;
    let mut screen: [[u32; 40]; 6] = [[0; 40]; 6];

    for line in input.lines() {
        match line.split_once(" ") {
            None => { draw_screen(&mut cycle, &mut register_value, 1, &mut screen); },
            Some((_, value)) => {
                draw_screen(&mut cycle, &mut register_value, 2, &mut screen); 
                register_value += value.parse::<i32>().unwrap();
            }
        };
    }

    let mut out = String::new();
    for i in 0..6 {
        for j in 0..40 {
            match screen[i][j] {
                0 => out.push('.'),
                1 => out.push('#'),
                _ => unreachable!()
            };
        }
        out.push('\n');
    }

    out.pop();

    Some(out)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(String::from("##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....")));
    }
}
