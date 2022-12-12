use pathfinding::prelude::astar;


const UPPERCASE_DIFF: u8 = 38;
const UPPERCASE_Z: u8 = 52;
const LOWERCASE_Z: u8 = 26;
const LOWERCASE_DIFF: u8 = 58;
const START_SCORE: u8 = 45;
const LOWERCASE_A: u8 = 1;
const END_SCORE: u8 = 31;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn distance(&self, other: &Pos) -> u32 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as u32
    }

    pub fn successors(&self, rows: &[Vec<u8>]) -> Vec<(Pos, u32)> {
        let row_index = self.y;
        let col_index = self.x;
        let mut successors: Vec<Pos> = vec![];
        let item = rows[row_index][col_index];
        let row_length = rows[row_index].len();

        if row_index > 0 && 
            (rows[row_index - 1][col_index] <= item + 1 || 
                (item == LOWERCASE_Z && rows[row_index - 1][col_index] == END_SCORE)
            ) 
        {
            successors.push(Pos {
                x: col_index,
                y: (row_index - 1),
            });
        }

        if row_index < rows.len() - 1 && 
            (rows[row_index + 1][col_index] <= item + 1 ||
                (item == LOWERCASE_Z && rows[row_index + 1][col_index] == END_SCORE)    
            ) 
        {
            successors.push(Pos {
                x: col_index,
                y: (row_index + 1),
            });
        }

        if col_index > 0 && 
            (rows[row_index][col_index - 1] <= item + 1 || 
                (item == LOWERCASE_Z && rows[row_index][col_index -1] == END_SCORE)
            ) 
        {
            successors.push(Pos {
                x: (col_index - 1),
                y: row_index,
            });
        }

        if col_index < row_length - 1 && 
            (rows[row_index][col_index + 1] <= item + 1 || 
                (item == LOWERCASE_Z && rows[row_index][col_index + 1] == END_SCORE)
            ) 
        {
            successors.push(Pos {
                x: (col_index + 1),
                y: row_index,
            });
        }

        successors.into_iter().map(|pos| (pos, 1)).collect()
    }
}

pub fn score_item(item: char) -> u8 {
    let mut scored_item = item as u8 - UPPERCASE_DIFF;
    if scored_item > UPPERCASE_Z {
        scored_item -= LOWERCASE_DIFF;
    }
    scored_item as u8
}

pub fn parse(input: &str) -> Vec<Vec<u8>> {
    let rows = input
        .split('\n')
        .filter(|rucksack| !rucksack.is_empty())
        .map(|row| row.chars().map(score_item).collect::<Vec<u8>>())
        .collect();

    rows
}

pub fn get_start_and_end(rows: &[Vec<u8>]) -> (Pos, Pos) {
    let mut start_pos = Pos {
        x: 0,
        y: 0,
    };
    let mut end_pos = Pos {
        x: 0,
        y: 0,
    };
    for (row_index, row) in rows.iter().enumerate() {
        for (col_index, item) in row.iter().enumerate() {
            if *item == START_SCORE {
                start_pos = Pos {
                    x: col_index,
                    y: row_index,
                };
            } else if *item == END_SCORE {
                end_pos = Pos {
                    x: col_index,
                    y: row_index,
                };
            }
        }
    }
    (start_pos, end_pos)
}

pub fn get_all_possible_starts(rows: &[Vec<u8>]) -> Vec<Pos> {
    let mut start_pos = vec![];
    for (row_index, row) in rows.iter().enumerate() {
        for (col_index, item) in row.iter().enumerate() {
            if *item == START_SCORE || *item == LOWERCASE_A {
                start_pos.push(Pos {
                    x: col_index,
                    y: row_index,
                });
            }
        }
    }
    start_pos
}

pub fn part_one(input: &str) -> Option<u32> {
    let elevations = parse(input);
    let (start_pos, end_pos) = get_start_and_end(&elevations);
    let result = astar(
        &start_pos, 
        |pos: &Pos| pos.successors(&elevations), 
        |pos| pos.distance(&end_pos), 
        |pos| pos == &end_pos
    );

    Some(result.expect("no path found").1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let elevations = parse(input);
    let (_start_pos, end_pos) = get_start_and_end(&elevations);
    let possible_starts = get_all_possible_starts(&elevations);

    let mut shortest: Vec<u32> = vec![];

    for start in possible_starts {
        let result = astar(
            &start, 
            |pos: &Pos| pos.successors(&elevations), 
            |pos| pos.distance(&end_pos), 
            |pos| pos == &end_pos
        );
        if let Some(dist) = result {
            shortest.push(dist.1);
        }
    }

    Some(*shortest.iter().min().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
