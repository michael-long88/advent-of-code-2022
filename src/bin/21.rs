use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum Operation<'a> {
    Add(&'a str, &'a str),
    Subtract(&'a str, &'a str),
    Multiply(&'a str, &'a str),
    Divide(&'a str, &'a str),
    Equal(&'a str, &'a str),
}

#[derive(Debug, Clone, PartialEq)]
enum Monkey<'a> {
    Yell(i64),
    Operation(Operation<'a>),
    Invalid(i64),
}

fn parse(input: &str, part2: bool, human_value: i64) -> HashMap<&str, Monkey> {
    let mut monkey_map = HashMap::new();
    input
        .lines()
        .for_each(|line| {
            let mut parts = line.split(": ");
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();
            let value = match value.parse::<i64>() {
                Ok(number) => {
                    if part2 && key == "humn" {
                        Monkey::Yell(human_value as i64)
                    } else {
                        Monkey::Yell(number)
                    }
                },
                Err(_) => {
                    let mut operation_pieces = value.split_whitespace();
                    let left_monkey = operation_pieces.next().unwrap();
                    let mut operation = operation_pieces.next().unwrap();
                    let right_monkey = operation_pieces.next().unwrap();
                    if key == "root" && part2 {
                        operation = "=";
                    }
                    let operation = match operation {
                        "+" => Operation::Add(left_monkey, right_monkey),
                        "-" => Operation::Subtract(left_monkey, right_monkey),
                        "*" => Operation::Multiply(left_monkey, right_monkey),
                        "/" => Operation::Divide(left_monkey, right_monkey),
                        "=" => Operation::Equal(left_monkey, right_monkey),
                        _ => panic!("Unknown operation: {}", operation),
                    };

                    Monkey::Operation(operation)
                }
            };
            monkey_map.insert(key, value);
        });
    monkey_map
}

pub fn yell_number(input: &str, part2: bool, human_value: i64) -> Result<i64, i64> {
    let mut monkey_map = parse(input, part2, human_value);
    let mut root_value: &Monkey = monkey_map.get("root").unwrap();
    // while !root_values_equal && matches!(root_value, Monkey::Operation(_)) {
    while matches!(root_value, Monkey::Operation(_)) {
        let monkey_map_copy = monkey_map.clone();
        for (key, value) in monkey_map_copy.iter() {
            match value {
                Monkey::Yell(_) => {},
                Monkey::Invalid(_) => {},
                Monkey::Operation(operation) => match operation {
                    Operation::Add(left, right) => {
                        let left_value = monkey_map.get(left).unwrap();
                        let right_value = monkey_map.get(right).unwrap();
                        if let (Monkey::Yell(left), Monkey::Yell(right)) =
                            (left_value, right_value)
                        {
                            monkey_map.insert(*key, Monkey::Yell(*left + *right));
                        }
                    },
                    Operation::Subtract(left, right) => {
                        let left_value = monkey_map.get(left).unwrap();
                        let right_value = monkey_map.get(right).unwrap();
                        if let (Monkey::Yell(left), Monkey::Yell(right)) =
                            (left_value, right_value)
                        {
                            monkey_map.insert(*key, Monkey::Yell(*left - *right));
                        }
                    },
                    Operation::Multiply(left, right) => {
                        let left_value = monkey_map.get(left).unwrap();
                        let right_value = monkey_map.get(right).unwrap();
                        if let (Monkey::Yell(left), Monkey::Yell(right)) =
                            (left_value, right_value)
                        {
                            monkey_map.insert(*key, Monkey::Yell(*left * *right));
                        }
                    },
                    Operation::Divide(left, right) => {
                        let left_value = monkey_map.get(left).unwrap();
                        let right_value = monkey_map.get(right).unwrap();
                        if let (Monkey::Yell(left), Monkey::Yell(right)) =
                            (left_value, right_value)
                        {
                            monkey_map.insert(*key, Monkey::Yell(*left / *right));
                        }
                    },
                    Operation::Equal(left, right) => {
                        let left_value = monkey_map.get(left).unwrap();
                        let right_value = monkey_map.get(right).unwrap();
                        if let (Monkey::Yell(left), Monkey::Yell(right)) =
                            (left_value, right_value)
                        {
                            if left == right {
                                monkey_map.insert(*key, Monkey::Yell(human_value));
                            } else {
                                monkey_map.insert(*key, Monkey::Invalid((right - left).abs()));
                            }
                        }
                    }
                },
            }
        }
        root_value = monkey_map.get("root").unwrap();
    }
    match root_value {
        Monkey::Yell(number) => Ok(*number),
        Monkey::Operation(_) => panic!("Root value is still an operation"),
        Monkey::Invalid(number) => Err(*number),
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    match yell_number(input, false, 0) {
        Ok(number) => Some(number),
        Err(_) => None,
    }
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut human_value = 1;
    // let mut learning_value = 0.5;

    loop {
        match yell_number(input, true, human_value) {
            Ok(number) => return Some(number),
            Err(_) => {
                // let human_value += (learning_value * (2 * diff) as f64) as i64;
                // let human_value += (learning_value * diff as f64) as i64;
                // if learning_value >= 0.2 {
                //     learning_value -= 0.1;
                // }
                human_value += 1;
                if human_value % 100_000_000 == 0 {
                    println!("Start value: {}", human_value);
                }
            },
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
