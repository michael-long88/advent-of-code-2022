use array_tool::vec::Intersect;

const UPPERCASE_DIFF: u8 = 38;
const UPPERCASE_Z: u8 = 52;
const LOWERCASE_DIFF: u8 = 58;


pub fn parse(input: &str) -> Vec<&str> {
    let rucksacks = input
        .split('\n')
        .filter(|rucksack| !rucksack.is_empty())
        .collect();

    rucksacks
}

pub fn score_item(item: &str) -> u32 {
    let mut scored_item = item.as_bytes()[0] - UPPERCASE_DIFF;
    if scored_item > UPPERCASE_Z {
        scored_item -= LOWERCASE_DIFF;
    }
    scored_item as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let count = parse(input)
        .iter()
        .map(|rucksack| {
            let midpoint = (rucksack.len() / 2) as usize;
            let split_rucksack = rucksack.split_at(midpoint);
            vec![split_rucksack.0, split_rucksack.1]
        })
        .map(|rucksack| {
            let shared_item = rucksack[0]
                .split("")
                .filter(|letter| !letter.is_empty())
                .collect::<Vec<&str>>()
                .intersect(rucksack[1]
                    .split("")
                    .filter(|letter| !letter.is_empty())
                    .collect()
                )[0];
            score_item(shared_item)
            
        })
        .sum();
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let count = parse(input)
        .chunks(3)
        .map(|chunk| {
            let shared_items1 = chunk[0]
                .split("")
                .filter(|letter| !letter.is_empty())
                .collect::<Vec<&str>>()
                .intersect(chunk[1]
                    .split("")
                    .filter(|letter| !letter.is_empty())
                    .collect()
                );
            let shared_items2 = chunk[1]
                .split("")
                .filter(|letter| !letter.is_empty())
                .collect::<Vec<&str>>()
                .intersect(chunk[2]
                    .split("")
                    .filter(|letter| !letter.is_empty())
                    .collect()
                );
            let shared_item = shared_items1.intersect(shared_items2)[0];
            score_item(shared_item)
        })
        .sum();
    Some(count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
