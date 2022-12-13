use itertools::Itertools;
use serde_json::Value;
use std::{cmp::{max, Ordering}, io::Lines};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}

impl TryFrom<Value> for Packet {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Number(num) => Ok(Self::Integer(num.as_u64().unwrap() as u32)),
            Value::Array(arr) => Ok(Self::List(arr.iter().map(|v| Packet::try_from(v.clone()).unwrap()).collect())),
            _ => Err("Invalid Packet".to_owned()),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Integer(left_packet_num), Self::Integer(right_packet_num)) => left_packet_num.cmp(right_packet_num),
            (Self::Integer(_), Self::List(_)) => Self::List(vec![self.clone()]).cmp(other),
            (Self::List(_), Self::Integer(_)) => self.cmp(&Self::List(vec![other.clone()])),
            (Self::List(left_packet_list), Self::List(right_packet_list)) => {
                for (left_packet_num, right_packet_num) in left_packet_list.iter().zip(right_packet_list) {
                    if left_packet_num.cmp(right_packet_num) != Ordering::Equal {
                        return left_packet_num.cmp(right_packet_num);
                    }
                }
                left_packet_list.len().cmp(&right_packet_list.len())
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let signals = input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let packet_json = serde_json::from_str::<Value>(line).unwrap();
            Packet::try_from(packet_json).unwrap()
        })
        .collect::<Vec<_>>();
    let pair_sums: u32 = signals.iter()
        .tuples()
        .positions(|(a,b)| a.cmp(b) != Ordering::Greater)
        .map(|i| i as u32 + 1)
        .sum();
    Some(pair_sums)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut signals = input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let packet_json = serde_json::from_str::<Value>(line).unwrap();
            Packet::try_from(packet_json).unwrap()
        })
        .collect::<Vec<_>>();
    let beacons = [
        Packet::try_from(serde_json::from_str::<Value>("[[2]]").unwrap()).unwrap(),
        Packet::try_from(serde_json::from_str::<Value>("[[6]]").unwrap()).unwrap(),
    ];
    signals.extend(beacons.iter().cloned());
    signals.sort();
    let decoder_key = signals.iter().positions(|b| beacons.contains(b)).map(|i| i as u32 + 1).product();

    Some(decoder_key)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
