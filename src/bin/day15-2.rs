use adventofcode::read_file;
use regex::Regex;
use std::ops::RangeInclusive;

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

#[derive(Debug)]
struct Limits {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
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

    let result = part2(
        &pairs,
        &Limits {
            x_min: 0,
            x_max: 4000000,
            y_min: 0,
            y_max: 4000000,
        },
    );
    println!("Part2 {result:?}");
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

fn part2(pairs: &Vec<Pair>, limits: &Limits) -> Option<i64> {
    for line in limits.y_min..=limits.y_max {
        if let Some(pos) = check_line(pairs, limits, line) {
            println!("Part2: {pos:?}");
            return Some(pos.0 * 4000000 + pos.1);
        }
    }
    None
}

fn check_line(pairs: &Vec<Pair>, limits: &Limits, line_to_check: i64) -> Option<Position> {
    let mut total_coverage: Vec<MyRange> = vec![];

    for pair in pairs {
        let crosses_line = (pair.sensor.1 - line_to_check).abs() <= pair.distance;
        if !crosses_line {
            continue;
        }

        let sensor_coverage = get_x_within_distance(
            &pair.sensor,
            pair.distance,
            line_to_check,
            limits.x_min,
            limits.x_max,
        );

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

    if total_values == limits.x_max + 1 - limits.x_min {
        None
    } else {
        total_coverage.sort_by(|a, b| a.0.start().cmp(b.0.start()));
        let x = total_coverage[0].0.end() + 1;
        Some(Position(x, line_to_check))
    }
}

// get a list of x positions on line `y` within distance of current position
fn get_x_within_distance(pos: &Position, distance: i64, y: i64, min_x: i64, max_x: i64) -> MyRange {
    let max_distance = distance - (pos.1 - y).abs();
    let min = std::cmp::max(pos.0 - max_distance, min_x);
    let max = std::cmp::min(pos.0 + max_distance, max_x);
    MyRange(min..=max)
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
    fn test_part2() {
        let pairs = parse_input(example);
        let result = part2(
            &pairs,
            &Limits {
                x_min: 0,
                x_max: 20,
                y_min: 0,
                y_max: 20,
            },
        );
        assert_eq!(result.unwrap(), 56000011);
    }
}
