use itertools::Itertools;

#[derive(Debug, Clone)]
enum Operation {
    Add(u128),
    Multiply(u128),
    MultiplyByItself
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u128>,
    operation: Operation,
    test_divisor: u128,
    on_true: u128,
    on_false: u128,
    inspects: u128
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    for monkey in input.split("\n\n") {
        let lines: Vec<&str> = monkey.lines().map(|l| l.split_once(":").unwrap().1).collect();

        let starting_items: Vec<u128> = lines[1].trim().split(", ").flat_map(|l| l.parse::<u128>()).collect();

        let operation_line: Vec<&str> = lines[2].split(" ").collect();
        let operation = if let Ok(x) = operation_line[5].parse::<u128>() {
            match operation_line[4] {
                "*" => Operation::Multiply(x),
                "+" => Operation::Add(x),
                _ => unreachable!()
            }
        } else {
            Operation::MultiplyByItself
        };

        let test_divisor = lines[3].split(" ").flat_map(|x| x.parse::<u128>()).next().unwrap();
        let on_true = lines[4].split(" ").flat_map(|x| x.parse::<u128>()).next().unwrap();
        let on_false= lines[5].split(" ").flat_map(|x| x.parse::<u128>()).next().unwrap();

        let monkey = Monkey {
            items: starting_items,
            operation,
            test_divisor,
            on_true,
            on_false,
            inspects: 0
        };

        monkeys.push(monkey);
    }

    monkeys
}
fn print_monkeys(monkeys: &Vec<Monkey>) {
    for (i, m) in monkeys.iter().enumerate() {
        println!("Monkey {} {:?}", i, m.items);
    }
}

fn play(input: &str, rounds: u32, worry_divisor: u128) -> Option<u128> {
    let mut monkeys = parse_monkeys(input);
    let lcm: u128 = monkeys.iter().map(|m| m.test_divisor).product();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys[i].clone();
            let old_items_count = monkey.items.len();

            for j in 0..old_items_count {
                let item = &mut monkey.items[j];

                monkey.inspects += 1;

                match monkey.operation {
                    Operation::Add(x) => *item += x,
                    Operation::Multiply(x) => *item *= x,
                    Operation::MultiplyByItself => *item *= *item
                };

                *item /= worry_divisor;
                *item %= lcm;

                if *item % monkey.test_divisor == 0 {
                    monkeys[monkey.on_true as usize].items.push(*item);
                } else {
                    monkeys[monkey.on_false as usize].items.push(*item);
                }
            }

            for _ in 0..old_items_count {
                monkey.items.remove(0);
            }

            monkeys[i] = monkey;
        }
    }

    Some(
        monkeys
        .iter()
        .map(|m| m.inspects)
        .sorted_by(|a, b| b.cmp(a))
        .take(2)
        .product::<u128>()
    )
}

pub fn part_one(input: &str) -> Option<u128> {
    play(input, 20, 3)
}

pub fn part_two(input: &str) -> Option<u128> {
    play(input, 10000, 1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    parse_monkeys(input);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
