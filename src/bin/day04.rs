use adventofcode::read_file;
use std::ops::RangeInclusive;

struct MyRange(RangeInclusive<u16>);

impl MyRange {
    fn contains_range(&self, other_range: &Self) -> bool {
        if self.0.contains(&other_range.0.clone().min().unwrap())
            && self.0.contains(&other_range.0.clone().max().unwrap())
        {
            true
        } else {
            false
        }
    }

    fn overlaps(&self, other_range: &Self) -> bool {
        if self.0.contains(&other_range.0.clone().min().unwrap())
            || self.0.contains(&other_range.0.clone().max().unwrap())
        {
            true
        } else {
            false
        }
    }
}

fn main() {
    let file = read_file("04").expect("cant read file");
    let input: Vec<&str> = file.trim().split('\n').collect();

    let (total_part1, total_part2) = run(&input).unwrap();

    println!("Part1: {total_part1}");
    println!("Part2: {total_part2}");
}

fn run(input: &Vec<&str>) -> Result<(u16, u16), &'static str> {
    let mut total_part1: u16 = 0;
    let mut total_part2: u16 = 0;

    for pair in input {
        let pair: Vec<Vec<u16>> = pair
            .split(',')
            .map(|p| {
                p.split('-')
                    .map(|v| v.parse::<u16>().unwrap())
                    .collect::<Vec<u16>>()
            })
            .collect();

        let [pair1, pair2] = [&pair[0], &pair[1]];

        let range1 = MyRange(pair1[0]..=pair1[1]);
        let range2 = MyRange(pair2[0]..=pair2[1]);

        if range1.contains_range(&range2) || range2.contains_range(&range1) {
            total_part1 += 1;
        }
        if range1.overlaps(&range2) || range2.overlaps(&range1) {
            total_part2 += 1;
        }
    }

    Ok((total_part1, total_part2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example_input: &str = "
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
        ";
        let input: Vec<&str> = example_input.trim().split('\n').map(|v| v.trim()).collect();
        let (part1, part2) = run(&input).unwrap();

        assert_eq!(part1, 2);
        assert_eq!(part2, 4);
    }
}
