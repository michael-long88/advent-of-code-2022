pub fn parse(input: &str) -> Vec<Vec<u32>> {
    let sectors = input
        .split('\n')
        .filter(|sector| !sector.is_empty())
        .map(|sector| {
            let mapped_sector = sector
                .split(',')
                .collect::<Vec<&str>>()
                .iter()    
                .flat_map(|subsector| {
                    subsector.split('-')
                        .map(|c| c.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<u32>>();

            mapped_sector
        })
        .collect();

    sectors
}

pub fn part_one(input: &str) -> Option<u32> {
    let count = parse(input)
       .into_iter()
       .map(|sector_ranges| {
            if (sector_ranges[0] <= sector_ranges[2] && sector_ranges[1] >= sector_ranges[3]) || (sector_ranges[2] <= sector_ranges[0] && sector_ranges[3] >= sector_ranges[1]) {
                1
            } else {
                0
            }
       })
       .sum();

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let count = parse(input)
       .into_iter()
       .map(|sector_range| {
            let range1 = sector_range[0]..=sector_range[1];
            let range2 = sector_range[2]..=sector_range[3];
            if range2.contains(&sector_range[0]) || range2.contains(&sector_range[1]) || range1.contains(&sector_range[2]) || range1.contains(&sector_range[3]) {
                1
            } else {
                0
            }
       })
       .sum();

    Some(count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
