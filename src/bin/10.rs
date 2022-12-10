pub struct Signal {
    pub cycles: i32,
    pub value: i32,
}

fn parse_input(input: &str) -> Vec<Signal> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_whitespace())
        .map(|mut line| {
            let instruction = line.next().unwrap();
            let value = match line.next() {
                Some(value) => value.parse::<i32>().unwrap_or(0_i32),
                None => 0_i32,
            };
            match instruction {
                "noop" => Signal { cycles: 1, value },
                "addx" => Signal { cycles: 2, value },
                _ => panic!("Invalid instruction"),
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<i32> {
    let cycles = vec![20, 60, 100, 140, 180, 220];
    let mut registry = 1;
    let mut cycle = 0;
    let mut signal_strengths: Vec<i32> = vec![];
    let signals = parse_input(input);

    for signal in signals {
        for _ in 0..signal.cycles {
            cycle += 1;
            if cycles.contains(&cycle) {
                signal_strengths.push(registry * cycle);
            }
        }
        if cycle > 220 {
            // println!("Above 220");
            break;
        }
        registry += signal.value;
    };

    Some(signal_strengths.iter().sum::<i32>())
}

pub fn part_two(input: &str) -> String {
    let mut crt: Vec<Vec<String>> = vec![vec![]; 6];
    let mut pixel: i32 = 0;
    let mut registry: i32 = 1;
    let mut cycle = 0;
    let mut row;
    let signals = parse_input(input);

    for signal in signals {
        for _ in 0..signal.cycles {
            row = cycle / 40;
            cycle += 1;
            if (registry - pixel).abs() <= 1 {
                crt[row].push("#".to_owned());
            } else {
                crt[row].push(".".to_owned());
            }
            pixel += 1;
            pixel %= 40;
        }
        registry += signal.value;
    };

    for row in crt {
        println!("{}", row.join(""));
    }
    
    "".to_owned()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    part_two(input);
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
        part_two(&input);
    }
}
