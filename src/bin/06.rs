use std::collections::HashSet;


pub fn parse(input: &str) -> &str {
    input
        .lines()
        .collect::<Vec<&str>>()[0]
}

pub fn get_start_of_message_marker(input: &str, offset: usize) -> u32 {
    let mut sequence_length: usize = 0;
    let packet_markers = parse(input)
        .chars()
        .collect::<Vec<char>>();

    for (index, window) in packet_markers.windows(offset).enumerate() {
        if window.iter().collect::<HashSet<_>>().len() == offset {
            sequence_length = index + offset;
            break;
        }
    }

    sequence_length as u32
}

pub fn part_one(input: &str) -> Option<u32> {    
    Some(get_start_of_message_marker(input, 4))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(get_start_of_message_marker(input, 14))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(23));
    }
}
