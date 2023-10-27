use adventofcode::read_file;
use regex::Regex;
use std::collections::VecDeque;

fn main() {
    let day = Regex::new(r".*day(\d+).*")
        .unwrap()
        .captures(file!())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    let file = read_file(day).unwrap();
    let game = parse_input(&file);
    part1(game);
}

#[derive(Debug)]
struct Game {
    current: Position,
    end: Position,
    max_x: u8,
    max_y: u8,
    directions: Vec<Direction>,
    blizzards: Vec<Position>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Position(u8, u8);

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_arrow(c: char) -> Self {
        match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            'v' => Direction::Down,
            '^' => Direction::Up,
            _ => panic!("wrong direction"),
        }
    }

    fn shift(&self, pos: &mut Position, max: (u8, u8)) {
        match self {
            Direction::Up => {
                pos.1 = if pos.1 == 1 { max.1 } else { pos.1 - 1 };
            }
            Direction::Down => {
                pos.1 = if pos.1 == max.1 { 1 } else { pos.1 + 1 };
            }
            Direction::Left => {
                pos.0 = if pos.0 == 0 { max.0 } else { pos.0 - 1 };
            }
            Direction::Right => {
                pos.0 = if pos.0 == max.0 { 0 } else { pos.0 + 1 };
            }
        };
    }
}

fn parse_input(input: &str) -> Game {
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let start = lines
        .first()
        .unwrap()
        .chars()
        .position(|elt| elt == '.')
        .unwrap() as u8;
    let last = lines
        .last()
        .unwrap()
        .chars()
        .position(|elt| elt == '.')
        .unwrap() as u8;
    let start = Position(start - 1, 0);
    let end = Position(last - 1, (lines.len() - 1) as u8);

    let max_x = (lines.first().unwrap().len() - 3) as u8;
    let max_y = (lines.len() - 2) as u8;

    let mut directions: Vec<Direction> = vec![];
    let mut blizzards: Vec<Position> = vec![];
    for (y, line) in lines[1..(lines.len() - 1)].iter().enumerate() {
        for (x, elt) in line.trim_matches('#').char_indices() {
            if elt != '.' {
                directions.push(Direction::from_arrow(elt));
                blizzards.push(Position(x as u8, (y + 1) as u8));
            }
        }
    }

    Game {
        current: start,
        end,
        max_x,
        max_y,
        directions,
        blizzards,
    }
}

fn part1(game: Game) -> u32 {
    let Game {
        current: start,
        max_x,
        max_y,
        blizzards,
        end,
        directions,
    } = game;
    let mut paths_taken = VecDeque::from([(start, blizzards, 0)]);

    let moves = 'main: loop {
        if let Some((current_position, blizzards, moves_count)) = paths_taken.pop_front() {
            let mut blizzards = blizzards;
            for i in 0..blizzards.len() {
                directions[i].shift(&mut blizzards[i], (max_x, max_y));
            }

            for pos in next_positions(&current_position, &blizzards, (max_x, max_y), &end) {
                if pos == end {
                    break 'main (moves_count + 1);
                }

                paths_taken.push_back((pos, blizzards.clone(), moves_count + 1));
            }
        }
    };

    moves
}

fn next_positions(
    current: &Position,
    blizzards: &Vec<Position>,
    max: (u8, u8),
    end: &Position,
) -> Vec<Position> {
    if current == &Position(0, 0) {
        return vec![Position(0, 1)];
    }

    let mut positions = vec![];
    // left
    if current.0 > 0 {
        positions.push(Position(current.0 - 1, current.1));
    }
    // right
    if current.0 < max.0 {
        positions.push(Position(current.0 + 1, current.1));
    }
    // up
    if current.1 > 1 {
        positions.push(Position(current.0, current.1 - 1));
    }

    // down
    // here we need to check if we can reach `end` position
    if current.1 < max.1 || end == &Position(current.0, current.1 + 1) {
        positions.push(Position(current.0, current.1 + 1));
    }
    // let positions: Vec<Position> = positions
    //     .iter()
    //     .filter(|pos| !blizzards.contains(pos))
    //     .cloned()
    //     .collect();
    for blizzard in blizzards {
        positions.retain(|p| p != blizzard);
        if positions.len() == 0 {
            break;
        }
    }

    if positions.len() == 0 {
        vec![current.clone()]
    } else {
        positions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    const example: &str = "
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
    ";

    #[test]
    fn test_part1() {
        let game = parse_input(&example);

        let start = Instant::now();
        let result = part1(game);
        let duration = start.elapsed();
        println!("Time elapsed: {:?}", duration);

        println!("RESULT: {result}");
        assert_eq!(result, 18)
    }
}
