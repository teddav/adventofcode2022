use adventofcode::read_file;
use regex::Regex;

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

#[derive(Debug)]
enum Path {
    Steps(u32),
    Left,
    Right,
}

const DIRECTIONS: [char; 4] = ['R', 'D', 'L', 'U'];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Nil,
}

#[derive(Debug)]
struct Position(usize, usize);

fn main() {
    let day = Regex::new(r".*day(\d+).*")
        .unwrap()
        .captures(file!())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    let file = read_file(day).unwrap();

    let (map, path) = parse_input(&example);

    let result1 = part1(&map, &path);
    println!("Part1: {result1}");
}

fn part1(map: &Vec<Vec<Tile>>, path: &Vec<Path>) -> usize {
    let mut direction: i32 = 0; // ['R', 'D', 'L', 'U']
    let mut position = Position(map[0].iter().position(|e| *e != Tile::Nil).unwrap(), 0);

    for instruction in path {
        match *instruction {
            Path::Left => direction = (direction - 1).rem_euclid(4),
            Path::Right => direction = (direction + 1).rem_euclid(4),
            Path::Steps(n) => {
                position = walk(map, &position, DIRECTIONS[direction as usize], n);
            }
        };
    }

    1000 * (position.1 + 1) + 4 * (position.0 + 1) + direction as usize
}

fn walk(map: &Vec<Vec<Tile>>, start: &Position, direction: char, mut steps: u32) -> Position {
    let (line, mut index) = match direction {
        'U' | 'D' => (
            map.iter().map(|line| line[start.0]).collect::<Vec<Tile>>(),
            start.1 as i32,
        ),
        'R' | 'L' => (map[start.1].clone(), start.0 as i32),
        _ => panic!("wrong direction"),
    };

    let step = match direction {
        'U' => -1,
        'L' => -1,
        'D' => 1,
        'R' => 1,
        _ => panic!("wrong direction"),
    };

    while steps > 0 {
        let mut next_index = (index + step).rem_euclid(line.len() as i32);

        while line[next_index as usize] == Tile::Nil {
            next_index = (next_index + step).rem_euclid(line.len() as i32);
        }

        index = match line[next_index as usize] {
            Tile::Empty => next_index,
            Tile::Wall => break,
            _ => panic!("cant be Nil"),
        };
        steps -= 1;
    }

    match direction {
        'U' | 'D' => Position(start.0, index as usize),
        'R' | 'L' => Position(index as usize, start.1),
        _ => panic!("wrong direction"),
    }
}

fn parse_input(input: &str) -> (Vec<Vec<Tile>>, Vec<Path>) {
    let [input, _path]: [&str; 2] = input
        .trim_start_matches('\n')
        .trim_end_matches('\n')
        .split("\n\n")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap();

    let _map = input.split('\n').collect::<Vec<&str>>();
    let max_length = _map
        .iter()
        .fold(0, |acc, v| if v.len() > acc { v.len() } else { acc });

    let map: Vec<Vec<Tile>> = _map
        .iter()
        .map(|line| {
            let mut _line: Vec<Tile> = line
                .chars()
                .map(|c| match c {
                    ' ' => Tile::Nil,
                    '.' => Tile::Empty,
                    '#' => Tile::Wall,
                    _ => panic!("wrong tile"),
                })
                .collect();
            while _line.len() < max_length {
                _line.push(Tile::Nil);
            }
            _line
        })
        .collect();

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

    (map, path)
}
