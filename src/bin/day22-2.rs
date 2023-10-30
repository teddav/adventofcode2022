use adventofcode::read_file;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum Path {
    Steps(usize),
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}
use Direction::*;
impl Direction {
    fn change_direction(&self, shift: Path) -> Self {
        let mut current = *self as i8;
        current += match shift {
            Path::Left => -1,
            Path::Right => 1,
            _ => panic!("wrong direction"),
        };
        Direction::i8_to_enum(current.rem_euclid(4))
    }

    fn i8_to_enum(n: i8) -> Self {
        match n {
            0 => Self::Right,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Up,
            _ => panic!("wrong direction"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Nil,
}

#[derive(Debug, Clone, Copy)]
struct Position(usize, usize);

type Face<const FACE_SIZE: usize> = [[Tile; FACE_SIZE]; FACE_SIZE];

#[derive(Debug)]
struct CubeFace<const FACE_SIZE: usize> {
    face: Face<FACE_SIZE>,
    start_position: Position,
}

#[derive(Debug)]
struct Cube<const FACE_SIZE: usize> {
    current_id: usize,
    faces: HashMap<usize, CubeFace<FACE_SIZE>>,
}

impl<const FACE_SIZE: usize> Cube<FACE_SIZE> {
    fn new() -> Self {
        Self {
            current_id: 0,
            faces: HashMap::new(),
        }
    }

    fn add_faces(&mut self, faces: Vec<Face<FACE_SIZE>>, start_positions: Vec<Position>) {
        for (i, f) in faces.into_iter().enumerate() {
            self.add_face(f, start_positions[i]);
        }
    }

    fn add_face(&mut self, face: Face<FACE_SIZE>, start_position: Position) {
        self.faces.insert(
            self.current_id,
            CubeFace {
                face,
                start_position,
            },
        );
        self.current_id += 1;
    }

    fn get_face(&self, n: usize) -> Face<FACE_SIZE> {
        self.faces.get(&n).unwrap().face
    }
}

fn main() {
    let day = Regex::new(r".*day(\d+).*")
        .unwrap()
        .captures(file!())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    let file = read_file(day).unwrap();

    let (cube, path) = parse_input::<50>(&file);
    let result = part2::<50>(cube, path, switch_face_main);
    println!("Result: {result}");
}

fn parse_input<const FACE_SIZE: usize>(input: &str) -> (Cube<FACE_SIZE>, Vec<Path>) {
    let [input, _path]: [&str; 2] = input
        .trim_start_matches('\n')
        .trim_end_matches('\n')
        .split("\n\n")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap();

    let mut path: Vec<Path> = vec![];
    let mut iter_path = _path.chars().peekable();
    while let Some(c) = iter_path.next() {
        path.push(match c {
            'R' => Path::Right,
            'L' => Path::Left,
            c if c.is_numeric() => {
                let mut temp = String::from(c);

                while let Some(next_c) = iter_path.peek() {
                    if next_c.is_numeric() {
                        temp += &next_c.to_string();
                        iter_path.next();
                    } else {
                        break;
                    }
                }
                Path::Steps(temp.parse().unwrap())
            }
            _ => panic!("wrong direction"),
        });
    }

    let lines = input.split('\n').collect::<Vec<&str>>();
    let mut cube = Cube::<FACE_SIZE>::new();
    let mut faces = vec![];
    let mut start_positions = vec![];

    for (line_index, line) in lines.iter().enumerate() {
        if line_index.rem_euclid(FACE_SIZE) == 0 {
            cube.add_faces(faces, start_positions);
            faces = vec![];
            start_positions = vec![];
        }

        let mut start_index = -1;
        for (i, c) in line.char_indices() {
            if c != '.' && c != '#' {
                continue;
            }
            if start_index == -1 {
                start_index = i as isize;
            }
            let face_id = (i - (start_index as usize)) / FACE_SIZE;
            if faces.len() <= face_id {
                faces.push([[Tile::Nil; FACE_SIZE]; FACE_SIZE]);
                start_positions.push(Position(i + 1, line_index + 1));
            }

            let char_index = (i - (start_index as usize)) % FACE_SIZE;
            let line_index_face = line_index % FACE_SIZE;

            faces[face_id][line_index_face][char_index] = match c {
                '.' => Tile::Empty,
                '#' => Tile::Wall,
                _ => panic!("wrong char {c}"),
            };
        }
    }
    cube.add_faces(faces, start_positions);
    (cube, path)
}

fn part2<const FACE_SIZE: usize>(
    cube: Cube<FACE_SIZE>,
    path: Vec<Path>,
    switch_face: TSwitchFace,
) -> usize {
    let mut direction = Direction::Right;
    let mut face_id = 0;
    let mut position = Position(0, 0);

    for instruction in path {
        match instruction {
            Path::Steps(n) => {
                (face_id, position, direction) =
                    walk(n, &cube, face_id, position, direction, switch_face)
            }
            _ => direction = direction.change_direction(instruction),
        };
    }

    let face = cube.faces.get(&face_id).unwrap().start_position;
    let x = face.0 + position.0;
    let y = face.1 + position.1;
    1000 * y + 4 * x + (direction as usize)
}

fn walk<const FACE_SIZE: usize>(
    mut steps: usize,
    cube: &Cube<FACE_SIZE>,
    mut current_face_id: usize,
    mut current_position: Position,
    mut direction: Direction,
    switch_face: TSwitchFace,
) -> (usize, Position, Direction) {
    let max_index = FACE_SIZE - 1;

    while steps > 0 {
        let mut temp_next_position = current_position;
        let mut temp_face_id = current_face_id;
        let mut temp_direction = direction;

        match direction {
            Right => {
                if current_position.0 == max_index {
                    (temp_face_id, temp_direction, temp_next_position) =
                        switch_face(current_face_id, direction, current_position, max_index);
                } else {
                    temp_next_position.0 += 1;
                }
            }
            Down => {
                if current_position.1 == max_index {
                    (temp_face_id, temp_direction, temp_next_position) =
                        switch_face(current_face_id, direction, current_position, max_index);
                } else {
                    temp_next_position.1 += 1;
                }
            }
            Left => {
                if current_position.0 == 0 {
                    (temp_face_id, temp_direction, temp_next_position) =
                        switch_face(current_face_id, direction, current_position, max_index);
                } else {
                    temp_next_position.0 -= 1;
                }
            }
            Up => {
                if current_position.1 == 0 {
                    (temp_face_id, temp_direction, temp_next_position) =
                        switch_face(current_face_id, direction, current_position, max_index);
                } else {
                    temp_next_position.1 -= 1;
                }
            }
        }

        let temp_face = cube.get_face(temp_face_id);
        match temp_face[temp_next_position.1][temp_next_position.0] {
            Tile::Wall => break,
            Tile::Empty => {
                current_face_id = temp_face_id;
                current_position = temp_next_position;
                direction = temp_direction;
            }
            _ => panic!("cannot be other than wall or empty"),
        }

        steps -= 1;
    }
    (current_face_id, current_position, direction)
}

// https://www.edumedia-sciences.com/en/media/412-nets-of-a-cube
type TSwitchFace = fn(usize, Direction, Position, usize) -> (usize, Direction, Position);

fn switch_face_main(
    face_id: usize,
    direction: Direction,
    position: Position,
    max_index: usize,
) -> (usize, Direction, Position) {
    // https://www.edumedia-sciences.com/en/media/412-nets-of-a-cube
    match face_id {
        0 => match direction {
            Right => (1, Left, Position(0, position.1)),
            Down => (2, Down, Position(position.0, 0)),
            Left => (4, Up, Position(max_index - position.1, max_index)),
            Up => (5, Up, Position(position.0, max_index)),
        },
        1 => match direction {
            Right => (3, Left, Position(max_index, max_index - position.1)),
            Down => (2, Left, Position(max_index, position.0)),
            Left => (0, Left, Position(max_index, position.1)),
            Up => (5, Left, Position(max_index, max_index - position.0)),
        },
        2 => match direction {
            Right => (1, Up, Position(position.1, max_index)),
            Down => (3, Down, Position(0, position.1)),
            Left => (4, Right, Position(0, max_index - position.1)),
            Up => (0, Up, Position(position.0, max_index)),
        },
        3 => match direction {
            Right => (1, Left, Position(max_index, max_index - position.1)),
            Down => (5, Down, Position(position.0, 0)),
            Left => (4, Down, Position(position.0, 0)),
            Up => (2, Up, Position(position.0, max_index)),
        },
        4 => match direction {
            Right => (5, Right, Position(0, position.1)),
            Down => (0, Right, Position(0, max_index - position.0)),
            Left => (2, Right, Position(0, max_index - position.1)),
            Up => (3, Right, Position(0, position.0)),
        },
        5 => match direction {
            Right => (1, Down, Position(max_index - position.1, 0)),
            Down => (0, Down, Position(position.0, 0)),
            Left => (4, Left, Position(max_index, position.1)),
            Up => (3, Up, Position(position.0, max_index)),
        },
        _ => panic!("error"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const example: &str = "
        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";

    fn switch_face_example(
        face_id: usize,
        direction: Direction,
        position: Position,
        max_index: usize,
    ) -> (usize, Direction, Position) {
        match face_id {
            0 => match direction {
                Right => (5, Left, Position(max_index, max_index - position.1)),
                Down => (3, Down, Position(position.0, 0)),
                Left => (2, Down, Position(position.1, 0)),
                Up => (1, Down, Position(max_index - position.0, 0)),
            },
            1 => match direction {
                Right => (2, Right, Position(0, position.1)),
                Down => (4, Up, Position(max_index - position.0, max_index)),
                Left => (5, Up, Position(max_index - position.1, max_index)),
                Up => (0, Down, Position(position.0, max_index)),
            },
            2 => match direction {
                Right => (3, Right, Position(0, position.1)),
                Down => (4, Right, Position(0, max_index - position.0)),
                Left => (1, Left, Position(max_index, position.1)),
                Up => (0, Right, Position(0, position.0)),
            },
            3 => match direction {
                Right => (5, Down, Position(max_index - position.1, 0)),
                Down => (4, Down, Position(position.0, 0)),
                Left => (2, Left, Position(max_index, position.1)),
                Up => (0, Up, Position(position.0, max_index)),
            },
            4 => match direction {
                Right => (5, Right, Position(0, position.1)),
                Down => (1, Up, Position(max_index - position.0, max_index)),
                Left => (2, Up, Position(max_index - position.1, max_index)),
                Up => (3, Up, Position(position.0, max_index)),
            },
            5 => match direction {
                Right => (0, Left, Position(max_index, max_index - position.1)),
                Down => (1, Right, Position(0, max_index - position.0)),
                Left => (4, Left, Position(max_index, position.1)),
                Up => (3, Left, Position(max_index, max_index - position.0)),
            },
            _ => panic!("error"),
        }
    }

    #[test]
    fn test_part2() {
        let (cube, path) = parse_input::<4>(&example);
        let result = part2::<4>(cube, path, switch_face_example);
        assert_eq!(result, 5031);
    }
}
