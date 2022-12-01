use std::collections::BinaryHeap;

pub fn get_total_calories(input: &str) -> Vec<u32> {
    let total_calories = input
        .split("\n\n")
        .map(|inventory| inventory.lines().map(|c| c.parse::<u32>().unwrap()).sum())
        .collect::<Vec<u32>>();

    total_calories
}

pub fn part_one(input: &str) -> Option<u32> {
    let total_calories = get_total_calories(input);

    total_calories.iter().max().copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    let total_calories = get_total_calories(input);

    let mut heap = total_calories.iter().copied().collect::<BinaryHeap<u32>>();
    let mut top_three = Vec::new();
    for _ in 0..3 {
        if let Some(v) = heap.pop() {
            top_three.push(v);
        }
    }

    Some(top_three.iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
