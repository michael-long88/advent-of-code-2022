pub fn parse(input: &str) -> Vec<&str> {
    let game_rounds = input
        .split('\n')
        .filter(|game_round| !game_round.is_empty())
        .collect();

    game_rounds
}

pub fn get_score(shape1: &str, shape2: &str) -> u32 {
    match shape1 {
        "A" => match shape2 {  // Rock
            "X" => 4,  // Rock
            "Y" => 8,  // Paper
            "Z" => 3,  // Scissors
            &_ => 0
        },
        "B" => match shape2 {  // Paper
            "X" => 1,  // Rock
            "Y" => 5,  // Paper
            "Z" => 9,  // Scissors
            &_ => 0
        },
        "C" => match shape2 {  // Scissors
            "X" => 7,  // Rock
            "Y" => 2,  // Paper
            "Z" => 6,  // Scissors
            &_ => 0
        },
        &_ => 0
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let game_rounds = parse(input);
    let score = game_rounds
        .iter()
        .map(|game_round| {
            let shapes = game_round.split(' ').collect::<Vec<&str>>();
            get_score(shapes[0], shapes[1])

        })
        .sum();

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let game_rounds = parse(input);
    let score = game_rounds
        .iter()
        .map(|game_round| {
            let shapes = game_round.split(' ').collect::<Vec<&str>>();
            match shapes[0] {
                "A" => match shapes[1] {  // Rock
                    "X" => get_score(shapes[0], "Z"),  // Lose
                    "Y" => get_score(shapes[0], "X"),  // Draw
                    "Z" => get_score(shapes[0], "Y"),  // Win
                    &_ => 0
                },
                "B" => match shapes[1] {  // Paper
                    "X" => get_score(shapes[0], "X"),  // Lose
                    "Y" => get_score(shapes[0], "Y"),  // Draw
                    "Z" => get_score(shapes[0], "Z"),  // Win
                    &_ => 0
                },
                "C" => match shapes[1] {  // Scissors
                    "X" => get_score(shapes[0], "Y"),  // Lose
                    "Y" => get_score(shapes[0], "Z"),  // Draw
                    "Z" => get_score(shapes[0], "X"),  // Win
                    &_ => 0
                },
                &_ => 0
            }

        })
        .sum();

    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(46));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(56));
    }
}
