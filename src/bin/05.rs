pub fn parse(input: &str) -> Vec<&str> {
    // Input is separated by 2 newlines, dividing the stacks of crates
    // and the directions for moving the crates
    input
        .split("\n\n")
        .filter(|section| !section.is_empty())
        .collect()
}

pub fn get_crates(block: &str) -> Vec<Vec<char>> {
    let rows: Vec<&str> = block
        .split('\n')
        .filter(|section| !section.is_empty())
        .collect();
    let number_of_stacks = (rows[0].len() + 1) / 4;
    let mut boxes = vec![Vec::<char>::new(); number_of_stacks];

    rows
        .iter()
        .for_each(|row| {
            row.chars().enumerate().for_each(|(index, crate_label)| {
                if crate_label.is_ascii_alphabetic() {
                    boxes[index / 4].insert(0, crate_label)
                }
            });
        });

    boxes

}

pub fn get_directions(directions: &str) -> Vec<Vec<u32>> {
    // Since the directions follow the format "move N from S to E",
    // we can just grab every 2nd element when split by whitespace
    directions
        .lines()
        .filter(|section| !section.is_empty())
        .map(|row| {
            row
                .split_whitespace()
                .into_iter()
                .skip(1)
                .step_by(2)
                .map(|num_str| num_str.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

pub fn move_crates(input: &str, keep_order: bool) -> Vec<Vec<char>> {
    let parsed_strings = parse(input);
    let mut stacks = get_crates(parsed_strings[0]);
    let directions = get_directions(parsed_strings[1]);
    // Following the format "move N from S to E", we take the last N elements from stacks[S] and
    // move them to stacks[E]
    directions.iter().for_each(|row| {
        let final_length = stacks[(row[1] - 1) as usize].len().saturating_sub(row[0] as usize);
        let mut tail = stacks[(row[1] - 1) as usize].split_off(final_length);
        if !keep_order {
            tail.reverse();
        }
        stacks[(row[2] - 1) as usize].extend(tail);
    });

    stacks
}

pub fn get_top_crates(stacks: Vec<Vec<char>>) -> String {
    stacks
        .iter()    
        .map(|stack| {
            stack.last().unwrap().to_string()
        })
        .collect::<Vec<String>>()
        .join("")
}

pub fn part_one(input: &str) -> Option<String> {
    let stacks = move_crates(input, false);
    Some(get_top_crates(stacks))

}

pub fn part_two(input: &str) -> Option<String> {
    let stacks = move_crates(input, true);
    Some(get_top_crates(stacks))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_owned()));
    }
}
