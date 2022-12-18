use adventofcode::read_file;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Cube(i32, i32, i32);

impl Cube {
    fn neighbors(&self) -> Vec<Cube> {
        vec![
            (1, 0, 0),
            (0, 1, 0),
            (0, 0, 1),
            (-1, 0, 0),
            (0, -1, 0),
            (0, 0, -1),
        ]
        .iter()
        .map(|neighbor| {
            Cube(
                self.0 + neighbor.0,
                self.1 + neighbor.1,
                self.2 + neighbor.2,
            )
        })
        .collect()
    }
}

fn main() {
    let file = read_file("18").unwrap();
    let cubes = parse_input(&file);

    let result1 = part1(&cubes);
    println!("Part1 {result1}");

    let result2 = part2(&cubes);
    println!("Part2 {result2}");
}

fn parse_input(input: &str) -> Vec<Cube> {
    input
        .trim()
        .split('\n')
        .map(|cube| {
            let pos = cube
                .split(',')
                .map(|i| i.parse().unwrap())
                .collect::<Vec<i32>>();
            Cube(pos[0], pos[1], pos[2])
        })
        .collect()
}

fn part1(cubes: &Vec<Cube>) -> usize {
    let mut total = cubes.len() * 6;
    for cube in cubes {
        for neighbor in cube.neighbors() {
            if cubes.contains(&neighbor) {
                total -= 1;
            }
        }
    }
    total
}

fn part2(cubes: &Vec<Cube>) -> usize {
    let x_min = cubes.iter().map(|c| c.0).min().unwrap();
    let x_max = cubes.iter().map(|c| c.0).max().unwrap();
    let y_min = cubes.iter().map(|c| c.1).min().unwrap();
    let y_max = cubes.iter().map(|c| c.1).max().unwrap();
    let z_min = cubes.iter().map(|c| c.2).min().unwrap();
    let z_max = cubes.iter().map(|c| c.2).max().unwrap();

    let x_range = x_min - 1..=x_max + 1;
    let y_range = y_min - 1..=y_max + 1;
    let z_range = z_min - 1..=z_max + 1;

    let mut water: HashSet<Cube> = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(Cube(x_min - 1, y_min - 1, z_min - 1));

    while let Some(cube) = queue.pop_front() {
        if x_range.contains(&cube.0) && y_range.contains(&cube.1) && z_range.contains(&cube.2) {
            for neighbor in cube.neighbors() {
                if !cubes.contains(&neighbor) && !water.contains(&neighbor) {
                    water.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    cubes
        .iter()
        .map(|cube| {
            cube.neighbors()
                .iter()
                .filter(|c| water.contains(c))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const example: &str = "
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

    #[test]
    fn test_part1() {
        let cubes = parse_input(example);
        let result = part1(&cubes);
        assert_eq!(result, 64);
    }

    #[test]
    fn test_part2() {
        let cubes = parse_input(example);
        let result = part2(&cubes);
        assert_eq!(result, 58);
    }
}
