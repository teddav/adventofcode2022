use adventofcode::read_file;
use std::collections::HashSet;

const example: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position(u64, u64);

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
    let result1 = part1(&file);
    println!("Part1: {result1:?}");
}

fn part1(input: &str) -> u64 {
    let mut top: HashSet<Position> = HashSet::from_iter(vec![
        Position(0, 0),
        Position(1, 0),
        Position(2, 0),
        Position(3, 0),
        Position(4, 0),
        Position(5, 0),
        Position(6, 0),
    ]);
    let mut top_y = top
        .iter()
        .fold(0, |acc, x| if x.1 > acc { x.1 } else { acc });

    let mut i = 0;

    for y in 0..2022 {
        let mut piece = Shape::new(y % 5, top_y + 4);
        loop {
            let direction = input.chars().nth(i % input.len()).unwrap();
            i += 1;

            match direction {
                '<' => {
                    if !piece.is_collision_next('l', &top) {
                        piece.move_left();
                    }
                }
                '>' => {
                    if !piece.is_collision_next('r', &top) {
                        piece.move_right();
                    }
                }
                _ => panic!("wrong move"),
            };

            if piece.is_collision_next('d', &top) {
                break;
            }
            piece.move_down();
        }

        let _pos: HashSet<Position> = HashSet::from_iter(piece.clone().0);
        top.extend(&_pos);

        top_y = top.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    }
    top_y
}

#[cfg(test)]
mod tests {
    use super::*;

    const example: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part1() {
        assert_eq!(part1(example), 3068);
    }
}
