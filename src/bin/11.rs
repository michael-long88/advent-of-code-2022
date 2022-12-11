use std::collections::VecDeque;


#[derive(Debug, Clone)]
pub struct Monkey {
    items_held: VecDeque<u64>,
    operation: String,
    divide_by: u32,
    throw_to_if_true: u32,
    throw_to_if_false: u32,
    times_inspected: u64,
}

impl Monkey {
    pub fn throw_to(&mut self, monkey: &Monkey, items: &Vec<u64>) -> Vec<u64> {
        let mut new_items = vec![];
        for item in items {
            let new_item = match &monkey.operation[..] {
                "new = old + 1" => item + 1,
                "new = old - 1" => item - 1,
                "new = old * 2" => item * 2,
                "new = old / 2" => item / 2,
                _ => panic!("Invalid operation"),
            };
            new_items.push(new_item);
        }
        self.items_held = VecDeque::new();
        new_items
    }
}

pub fn parse(input: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];

    for monkey_block in input.split("\n\n") {
        let mut lines = monkey_block.lines();
        let _id = lines.next().unwrap().strip_suffix(':').unwrap().split_whitespace().last().unwrap().parse::<u32>().unwrap();
        let items_held = lines.next()
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<VecDeque<u64>>();
        let operation = lines.next().unwrap().strip_prefix("  Operation: new = old ").unwrap().to_owned();
        let divide_by = lines.next().unwrap().strip_prefix("  Test: divisible by ").unwrap().parse::<u32>().unwrap();
        let throw_to_if_true = lines.next().unwrap().strip_prefix("    If true: throw to monkey ").unwrap().parse::<u32>().unwrap();
        let throw_to_if_false = lines.next().unwrap().strip_prefix("    If false: throw to monkey ").unwrap().parse::<u32>().unwrap();
        monkeys.push(Monkey {
            items_held,
            operation,
            divide_by,
            throw_to_if_true,
            throw_to_if_false,
            times_inspected: 0,
        });
    }
    monkeys
}

pub fn calculate_monkey_business(input: &str, num_rounds: u32, managing_stress: bool) -> Option<u64> {
    let mut completed_rounds = 0;
    let mut worry_level;
    let mut monkeys = parse(input);
    let monkey_len = monkeys.len();
    let mut monkey_to_send_to;
    let magic_trick = monkeys
        .iter()
        .map(|monkey| monkey.divide_by as u64)
        .product::<u64>();

    while completed_rounds < num_rounds {
        for monkey_idx in 0..monkey_len {
            for _ in 0..monkeys[monkey_idx].items_held.len() {
                let monkey = monkeys.get_mut(monkey_idx).unwrap();
                let item = monkey.items_held.pop_front().unwrap();
                let mut operations = monkey.operation.split_whitespace();
                match operations.next().unwrap() {
                    "+" => {
                        worry_level = item + operations.next().unwrap().parse::<u64>().unwrap_or(item);
                        worry_level %= magic_trick;
                    },
                    "-" => {
                        worry_level = item - operations.next().unwrap().parse::<u64>().unwrap_or(item);
                        worry_level %= magic_trick;
                    },
                    "*" => {
                        worry_level = item * operations.next().unwrap().parse::<u64>().unwrap_or(item);
                        worry_level %= magic_trick;
                    },
                    "/" => {
                        worry_level = item / operations.next().unwrap().parse::<u64>().unwrap_or(item);
                        worry_level %= magic_trick;
                    },
                    _ => panic!("Invalid operation"),
                }
                if managing_stress {
                    worry_level /= 3;
                }
                monkey.times_inspected += 1;
                if worry_level % monkey.divide_by as u64 == 0 {
                    monkey_to_send_to = monkey.throw_to_if_true as usize;
                } else {
                    monkey_to_send_to = monkey.throw_to_if_false as usize;
                };
                monkeys.get_mut(monkey_to_send_to).unwrap().items_held.push_back(worry_level);
            }
            monkeys[monkey_idx].items_held = VecDeque::new();
        }
        completed_rounds += 1;
    };

    let mut monkey_business: Vec<u64> = monkeys.iter().map(|monkey| monkey.times_inspected).collect();
    monkey_business.sort();

    Some(monkey_business[monkey_business.len() - 2] * monkey_business[monkey_business.len() - 1])
}


pub fn part_one(input: &str) -> Option<u64> {
    calculate_monkey_business(input, 20, true)
}

pub fn part_two(input: &str) -> Option<u64> {
    calculate_monkey_business(input, 10_000, false)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10_605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2_713_310_158));
    }
}
