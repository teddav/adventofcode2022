use std::collections::HashSet;

use adventofcode::read_file;

#[derive(Debug)]
struct Action(char, i32);

fn main() -> Result<(), String> {
    let file = read_file("09").expect("cant read file");

    let actions: Vec<Action> = parse_input(&file);

    let result_part1 = part1(&actions)?;
    println!("Part1: {result_part1}");

    let result_part2 = part2(&actions)?;
    println!("Part2: {result_part2}");

    Ok(())
}

fn parse_input(input: &str) -> Vec<Action> {
    input
        .trim()
        .split('\n')
        .map(|e| {
            let split = e.split(' ').collect::<Vec<&str>>();
            Action(split[0].chars().next().unwrap(), split[1].parse().unwrap())
        })
        .collect()
}

fn part1(actions: &Vec<Action>) -> Result<usize, String> {
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();

    let mut head_position: (i32, i32) = (0, 0);
    let mut tail_position: (i32, i32) = (0, 0);

    for Action(direction, steps) in actions {
        let mut i = 0;
        while i < *steps {
            match direction {
                'R' => head_position.0 += 1,
                'L' => head_position.0 -= 1,
                'U' => head_position.1 += 1,
                'D' => head_position.1 -= 1,
                _ => return Err("not the right direction".to_string()),
            }
            i += 1;

            let x_diff = head_position.0 - tail_position.0;
            let y_diff = head_position.1 - tail_position.1;

            if x_diff.abs() > 1 || y_diff.abs() > 1 {
                tail_position.0 += if x_diff.abs() >= 1 {
                    x_diff.signum()
                } else {
                    0
                };
                tail_position.1 += if y_diff.abs() >= 1 {
                    y_diff.signum()
                } else {
                    0
                };
            }

            visited_positions.insert(tail_position);
        }
    }

    Ok(visited_positions.len())
}

fn part2(actions: &Vec<Action>) -> Result<usize, String> {
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();

    let mut head_position: (i32, i32) = (0, 0);
    let mut knots: [(i32, i32); 9] = [(0, 0); 9];

    for Action(direction, steps) in actions {
        let mut i = 0;
        while i < *steps {
            match direction {
                'R' => head_position.0 += 1,
                'L' => head_position.0 -= 1,
                'U' => head_position.1 += 1,
                'D' => head_position.1 -= 1,
                _ => return Err("not the right direction".to_string()),
            }
            i += 1;

            let mut knot_index = 0;
            while knot_index < knots.len() {
                let previous_knot = if knot_index == 0 {
                    head_position
                } else {
                    knots[knot_index - 1]
                };
                let current_knot = &mut knots[knot_index];

                let x_diff = previous_knot.0 - current_knot.0;
                let y_diff = previous_knot.1 - current_knot.1;

                if x_diff.abs() > 1 || y_diff.abs() > 1 {
                    current_knot.0 += if x_diff.abs() >= 1 {
                        x_diff.signum()
                    } else {
                        0
                    };
                    current_knot.1 += if y_diff.abs() >= 1 {
                        y_diff.signum()
                    } else {
                        0
                    };
                }

                knot_index += 1;
            }

            visited_positions.insert(knots[8]);
        }
    }

    Ok(visited_positions.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const example_input: &str = "
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    const example_input2: &str = "
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

    #[test]
    fn test_part1() {
        let actions: Vec<Action> = parse_input(example_input);
        let result = part1(&actions).unwrap();
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part2() {
        let actions: Vec<Action> = parse_input(example_input2);
        let result = part2(&actions).unwrap();
        assert_eq!(result, 36);
    }
}
