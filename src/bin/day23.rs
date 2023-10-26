use adventofcode::read_file;
use regex::Regex;
use std::collections::BTreeSet;

fn main() {
    let day = Regex::new(r".*day(\d+).*")
        .unwrap()
        .captures(file!())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    let file = read_file(day).unwrap();
    let mut elves: BTreeSet<Position> = parse_input(&file);
    // println!("Part1: {}", run_part1(elves));
    println!("Part2: {}", run_part2(elves));
}

fn run_part1(mut elves: Elves) -> i32 {
    for i in 0..10 {
        println!("Rount: {i}");
        (elves, _) = round(i, elves);
    }

    let mut xs = vec![];
    let mut ys = vec![];
    for elf in &elves {
        xs.push(elf.0);
        ys.push(elf.1);
    }
    let x_min = xs.iter().min().unwrap();
    let x_max = xs.iter().max().unwrap();
    let y_min = ys.iter().min().unwrap();
    let y_max = ys.iter().max().unwrap();
    let total_size = (x_max - x_min + 1) * (y_max - y_min + 1);
    total_size - elves.len() as i32
}

fn run_part2(mut elves: Elves) -> i32 {
    let mut i = 0;
    loop {
        println!("Rount: {i}");
        let ret = round(i, elves);
        elves = ret.0;
        if ret.1 == false {
            return (i as i32) + 1;
        }
        i += 1;
    }
}

type Elves = BTreeSet<Position>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Position(i32, i32);

impl Position {
    fn all_around(&self) -> [Position; 8] {
        let around = [
            [-1, -1],
            [-1, 0],
            [-1, 1],
            [0, -1],
            [0, 1],
            [1, -1],
            [1, 0],
            [1, 1],
        ];
        around.map(|m| Position(self.0 + m[0], self.1 + m[1]))
    }

    fn elf_around(&self, elves: &Vec<Position>) -> bool {
        let around = self.all_around();
        for pos in around {
            if elves.contains(&pos) {
                return true;
            }
        }
        return false;
    }
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}
impl Direction {
    fn positions_to_check(&self, p: &Position) -> ([Position; 3], Position) {
        match self {
            Direction::North => (
                [
                    Position(p.0, p.1 - 1),
                    Position(p.0 - 1, p.1 - 1),
                    Position(p.0 + 1, p.1 - 1),
                ],
                Position(p.0, p.1 - 1),
            ),
            Direction::South => (
                [
                    Position(p.0, p.1 + 1),
                    Position(p.0 - 1, p.1 + 1),
                    Position(p.0 + 1, p.1 + 1),
                ],
                Position(p.0, p.1 + 1),
            ),
            Direction::West => (
                [
                    Position(p.0 - 1, p.1),
                    Position(p.0 - 1, p.1 - 1),
                    Position(p.0 - 1, p.1 + 1),
                ],
                Position(p.0 - 1, p.1),
            ),
            Direction::East => (
                [
                    Position(p.0 + 1, p.1),
                    Position(p.0 + 1, p.1 - 1),
                    Position(p.0 + 1, p.1 + 1),
                ],
                Position(p.0 + 1, p.1),
            ),
        }
    }

    fn u8_to_direction(n: u32) -> Self {
        let n = n.rem_euclid(4);
        match n {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::West,
            3 => Direction::East,
            _ => panic!("wrong n"),
        }
    }
    fn round_order(n: u32) -> [Self; 4] {
        [
            Direction::u8_to_direction(n),
            Direction::u8_to_direction(n + 1),
            Direction::u8_to_direction(n + 2),
            Direction::u8_to_direction(n + 3),
        ]
    }
}

fn round(number: u32, elves: Elves) -> (Elves, bool) {
    let elves = Vec::from_iter(elves.clone().into_iter());
    let mut moves = elves.clone();
    let directions = Direction::round_order(number);

    let mut elf_moved = false;

    // println!("{moves:?}");
    // Part1
    'move_loop: for wanted_move in &mut moves {
        // println!("===");
        // println!("MOVE: {wanted_move:?}");
        if wanted_move.elf_around(&elves) == false {
            continue 'move_loop;
        }
        elf_moved = true;
        'direction_loop: for direction in &directions {
            // println!("-> checking {direction:?}");
            let (to_check, new_pos) = direction.positions_to_check(wanted_move);
            for pos in &to_check {
                // println!("checking {pos:?}");
                // if there is an elf, check next direction
                if elves.contains(pos) {
                    // println!("there is already an elf {pos:?}");
                    continue 'direction_loop;
                }
            }
            // if no elf in sight, prepare move, and pass to the next elf
            // println!("changing {wanted_move:?} to {new_pos:?}");
            *wanted_move = new_pos;
            continue 'move_loop;
        }
    }
    // println!("{moves:?}");

    // Part2
    let mut final_positions = Elves::new();
    for (i, wanted_move) in moves.clone().iter().enumerate() {
        let slice = moves
            .iter()
            .enumerate()
            .filter(|(_i, _)| i != *_i)
            .map(|e| *e.1)
            .collect::<Vec<Position>>();
        if !slice.contains(wanted_move) {
            final_positions.insert(*wanted_move);
        } else {
            final_positions.insert(elves[i]);
        }
    }
    // println!("final_positions: {final_positions:?}");
    (final_positions, elf_moved)
}

fn parse_input(input: &str) -> Elves {
    let input = input.trim().split("\n").collect::<Vec<&str>>();
    let mut elves = Elves::new();
    for (y, line) in input.iter().enumerate() {
        for (x, tile) in line.chars().enumerate() {
            if tile == '#' {
                elves.insert(Position(x as i32, y as i32));
            }
        }
    }
    elves
}

#[cfg(test)]
mod tests {
    use super::*;

    const example1: &str = "
.....
..##.
..#..
.....
..##.
.....
";

    const example2: &str = "
..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............
";

    #[test]
    fn test_part1() {
        let elves = parse_input(example1);
        let empty = run_part1(elves);
        assert_eq!(empty, 25);

        let elves = parse_input(example2);
        let empty = run_part1(elves);
        assert_eq!(empty, 110);
    }

    #[test]
    fn test_part2() {
        let elves = parse_input(example1);
        let round = run_part2(elves);
        assert_eq!(round, 4);

        let elves = parse_input(example2);
        let round = run_part2(elves);
        assert_eq!(round, 20);
    }
}
