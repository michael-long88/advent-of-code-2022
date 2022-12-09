use nalgebra::{Matrix, Dynamic, VecStorage};

pub type InputMatrix = Matrix<u32, Dynamic, Dynamic, VecStorage<u32, Dynamic, Dynamic>>;

pub fn parse(input: &str) -> InputMatrix {
    let rows = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();
    
    let input_matrix = InputMatrix::from_row_slice(rows.len(), rows[0].len(), &rows.iter().flat_map(|tree| tree.to_owned()).collect::<Vec<u32>>()[..]);

    input_matrix
}

pub fn part_one(input: &str) -> Option<u32> {
    let forest = parse(input);
    let mut count = 0;
    for row_index in 0..forest.nrows() {
        if row_index == 0 || row_index == forest.nrows() - 1 {
            // no neighbors
            count += forest.ncols();
            continue;
        }
        for col_index in 0..forest.ncols() {
            if col_index == 0 || col_index == forest.ncols() - 1 {
                // no neighbors
                count += 1;
                continue;
            }
            let forest_row = forest.row(row_index);
            let forest_row = forest_row.iter().collect::<Vec<&u32>>();
            
            let mut max = 0;
            let mut left = 0;
            forest_row[..col_index + 1].iter().enumerate().for_each(|(idx, &value)| {
                if value > &max {
                    max = *value;
                    left = idx;
                }
            });
            max = 0;

            let mut right = 0;
            forest_row[col_index..].iter().enumerate().for_each(|(idx, &value)| {
                if value >= &max {
                    max = *value;
                    right = idx;
                }
            });
            max = 0;

            let forest_col = forest.column(col_index);
            let forest_col = forest_col.iter().collect::<Vec<&u32>>();

            let mut up = 0;
            forest_col[..row_index + 1].iter().enumerate().for_each(|(idx, &value)| {
                if value > &max {
                    max = *value;
                    up = idx;
                }
            });
            max = 0;

            let mut down = 0;
            forest_col[row_index..].iter().enumerate().for_each(|(idx, &value)| {
                if value >= &max {
                    max = *value;
                    down = idx;
                }
            });

            if left == col_index || right == 0 || up == row_index || down == 0 {
                count += 1;
            }
        }
    }

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let forest = parse(input);
    let mut scenic_score = 0;
    for row_index in 0..forest.nrows() {
        if row_index == 0 || row_index == forest.nrows() - 1 {
            // no neighbors
            continue;
        }
        for col_index in 0..forest.ncols() {
            if col_index == 0 || col_index == forest.ncols() - 1 {
                // no neighbors
                continue;
            }
            let forest_row = forest.row(row_index);
            let forest_row = forest_row.iter().collect::<Vec<&u32>>();
            
            let mut left = 0;
            for (idx, &value) in forest_row[..col_index].iter().rev().enumerate() {
                if value >= &forest[(row_index, col_index)] {
                    left = idx + 1;
                    break;
                }
                left = idx + 1;
            }

            let mut right = 0;
            for (idx, &value) in forest_row[col_index + 1..].iter().enumerate() {
                if value >= &forest[(row_index, col_index)] {
                    right = idx + 1;
                    break;
                }
                right = idx + 1;
            }

            let forest_col = forest.column(col_index);
            let forest_col = forest_col.iter().collect::<Vec<&u32>>();

            let mut up = 0;
            for (idx, &value) in forest_col[..row_index].iter().rev().enumerate() {
                if value >= &forest[(row_index, col_index)] {
                    up = idx + 1;
                    break;
                }
                up = idx + 1;
            }

            let mut down = 0;
            for (idx, &value) in forest_col[row_index + 1..].iter().enumerate() {
                if value >= &forest[(row_index, col_index)] {
                    down = idx + 1;
                    break;
                }
                down = idx + 1;
            }

            let tree_scenic_score = left * right * up * down;
            if tree_scenic_score > scenic_score {
                scenic_score = tree_scenic_score;
            }
        }
    }

    Some(scenic_score as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
