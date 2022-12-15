use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> HashMap<u32, HashSet<u32>> {
    let mut cave: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut max_y = 0;

    input.split('\n')
        .filter(|line|!line.is_empty())
        .for_each(|line| {
            line.split(" -> ")
                .map(|coordinate_pair| {
                    let mut split_coordinate = coordinate_pair.split(',');
                    let x = split_coordinate.next().unwrap().parse::<u32>().unwrap();
                    let y = split_coordinate.next().unwrap().parse::<u32>().unwrap();
                    (x, y)
                })
                .collect::<Vec<(u32, u32)>>()
                .windows(2)
                .into_iter()
                .for_each(|window| {
                    let start = window[0];
                    let end = window[1];
                    if start.0 == end.0 {
                        if start.1 > end.1 {
                            if start.1 > max_y {
                                max_y = start.1;
                            }
                            (end.1..=start.1).for_each(|y| {
                                cave.entry(start.0).or_default().insert(y);
                            });
                        } else {
                            if end.1 > max_y {
                                max_y = end.1;
                            }
                            (start.1..=end.1).for_each(|y| {
                                cave.entry(start.0).or_default().insert(y);
                            });
                        }
                    } else if start.1 == end.1 {
                        if start.0 > end.0 {
                            (end.0..=start.0).for_each(|x| {
                                cave.entry(x).or_default().insert(start.1);
                            });
                        } else {
                            (start.0..=end.0).for_each(|x| {
                                cave.entry(x).or_default().insert(start.1);
                            });
                        }
                    }
                })
        });
    cave.entry(0).or_default().insert(max_y + 1);
    
    cave
}

fn shift(part2: bool, cave: &mut HashMap<u32, HashSet<u32>>) -> u32 {
    let mut x = 500;
    let mut y = 0;
    let max_y = *cave.get(&0).unwrap().iter().next().unwrap();
    let mut units_of_sand = 0;

    loop {
        if part2 && (y + 1 == max_y + 2) {
            cave.entry(x).or_default().insert(y);
            x = 500;
            y = 0;
        } else if !part2 && y + 1 >= max_y {
            return units_of_sand;
        }

        if cave.get(&x).is_some() && cave.get(&x).unwrap().get(&(y + 1)).is_some() { // check below
            if cave.get(&(x - 1)).is_some() && cave.get(&(x - 1)).unwrap().get(&(y + 1)).is_some() { // check below & left
                if cave.get(&(x + 1)).is_some() && cave.get(&(x + 1)).unwrap().get(&(y + 1)).is_some() { // check below & right
                    units_of_sand += 1;
                    if part2 && (x == 500 && y == 0) {
                        return units_of_sand;
                    }
                    cave.get_mut(&x).unwrap().insert(y);
                    x = 500;
                    y = 0;
                } else { // shift down & right
                    x += 1;
                    y += 1;
                }
            } else { // shift down & left
                x -= 1;
                y += 1;
            }
        } else { // shift down
            y += 1;
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut cave = parse(input);
    let units_of_sand = shift(false, &mut cave);

    Some(units_of_sand)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cave = parse(input);
    let units_of_sand = shift(true, &mut cave);

    Some(units_of_sand)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
