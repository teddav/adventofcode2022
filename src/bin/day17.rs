use adventofcode::read_file;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position(u64, u64);

type JetPosition = u64;
type RockId = u64;

#[derive(Debug)]
struct Cache {
    top_y: u64,
    fallen_pieces: u64,
}

#[derive(Debug, Clone)]
struct Shape(Vec<Position>);
impl Shape {
    fn new(index: u64, y: u64) -> Self {
        match index {
            0 => Shape(vec![
                Position(2, y),
                Position(3, y),
                Position(4, y),
                Position(5, y),
            ]),
            1 => Shape(vec![
                Position(2, y + 1),
                Position(3, y),
                Position(3, y + 1),
                Position(3, y + 2),
                Position(4, y + 1),
            ]),
            2 => Shape(vec![
                Position(2, y),
                Position(3, y),
                Position(4, y),
                Position(4, y + 1),
                Position(4, y + 2),
            ]),
            3 => Shape(vec![
                Position(2, y),
                Position(2, y + 1),
                Position(2, y + 2),
                Position(2, y + 3),
            ]),
            4 => Shape(vec![
                Position(2, y),
                Position(2, y + 1),
                Position(3, y),
                Position(3, y + 1),
            ]),
            _ => panic!("wrong index for shape"),
        }
    }

    fn is_collision_next(&self, direction: char, other: &HashSet<Position>) -> bool {
        let mut mutable = (*self).clone();
        let has_moved = match direction {
            'l' => mutable.move_left(),
            'r' => mutable.move_right(),
            'd' => mutable.move_down(),
            _ => panic!("is_collision_next: wrong direction"),
        };

        !has_moved || !HashSet::from_iter(mutable.0).is_disjoint(other)
    }

    fn move_left(&mut self) -> bool {
        if self.0.iter().find(|p| p.0 == 0).is_some() {
            return false;
        }
        for Position(x, y) in &mut self.0 {
            *x -= 1;
        }
        true
    }

    fn move_right(&mut self) -> bool {
        if self.0.iter().find(|p| p.0 == 6).is_some() {
            return false;
        }
        for Position(x, y) in &mut self.0 {
            *x += 1;
        }
        true
    }

    fn move_down(&mut self) -> bool {
        if self.0.iter().find(|p| p.1 == 1).is_some() {
            return false;
        }
        for Position(x, y) in &mut self.0 {
            *y -= 1;
        }
        true
    }
}

fn main() {
    let file = read_file("17").unwrap();
    let result1 = run(&file, 2022);
    println!("Part1: {result1:?}");

    let result2 = run(&file, 1000000000000);
    println!("Part2: {result2:?}");
}

fn run(input: &str, n_pieces: u64) -> u64 {
    let mut occupied_positions: HashSet<Position> = HashSet::from_iter(vec![
        Position(0, 0),
        Position(1, 0),
        Position(2, 0),
        Position(3, 0),
        Position(4, 0),
        Position(5, 0),
        Position(6, 0),
    ]);
    let mut top_y = occupied_positions
        .iter()
        .fold(0, |acc, x| if x.1 > acc { x.1 } else { acc });

    let mut i = 0;
    let mut y = 0;

    let mut cache: HashMap<(JetPosition, RockId, Vec<Position>), Cache> = HashMap::new();
    let mut cached = false;

    let mut height_from_repeat = 0;

    while y < n_pieces {
        let mut piece = Shape::new(y % 5, top_y + 4);
        loop {
            let direction = input.chars().nth(i % input.len()).unwrap();
            i += 1;

            match direction {
                '<' => {
                    if !piece.is_collision_next('l', &occupied_positions) {
                        piece.move_left();
                    }
                }
                '>' => {
                    if !piece.is_collision_next('r', &occupied_positions) {
                        piece.move_right();
                    }
                }
                _ => panic!("wrong move"),
            };

            if piece.is_collision_next('d', &occupied_positions) {
                break;
            }
            piece.move_down();
        }

        let _pos: HashSet<Position> = HashSet::from_iter(piece.clone().0);
        occupied_positions.extend(&_pos);

        top_y = occupied_positions
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap()
            .1;

        let latest_rows = get_latest_rows(&occupied_positions, 30, top_y);

        if let Some(in_cache) = cache.get(&((i % input.len()) as u64, y % 5, latest_rows.clone())) {
            if !cached {
                let repeats = (n_pieces - y) / (y - in_cache.fallen_pieces);
                let remaining = (n_pieces - y) % (y - in_cache.fallen_pieces);

                height_from_repeat = repeats * (top_y - in_cache.top_y);
                y = n_pieces - remaining;
                cached = true;
            }
        } else {
            cache.insert(
                ((i % input.len()) as u64, y % 5, latest_rows),
                Cache {
                    top_y: top_y,
                    fallen_pieces: y,
                },
            );
        }

        y += 1;
    }

    top_y + height_from_repeat
}

fn get_latest_rows(
    occupied_positions: &HashSet<Position>,
    n_rows: u64,
    top_row: u64,
) -> Vec<Position> {
    let mut latest_rows: Vec<Position> = vec![];

    let min_y = if top_row < n_rows {
        0
    } else {
        top_row - n_rows
    };

    for y in min_y..=top_row {
        for x in 0..=6 {
            if occupied_positions.contains(&Position(x, y)) {
                latest_rows.push(Position(x, y - min_y));
            }
        }
    }

    latest_rows.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    latest_rows
}

#[cfg(test)]
mod tests {
    use super::*;

    const example: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part1() {
        assert_eq!(run(example, 2022), 3068);
    }

    #[test]
    fn test_part2() {
        assert_eq!(run(example, 1000000000000), 1514285714288);
    }
}
