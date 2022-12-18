use adventofcode::read_file;
use regex::Regex;
use std::{collections::HashSet, ops::RangeInclusive};

#[derive(Debug, Clone, Copy)]
struct Position(i64, i64);

impl Position {
    fn get_distance(&self, other: &Self) -> i64 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

#[derive(Debug)]
struct Pair {
    sensor: Position,
    beacon: Position,
    distance: i64,
}

#[derive(Debug, Clone)]
struct MyRange(RangeInclusive<i64>);
impl MyRange {
    fn is_overlapping(&self, other: &Self) -> bool {
        if self.0.contains(other.0.start()) || self.0.contains(other.0.end()) {
            true
        } else {
            false
        }
    }

    fn merge(&self, other: &Self) -> Option<Self> {
        if self.is_overlapping(other) {
            Some(MyRange(
                std::cmp::min(*self.0.start(), *other.0.start())
                    ..=std::cmp::max(*self.0.end(), *other.0.end()),
            ))
        } else {
            None
        }
    }
}

fn main() {
    let file = read_file("15").unwrap();

    let pairs = parse_input(&file);
    let result1 = part1(&pairs, 2000000);
    println!("Part1 {result1}");
}

fn parse_input(input: &str) -> Vec<Pair> {
    let pairs = input
        .trim()
        .split('\n')
        .map(|line| {
            let re = Regex::new(
                r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
            )
            .unwrap();
            let captures = re.captures(line).unwrap();

            let sensor = Position(
                captures.get(1).unwrap().as_str().parse().unwrap(),
                captures.get(2).unwrap().as_str().parse().unwrap(),
            );
            let beacon = Position(
                captures.get(3).unwrap().as_str().parse().unwrap(),
                captures.get(4).unwrap().as_str().parse().unwrap(),
            );

            Pair {
                sensor,
                beacon,
                distance: sensor.get_distance(&beacon),
            }
        })
        .collect();

    pairs
}

fn part1(pairs: &Vec<Pair>, line_to_check: i64) -> usize {
    let mut total_coverage: Vec<MyRange> = vec![];

    for pair in pairs {
        let crosses_line = (pair.sensor.1 - line_to_check).abs() <= pair.distance;
        if !crosses_line {
            continue;
        }

        let sensor_coverage = get_x_within_distance(&pair.sensor, pair.distance, line_to_check);

        // we try to merge it into one of the existing ranges
        let mut merged = false;
        for (i, range) in total_coverage.iter().enumerate() {
            if let Some(merged_range) = range.merge(&sensor_coverage) {
                total_coverage.remove(i);
                total_coverage.push(merged_range);
                merged = true;
                break;
            }
        }
        if !merged {
            total_coverage.push(sensor_coverage);
        }
    }

    let mut i: isize = 0;
    while i < total_coverage.len() as isize {
        let mut j: isize = 0;
        while j < total_coverage.len() as isize {
            if i == j {
                j += 1;
                continue;
            }

            let range = &total_coverage[i as usize];
            let next_range = &total_coverage[j as usize];

            if let Some(merged_range) = range.merge(&next_range) {
                total_coverage.remove(i as usize);
                if j < i {
                    total_coverage.remove(j as usize);
                } else {
                    total_coverage.remove((j - 1) as usize);
                }
                total_coverage.push(merged_range);
                i = -1;
                break;
            }

            j += 1;
        }

        i += 1;
    }

    let total_values = total_coverage
        .iter()
        .fold(0, |acc, range| acc + (range.0.end() + 1 - range.0.start()));

    let beacons_positions_to_remove: HashSet<i64> = pairs
        .iter()
        .filter_map(|pair| {
            if pair.beacon.1 == line_to_check {
                if is_value_in_ranges(&total_coverage, pair.beacon.0) {
                    Some(pair.beacon.0)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    total_values as usize - beacons_positions_to_remove.len()
}

// get a list of x positions on line `y` within distance of current position
fn get_x_within_distance(pos: &Position, distance: i64, y: i64) -> MyRange {
    let max_distance = distance - (pos.1 - y).abs();
    let min = pos.0 - max_distance;
    let max = pos.0 + max_distance;
    MyRange(min..=max)
}

fn is_value_in_ranges(ranges: &Vec<MyRange>, value: i64) -> bool {
    for range in ranges {
        if range.0.contains(&value) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const example: &str = "
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn test_part1() {
        let pairs = parse_input(example);
        let result = part1(&pairs, 10);
        assert_eq!(result, 26);
    }
}
