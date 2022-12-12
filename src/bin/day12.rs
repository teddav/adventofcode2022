use adventofcode::read_file;
use pathfinding::prelude::{astar, bfs, dijkstra};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, PartialOrd)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
}

fn main() {
    let file = read_file("12").expect("cant read file");
    let input: Vec<&str> = file.trim().split('\n').collect();

    let (map, start, end) = parse_input(&input);
    // part1_backtrack(&input, &start, &end);
    part1(&map, &start, &end);

    let (map, start, end) = parse_input_part2(&input);
    let result = part2(&map, &start, &end);
    println!("Part2: {result}");
}

fn parse_input(input: &Vec<&str>) -> (Vec<Vec<u32>>, Position, Position) {
    let mut start = Position::new(0, 0);
    let mut end = Position::new(0, 0);

    let into_digits: Vec<Vec<u32>> = input
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = Position::new(x, y);
                        1
                    } else if c == 'E' {
                        end = Position::new(x, y);
                        26
                    } else {
                        c as u32 - 96
                    }
                })
                .collect()
        })
        .collect();

    // println!("{into_digits:#?}");
    // println!("start: {start:?} / end: {end:?}");

    (into_digits, start, end)
}

fn parse_input_part2(input: &Vec<&str>) -> (Vec<Vec<u32>>, Vec<Position>, Position) {
    let mut start = vec![];
    let mut end = Position::new(0, 0);

    let into_digits: Vec<Vec<u32>> = input
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' || c == 'a' {
                        start.push(Position::new(x, y));
                        1
                    } else if c == 'E' {
                        end = Position::new(x, y);
                        26
                    } else {
                        c as u32 - 96
                    }
                })
                .collect()
        })
        .collect();

    // println!("{into_digits:#?}");
    // println!("start: {start:?} / end: {end:?}");

    (into_digits, start, end)
}

fn get_next_possible_positions(
    input: &Vec<Vec<u32>>,
    current_position: &Position,
) -> Vec<(Position, u32)> {
    let value = input[current_position.y][current_position.x];
    let Position { x, y } = *current_position;

    // right, left, top, down
    let next_step = vec![
        (
            input.get(y).and_then(|col| col.get(x + 1)),
            Position::new(x + 1, y),
        ),
        if x > 0 {
            (
                input.get(y).and_then(|col| col.get(x - 1)),
                Position::new(x - 1, y),
            )
        } else {
            (None, Position::new(0, 0))
        },
        if y > 0 {
            (
                input.get(y - 1).and_then(|col| col.get(x)),
                Position::new(x, y - 1),
            )
        } else {
            (None, Position::new(0, 0))
        },
        (
            input.get(y + 1).and_then(|col| col.get(x)),
            Position::new(x, y + 1),
        ),
    ];

    let next_step: Vec<(Position, u32)> = next_step
        .iter()
        .filter(|_s| {
            if _s.0.is_some() {
                let s = _s.0.unwrap();
                if *s <= value + 1 {
                    return true;
                }
            }
            false
        })
        .map(|s| (s.1, *s.0.unwrap()))
        .collect();

    next_step
}

fn backtracking(
    input: &Vec<Vec<u32>>,
    start: &Position,
    end: &Position,
    path: &Vec<Position>,
) -> Option<Vec<Position>> {
    if start == end {
        return Some(path.clone());
    }

    let next_positions = get_next_possible_positions(input, start)
        .iter()
        .map(|v| v.0)
        .collect::<Vec<Position>>();

    for pos in next_positions {
        if !path.contains(&pos) {
            let mut new_path = path.clone();
            new_path.push(pos);
            match backtracking(input, &pos, end, &new_path) {
                Some(solution) => return Some(solution),
                None => continue,
            }
        }
    }

    None
}

fn part1_backtrack(input: &Vec<Vec<u32>>, start: &Position, end: &Position) -> usize {
    if let Some(result) = backtracking(input, start, end, &vec![*start]) {
        println!("Part1 backtracking    {}", result.len());
        result.len() - 1
    } else {
        println!("cant find with backtracking");
        0
    }
}

fn part1(input: &Vec<Vec<u32>>, start: &Position, end: &Position) -> usize {
    if let Some(path) = dijkstra(
        start,
        |p| get_next_possible_positions(input, p),
        |p| *p == *end,
    ) {
        println!("Part1 dijkstra        {}", path.0.len() - 1);
    } else {
        println!("cant dijkstra");
    }

    if let Some(path) = astar(
        start,
        |p| get_next_possible_positions(input, p),
        |p| ((p.x as i32 - end.x as i32).abs() + (p.y as i32 - end.y as i32).abs()) as u32,
        |p| *p == *end,
    ) {
        println!("Part1 astar           {}", path.0.len() - 1);
    } else {
        println!("cant astar");
    }

    if let Some(path) = bfs(
        start,
        |p| {
            get_next_possible_positions(input, p)
                .iter()
                .map(|v| v.0)
                .collect::<Vec<Position>>()
        },
        |p| *p == *end,
    ) {
        println!("Part1 bfs             {}", path.len() - 1);
        return path.len() - 1;
    } else {
        println!("cant bfs");
    }

    0
}

fn part2(input: &Vec<Vec<u32>>, starts: &Vec<Position>, end: &Position) -> usize {
    let mut paths: Vec<usize> = vec![];

    for start in starts {
        if let Some(path) = bfs(
            start,
            |p| {
                get_next_possible_positions(input, p)
                    .iter()
                    .map(|v| v.0)
                    .collect::<Vec<Position>>()
            },
            |p| *p == *end,
        ) {
            paths.push(path.len() - 1);
        }
    }

    *paths.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const example1: &str = "
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    const example2: &str = "
SabcdefghijklmnopqrstuvwxyzE
";

    #[test]
    fn test_part1() {
        let input: Vec<&str> = example1.trim().split('\n').collect();
        let (input, start, end) = parse_input(&input);
        let result = part1(&input, &start, &end);
        assert_eq!(result, 31);
    }

    #[test]
    fn test_part1_example2() {
        let input: Vec<&str> = example2.trim().split('\n').collect();
        let (input, start, end) = parse_input(&input);
        let result = part1(&input, &start, &end);
        assert_eq!(result, 27);
    }

    #[test]
    fn test_part1_backtrack() {
        let input: Vec<&str> = example2.trim().split('\n').collect();
        let (input, start, end) = parse_input(&input);
        let result = part1_backtrack(&input, &start, &end);
        assert_eq!(result, 27);
    }

    #[test]
    fn test_part2() {
        let input: Vec<&str> = example1.trim().split('\n').collect();
        let (input, start, end) = parse_input_part2(&input);
        let result = part2(&input, &start, &end);
        assert_eq!(result, 29);
    }
}
