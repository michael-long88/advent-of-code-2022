use regex::Regex;

pub fn parse(input: &str) -> Vec<&str> {
    // Input is separated by 2 newlines, dividing the stacks of crates
    // and the directions for moving the crates
    input
        .split("\n\n")
        .filter(|section| !section.is_empty())
        .collect()
}

pub fn get_crates(block: &str) -> Vec<Vec<&str>> {
    // Each row has a mix of either 3 or 4 spaces (e.g., "   ", "    ") or
    // a crate followed by a space (e.g., "[A] ")
    // This regex captures that pattern and then separates each row into a `Vec<&str>`
    // based on that pattern
    let crate_regex: regex::Regex = Regex::new(r"\s{3,4}|\[\w\] ?").unwrap();
    block
        .split('\n')
        .filter(|section| !section.is_empty())
        .map(|row| {
            crate_regex
                .find_iter(row)
                .map(|crate_chunk| crate_chunk.as_str().trim())
                .collect()
        })
        .into_iter()
        .collect()
}

pub fn get_transposed_stacks(crate_chunk: &str) -> Vec<Vec<&str>> {
    let mut crate_rows = get_crates(crate_chunk);
    // Drop the last element that's empty due to it being the number labels
    // in the input
    crate_rows.pop();
    let number_of_stacks = crate_rows[0].len();
    let mut towers: Vec<_> = crate_rows.into_iter().map(|n| n.into_iter()).collect();
    // Transpose the nested vectors (rows) so that each inner vector becomes the stacks
    // of crates with the "top" crate being the last element in each vector
    (0..number_of_stacks)
        .map(|_| {
            towers
                .iter_mut()
                .map(|n| n.next().unwrap())
                .rev()
                .filter(|row| !row.is_empty())
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>()
}

pub fn get_directions(directions: &str) -> Vec<Vec<u32>> {
    // Each row follows the format "move <number of crates> from <start stack number> 
    // to <end stack number>"
    // This regex captures that pattern and then separates each row into a `Vec<u32>`
    // based on that pattern
    let direction_regex: regex::Regex = Regex::new(r"[0-9]{1,2}").unwrap();
    directions
        .split('\n')
        .filter(|section| !section.is_empty())
        .map(|row| {
            direction_regex
                .find_iter(row)
                .map(|crate_chunk| crate_chunk.as_str().parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

pub fn move_crates(input: &str, keep_order: bool) -> Vec<Vec<&str>> {
    let parsed_strings = parse(input);
    let mut transposed_stacks = get_transposed_stacks(parsed_strings[0]);
    let directions = get_directions(parsed_strings[1]);
    // Following the format "move N from S to E", we take the last N elements from transposed_stacks[S] and
    // move them to transposed_stacks[E]
    directions.iter().for_each(|row| {
        let final_length = transposed_stacks[(row[1] - 1) as usize].len().saturating_sub(row[0] as usize);
        let mut tail = transposed_stacks[(row[1] - 1) as usize].split_off(final_length);
        if !keep_order {
            tail.reverse();
        }
        transposed_stacks[(row[2] - 1) as usize].extend(tail);
    });

    transposed_stacks
}

pub fn get_top_crates(stacks: Vec<Vec<&str>>) -> String {
    // Get the last element in each nested vector ("top crate")
    // and extract the crate label from the element (e.g., "[A]" -> "A")
    // returning the concatenated string of each crate label
    let letter_regex: regex::Regex = Regex::new(r"\w").unwrap();

    stacks
        .iter()    
        .map(|tower| {
            letter_regex.find(tower.last().unwrap())
                .map(|letter| letter.as_str())
                .unwrap()
        })
        .collect::<Vec<&str>>()
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
