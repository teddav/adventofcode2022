use adventofcode::read_file;
use std::collections::HashSet;

fn main() {
    let file = read_file("06").expect("cant read file");
    println!("Part1: {}", run(&file, 4));
    println!("Part2: {}", run(&file, 14));
}

fn run(buffer: &str, marker_size: usize) -> usize {
    let buffer: Vec<char> = buffer.chars().collect();
    let mut index = marker_size - 1;
    let len = buffer.len();
    while index < len {
        let slice = &buffer[index - (marker_size - 1)..=index];
        let hashset: HashSet<&char> = HashSet::from_iter(slice);
        if hashset.len() == marker_size {
            return index + 1;
        }
        index += 1;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const example_input: &str = "
mjqjpqmgbljsphdztnvjfqwrcgsmlb
bvwbjplbgvbhsrlpgdmjqwftvncz
nppdvjthqldpwncqszvftbrmjlhg
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw
";

    #[test]
    fn test_part1() {
        let result: Vec<usize> = example_input
            .split('\n')
            .filter(|buffer| !buffer.is_empty())
            .map(|buffer| run(buffer, 4))
            .collect();

        assert_eq!(result, [7, 5, 6, 10, 11]);
    }

    #[test]
    fn test_part2() {
        let result: Vec<usize> = example_input
            .split('\n')
            .filter(|buffer| !buffer.is_empty())
            .map(|buffer| run(buffer, 14))
            .collect();

        assert_eq!(result, [19, 23, 23, 29, 26]);
    }
}
