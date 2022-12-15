// only looking at sensors with a beacon that has y=2_000_000
// calculate manhattan distance for each sensor to it's corresponding beacon
// add coordinates in row 2_000_000 to a set that are within that manhattan distance

use std::collections::HashSet;

pub struct Sensor {
    pub x: i64,
    pub y: i64,
    pub radius: i64,
}

pub struct Beacon {
    pub x: i64,
    pub y: i64,
}

impl Sensor {
    pub fn manhattan_dist(&self, other: &Beacon) -> i64 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as i64
    }
}

pub fn manhattan_dist(sensor: &Sensor, beacon: &Beacon) -> i64 {
    (sensor.x.abs_diff(beacon.x) + sensor.y.abs_diff(beacon.y)) as i64
}

pub struct Tunnels {
    pub network: Vec<(Sensor, Beacon)>,
}

pub fn get_device_coordinates(input: &str) -> (i64, i64) {
    let device_split = input.split(" at ").last().unwrap();
    let x = device_split
        .split(", ")
        .next()
        .unwrap()
        .split('=')
        .last()
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let y = device_split
        .split(", ")
        .last()
        .unwrap()
        .split('=')
        .last()
        .unwrap()
        .parse::<i64>()
        .unwrap();

    (x, y)
}

pub fn parse(input: &str) -> Tunnels {
    let mut network: Vec<(Sensor, Beacon)> = vec![];

    input
        .lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let mut positions = line.split(": ");
            let sensor_split = positions.next().unwrap();
            let beacon_split = positions.next().unwrap();
            let (sensor_x, sensor_y) = get_device_coordinates(sensor_split);
            let (beacon_x, beacon_y) = get_device_coordinates(beacon_split);
            let mut sensor = Sensor {
                x: sensor_x,
                y: sensor_y,
                radius: 1,
            };
            let beacon = Beacon {
                x: beacon_x,
                y: beacon_y,
            };
            sensor.radius = manhattan_dist(&sensor, &beacon);
            network.push((sensor, beacon));
        });

    Tunnels { network }
}

pub fn calculate_invalid_beacon_placements(input: &str, row_num: i64) -> Option<u32> {
    let tunnels = parse(input);
    let mut invalid_beacon_placements: HashSet<(i64, i64)> = HashSet::new();
    tunnels
        .network
        .iter()
        .for_each(|(sensor, _)| {
            if (sensor.radius - sensor.y.abs_diff(row_num) as i64) < 0 {
                return;
            }
            let min_x = sensor.x - (sensor.radius - sensor.y.abs_diff(row_num) as i64);
            let max_x = sensor.x + (sensor.radius - sensor.y.abs_diff(row_num) as i64);
            (min_x..=max_x).for_each(|x| {
                invalid_beacon_placements.insert((x, row_num));
            })
        });

    let existing_beacon_count = tunnels
        .network
        .iter()
        .map(|(_, beacon)| (beacon.x, beacon.y))
        .collect::<HashSet<(i64, i64)>>()
        .iter()
        .filter(|(x, y)| y == &row_num && invalid_beacon_placements.contains(&(*x, *y)))
        .count();

    Some(invalid_beacon_placements.len() as u32 - existing_beacon_count as u32)
}

pub fn calculate_tuning_frequency(input: &str, singal_strength: i64) -> Option<i64> {
    let tunnels = parse(input);
    let sensors = tunnels
        .network
        .iter()
        .map(|(sensor, _)| sensor)
        .collect::<Vec<&Sensor>>();
    for sensor in sensors.iter() {
        for x in (sensor.x - sensor.radius - 1)..=(sensor.x + sensor.radius + 1) {
            if x > singal_strength {
                break;
            } else if x < 0 {
                continue;
            }

            let delta_y = sensor.radius - (x - sensor.x).abs() + 1;
            'a: for y in [sensor.y + delta_y, sensor.y - delta_y] {
                if y <= singal_strength && y >= 0 {
                    for adjacent_sensor in sensors.iter() {
                        if (adjacent_sensor.x - x).abs() + (adjacent_sensor.y - y).abs() <= adjacent_sensor.radius {
                            break 'a;
                        }
                    }
                    return Some(x * 4_000_000 + y);
                }
            }
        }
    }
    None
}

pub fn part_one_test(input: &str) -> Option<u32> {
    calculate_invalid_beacon_placements(input, 10)
}

pub fn part_one(input: &str) -> Option<u32> {
    calculate_invalid_beacon_placements(input, 2_000_000)
}

pub fn part_two_test(input: &str) -> Option<i64> {
    calculate_tuning_frequency(input, 20)
}

pub fn part_two(input: &str) -> Option<i64> {
    calculate_tuning_frequency(input, 4_000_000)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one_test(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two_test(&input), Some(56_000_011));
    }
}
