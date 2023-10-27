use adventofcode::read_file;
use regex::Regex;
use std::collections::VecDeque;
use std::time::Instant;

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

    let start = Instant::now();
    let result = part2(game);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    println!("RESULT: {result}");
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

    fn to_arrow(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
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

fn part2(game: Game) -> usize {
    let Game {
        current: start,
        max_x,
        max_y,
        blizzards,
        end,
        directions,
    } = game;

    let mut paths_taken = VecDeque::from([(start, 0)]);
    let all_blizzards = get_all_blizzards_positions(blizzards, &directions, max_x, max_y);
    let all_blizzards_count = all_blizzards.len();
    println!("all_blizzards_count: {all_blizzards_count}");
    let mut already_seen: Vec<(Position, usize)> = vec![];

    let mut goal = end;
    let mut finish = false;

    'main: loop {
        if let Some((current_position, moves_count)) = paths_taken.pop_front() {
            let count = (moves_count + 1) % all_blizzards_count;
            let current_blizzards = &all_blizzards[count];
            // print_map(&current_blizzards, &directions, (max_x, max_y));

            if !already_seen.contains(&(current_position, count)) {
                already_seen.push((current_position, count));

                for pos in next_positions(
                    &current_position,
                    &current_blizzards,
                    (max_x, max_y),
                    &end,
                    Some(&goal),
                ) {
                    if pos == goal {
                        println!("Reached goal !! {moves_count}");
                        paths_taken = VecDeque::from([(goal, moves_count + 1)]);
                        already_seen = vec![];

                        if goal == end && finish {
                            break 'main moves_count + 1;
                        }
                        goal = if goal == end { start } else { end };
                        finish = true;
                        continue 'main;
                    }

                    paths_taken.push_back((pos, moves_count + 1));
                }
            }
        } else {
            println!("no more paths...");
            break 0;
        }
    }
}

fn part1(game: Game) -> usize {
    let Game {
        current: start,
        max_x,
        max_y,
        blizzards,
        end,
        directions,
    } = game;

    let mut paths_taken = VecDeque::from([(start, 0)]);
    let all_blizzards = get_all_blizzards_positions(blizzards, &directions, max_x, max_y);
    let all_blizzards_count = all_blizzards.len();
    println!("all_blizzards_count: {all_blizzards_count}");
    let mut already_seen: Vec<(Position, usize)> = vec![];

    'main: loop {
        if let Some((current_position, moves_count)) = paths_taken.pop_front() {
            let count = (moves_count + 1) % all_blizzards_count;
            let current_blizzards = &all_blizzards[count];
            // print_map(&current_blizzards, &directions, (max_x, max_y));

            if !already_seen.contains(&(current_position, count)) {
                already_seen.push((current_position, count));

                for pos in next_positions(
                    &current_position,
                    &current_blizzards,
                    (max_x, max_y),
                    &end,
                    None,
                ) {
                    if pos == end {
                        break 'main (moves_count + 1);
                    }

                    paths_taken.push_back((pos, moves_count + 1));
                }
            }
        } else {
            println!("no more paths...");
            break 0;
        }
    }
}

fn print_map(blizzards: &Vec<Position>, directions: &Vec<Direction>, max: (u8, u8)) {
    let mut lines: Vec<Vec<char>> = vec![];
    for _ in 0..max.1 {
        let s = "."
            .repeat((max.0 + 1) as usize)
            .chars()
            .collect::<Vec<char>>();
        lines.push(s);
    }
    for (i, c) in blizzards.iter().enumerate() {
        let value = lines[(c.1 - 1) as usize][(c.0) as usize];
        if value == '.' {
            lines[(c.1 - 1) as usize][(c.0) as usize] = directions[i].to_arrow();
        } else {
            if let Ok(value) = value.to_string().parse::<u8>() {
                lines[(c.1 - 1) as usize][(c.0) as usize] = (value + 1) as char;
            } else {
                lines[(c.1 - 1) as usize][(c.0) as usize] = '2';
            }
        }
    }
    for line in lines {
        println!("{}", line.iter().collect::<String>());
    }
    println!("{}", "-".repeat((max.0 + 1) as usize));
}

fn get_all_blizzards_positions(
    start: Vec<Position>,
    directions: &Vec<Direction>,
    max_x: u8,
    max_y: u8,
) -> Vec<Vec<Position>> {
    let mut all_blizzards = vec![start.clone()];
    let mut current = start.clone();

    loop {
        for b in 0..current.len() {
            directions[b].shift(&mut current[b], (max_x, max_y));
        }
        if current == start {
            break;
        }
        all_blizzards.push(current.clone());
    }
    // for b in all_blizzards.clone() {
    //     print_map(&b, directions, (max_x, max_y));
    // }
    all_blizzards
}

fn next_positions(
    current: &Position,
    blizzards: &Vec<Position>,
    max: (u8, u8),
    end: &Position,
    goal: Option<&Position>,
) -> Vec<Position> {
    if current == &Position(0, 0) {
        return vec![Position(0, 1)];
    }
    if current == end {
        return vec![Position(current.0, current.1 - 1)];
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

    if let Some(goal) = goal {
        if goal == &Position(current.0, current.1 + 1) {
            positions.push(Position(current.0, current.1 + 1));
        } else if goal == &Position(current.0, current.1 - 1) {
            positions.push(Position(current.0, current.1 - 1));
        }
    }

    // we also have the option not to move and to stay at the same spot
    positions.push(current.clone());

    positions
        .iter()
        .filter(|pos| !blizzards.contains(pos))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

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
        // println!("game: {game:#?}");

        let start = Instant::now();
        let result = part1(game);
        let duration = start.elapsed();
        println!("Time elapsed 1: {:?}", duration);

        println!("RESULT 1: {result}");
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part2() {
        let game = parse_input(&example);

        let start = Instant::now();
        let result = part2(game);
        let duration = start.elapsed();
        println!("Time elapsed 2: {:?}", duration);

        println!("RESULT 2: {result}");
        assert_eq!(result, 54);
    }
}
