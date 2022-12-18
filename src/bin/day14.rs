use adventofcode::read_file;
use std::collections::HashSet;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Position(u32, u32);

fn main() {
    let file = read_file("14").expect("cant read file");
    let positions: Vec<Vec<Position>> = parse_to_positions(&file);

    let (blocked_positions, lowest_y) = get_blocked_positions(&positions);
    let result1 = part1(&blocked_positions, lowest_y);
    println!("Part1: {result1}");

    let result2 = part2(&blocked_positions, lowest_y);
    println!("Part2: {result2}");
}

fn parse_to_positions(input: &str) -> Vec<Vec<Position>> {
    input
        .trim()
        .split('\n')
        .map(|line| {
            line.split(" -> ")
                .map(|position| {
                    let pos: [&str; 2] = position
                        .split(',')
                        .collect::<Vec<&str>>()
                        .try_into()
                        .unwrap();
                    Position(pos[0].parse().unwrap(), pos[1].parse().unwrap())
                })
                .collect()
        })
        .collect()
}

fn get_blocked_positions(positions: &Vec<Vec<Position>>) -> (HashSet<Position>, u32) {
    let mut blocked: HashSet<Position> = HashSet::new();
    let mut lowest_y = 0;

    for lines in positions {
        let mut i = 0;
        loop {
            if i > lines.len() - 2 {
                break;
            }
            let start = &lines[i];
            let end = &lines[i + 1];

            if start.0 == end.0 {
                for y in std::cmp::min(start.1, end.1)..=std::cmp::max(start.1, end.1) {
                    blocked.insert(Position(start.0, y));
                    if y > lowest_y {
                        lowest_y = y;
                    }
                }
            } else {
                for x in std::cmp::min(start.0, end.0)..=std::cmp::max(start.0, end.0) {
                    blocked.insert(Position(x, start.1));
                    if start.1 > lowest_y {
                        lowest_y = start.1;
                    }
                }
            }

            i += 1;
        }
    }

    (blocked, lowest_y)
}

fn part1(blocked_positions: &HashSet<Position>, lowest_y: u32) -> u32 {
    let mut blocked_positions = (*blocked_positions).clone();
    let mut sand_deposited: u32 = 0;

    let start = Position(500, 0);

    'main: loop {
        let mut sand_current = start.clone();

        loop {
            if sand_current.1 > lowest_y {
                break 'main;
            }

            if !blocked_positions.contains(&Position(sand_current.0, sand_current.1 + 1)) {
                sand_current.1 += 1;
                continue;
            }

            if !blocked_positions.contains(&Position(sand_current.0 - 1, sand_current.1 + 1)) {
                sand_current.0 -= 1;
                sand_current.1 += 1;
                continue;
            }
            if !blocked_positions.contains(&Position(sand_current.0 + 1, sand_current.1 + 1)) {
                sand_current.0 += 1;
                sand_current.1 += 1;
                continue;
            }

            blocked_positions.insert(sand_current);
            sand_deposited += 1;
            break;
        }
    }

    sand_deposited
}

fn part2(blocked_positions: &HashSet<Position>, lowest_y: u32) -> u32 {
    let mut blocked_positions = (*blocked_positions).clone();
    let mut sand_deposited: u32 = 0;
    let floor = lowest_y + 2;

    let start = Position(500, 0);

    'main: loop {
        let mut sand_current = start.clone();

        loop {
            if !blocked_positions.contains(&Position(sand_current.0, sand_current.1 + 1))
                && sand_current.1 < floor - 1
            {
                sand_current.1 += 1;
                continue;
            }

            if !blocked_positions.contains(&Position(sand_current.0 - 1, sand_current.1 + 1))
                && sand_current.1 < floor - 1
            {
                sand_current.0 -= 1;
                sand_current.1 += 1;
                continue;
            }
            if !blocked_positions.contains(&Position(sand_current.0 + 1, sand_current.1 + 1))
                && sand_current.1 < floor - 1
            {
                sand_current.0 += 1;
                sand_current.1 += 1;
                continue;
            }

            if sand_current.eq(&start) {
                sand_deposited += 1;
                break 'main;
            }

            blocked_positions.insert(sand_current);
            sand_deposited += 1;
            break;
        }
    }

    sand_deposited
}

#[cfg(test)]
mod tests {
    use super::*;

    const example: &str = "
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    #[test]
    fn test_part1() {
        let positions: Vec<Vec<Position>> = parse_to_positions(example);
        let (blocked_positions, lowest_y) = get_blocked_positions(&positions);
        let result = part1(&blocked_positions, lowest_y);
        assert_eq!(result, 24);
    }

    #[test]
    fn test_part2() {
        let positions: Vec<Vec<Position>> = parse_to_positions(example);
        let (blocked_positions, lowest_y) = get_blocked_positions(&positions);
        let result = part2(&blocked_positions, lowest_y);
        assert_eq!(result, 93);
    }
}
