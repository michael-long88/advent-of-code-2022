use std::collections::HashSet;


pub enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

#[derive(Debug, Clone)]
pub struct Knot {
    x: i32,
    y: i32,
    visited: Vec<(i32, i32)>,
    tail: bool,
}

impl Default for Knot {
    fn default() -> Self {
        Self::new()
    }
}   

impl Knot {
    pub fn new() -> Self {
        Knot {
            x: 0,
            y: 0,
            visited: vec![(0, 0)],
            tail: false
        }
    }

    pub fn is_adjacent(&self, other: &Knot) -> bool {
        let x_diff = (self.x - other.x).abs();
        let y_diff = (self.y - other.y).abs();
        
        x_diff <= 1 && y_diff <= 1
    }

    pub fn move_relative(&mut self, other: &Knot) {
        loop {
            if self.is_adjacent(other) {
                self.visited.push((self.x, self.y));
                break;
            }
            if self.x != other.x {
                if self.x < other.x {
                    self.x += 1;
                } else {
                    self.x -= 1;
                }
            }
            if self.y != other.y {
                if self.y < other.y {
                    self.y += 1;
                } else {
                    self.y -= 1;
                }
            }
            if self.tail {
                self.visited.push((self.x, self.y));
            }
        }
    }
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Knot>,
}

impl Rope {
    pub fn new(length: usize) -> Self {
        Rope {
            knots: vec![Knot::new(); length],
        }
    }

    pub fn follow(&mut self) {
        for i in 1..self.knots.len() {
            let temp_knot = self.knots[i - 1].clone();
            self.knots[i].move_relative(&temp_knot);
        }
    }
    
    pub fn movement(&mut self, instruction: &Move) {
        match instruction {
            Move::Up(spaces) => {
                for _ in 0..*spaces {
                    self.knots[0].y += 1;
                    self.follow();
                }
            }
            Move::Down(spaces) => {
                for _ in 0..*spaces {
                    self.knots[0].y -= 1;
                    self.follow();
                }
            }
            Move::Left(spaces) => {
                for _ in 0..*spaces {
                    self.knots[0].x -= 1;
                    self.follow();
                }
            }
            Move::Right(spaces) => {
                for _ in 0..*spaces {
                    self.knots[0].x += 1;
                    self.follow();
                }
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|mut line| {
            let direction = line.next().unwrap();
            let distance = line.next().unwrap().parse::<i32>().unwrap();
            match direction {
                "R" => Move::Right(distance),
                "L" => Move::Left(distance),
                "U" => Move::Up(distance),
                "D" => Move::Down(distance),
                _ => panic!("Invalid direction"),
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let instructions = parse_input(input);
    let mut rope = Rope::new(2);
    rope.knots[1].tail = true;
    for instruction in instructions.iter() {
        rope.movement(instruction);
    }

    let tail = &rope.knots[rope.knots.len() - 1];
    let unique_locations: HashSet<(i32, i32)> = tail.visited.iter().cloned().collect();
    Some((unique_locations).len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let instructions = parse_input(input);
    let mut rope = Rope::new(10);
    rope.knots[9].tail = true;
    for instruction in instructions.iter() {
        rope.movement(instruction);
    }

    let tail = &rope.knots[rope.knots.len() - 1];
    let unique_locations: HashSet<(i32, i32)> = tail.visited.iter().cloned().collect();
    Some((unique_locations).len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}