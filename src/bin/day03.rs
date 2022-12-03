use adventofcode::read_file;
use std::collections::HashSet;

const INPUT: &str = "
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

fn main() -> Result<(), &'static str> {
    // let contents = INPUT.trim().split('\n').collect::<Vec<&str>>();

    let file = read_file("03").expect("cant read file");
    let contents = file.split('\n').collect::<Vec<&str>>();

    part1(&contents);
    part2(&contents);

    Ok(())
}

fn item_to_priority(c: char) -> Result<u8, &'static str> {
    match c {
        'a'..='z' => Ok((c as u8) - 96),
        'A'..='Z' => Ok((c as u8) - 38),
        _ => Err("doesn't match any char"),
    }
}

fn part1(contents: &Vec<&str>) {
    let mut total: u16 = 0;

    for s in contents {
        let splitted = s.split_at(s.len() / 2);
        let first_half = splitted
            .0
            .chars()
            .filter_map(|c| item_to_priority(c).ok())
            .collect::<Vec<u8>>();
        let second_half = splitted
            .1
            .chars()
            .filter_map(|c| item_to_priority(c).ok())
            .collect::<Vec<u8>>();
        // println!("first: /{:?}/ second: /{:?}/", first_half, second_half);

        let first_half_set: HashSet<u8> = HashSet::from_iter(first_half.iter().cloned());
        let second_half_set: HashSet<u8> = HashSet::from_iter(second_half.iter().cloned());

        let mut intersection = first_half_set.intersection(&second_half_set);

        if let Some(result) = intersection.nth(0) {
            total += *result as u16;
        }
    }

    println!("Part1: {total}");
}

fn part2(contents: &Vec<&str>) {
    let mut total: u16 = 0;

    let chunks: Vec<&[&str]> = contents.chunks(3).collect();
    for chunk in chunks {
        let mut hash_chunks: Vec<HashSet<char>> = chunk
            .iter()
            .map(|c| HashSet::from_iter(c.chars()))
            .collect();

        let mut result = hash_chunks.pop().unwrap();

        // https://stackoverflow.com/questions/65175088/is-there-a-zero-copy-way-to-find-the-intersection-of-an-arbitrary-number-of-sets
        result.retain(|item| hash_chunks.iter().all(|set| set.contains(item)));
        let result = result.into_iter().collect::<Vec<char>>();

        if let Ok(value) = item_to_priority(result[0]) {
            total += value as u16;
        }
    }
    println!("Part2: {total}");
}
