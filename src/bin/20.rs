pub struct EncryptedNumber {
    pub value: i64,
    pub position: usize
}

pub struct RingBuffer {
    pub buffer: Vec<EncryptedNumber>
}

impl RingBuffer {
    pub fn mix(&mut self, current_index: usize) {
        let buffer_lenth = self.buffer.len() as i64 - 1 ;
        let index = self.buffer
            .iter()
            .position(|en| en.position == current_index)
            .unwrap();
        let mut new_index = index as i64 + self.buffer[index].value;
        new_index = ((new_index % buffer_lenth) + buffer_lenth) % buffer_lenth;
        let number = self.buffer.remove(index);
        self.buffer.insert(new_index as usize, number);
    } 
}

pub fn parse(input: &str, decryption_key: i64) -> RingBuffer {
    let buffer = input
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.is_empty())
        .map(|(position, line)| {
            EncryptedNumber {
                value: line.parse::<i64>().unwrap() * decryption_key,
                position
            }
        })
        .collect();

    RingBuffer {
        buffer
    }
}

pub fn decrypt(input: &str, decryption_key: i64, cycles: u32) -> Option<i64> {
    let mut encrypted_numbers = parse(input, decryption_key);
    let encrypted_numbers_lenth = encrypted_numbers.buffer.len();
    for _ in 0..cycles {
        for current_index in 0..encrypted_numbers_lenth {
            encrypted_numbers.mix(current_index);
        }
    }
    let zero_index = encrypted_numbers.buffer
        .iter()
        .position(|en| en.value == 0 )
        .unwrap();
    
    let coordinate_sum: i64 = vec![1000, 2000, 3000]
        .iter()
        .map(|num| {
            encrypted_numbers.buffer[(zero_index + num) % encrypted_numbers_lenth].value
        })
        .sum();

    Some(coordinate_sum as i64)
}

pub fn part_one(input: &str) -> Option<i64> {
    decrypt(input, 1, 1)
}

pub fn part_two(input: &str) -> Option<i64> {
    decrypt(input, 811_589_153, 10)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1_623_178_306));
    }
}
