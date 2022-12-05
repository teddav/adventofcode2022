use adventofcode::read_file;
use std::collections::HashMap;

fn main() {
    let file = read_file("05").expect("cant read file");
    let (crates, moves) = parse_input(&file);

    part1(&crates, &moves);
    part2(&crates, &moves);
}

fn parse_input(input: &str) -> (HashMap<u8, Vec<char>>, Vec<Vec<&str>>) {
    let [crates_input, moves]: [&str; 2] = input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap();

    let mut crates_input: Vec<&str> = crates_input.split('\n').filter(|e| !e.is_empty()).collect();
    crates_input.pop(); // remove the numbers column since they are always in order

    let mut crates: HashMap<u8, Vec<char>> = HashMap::new();

    for line in crates_input {
        let mut count: u8 = 0;
        let mut chars = line.chars();

        while let Some(char) = chars.next() {
            if char == '[' {
                let current_crate = chars.next().unwrap();
                count += 1;

                let stack_number = ((count as f32) / 4.0).floor() as u8 + 1;

                crates
                    .entry(stack_number)
                    .or_insert(Vec::new())
                    .insert(0, current_crate);
            }
            count += 1;
        }
    }

    let moves: Vec<Vec<&str>> = moves
        .split('\n')
        .filter_map(|e| {
            if e.is_empty() {
                None
            } else {
                Some(e.split("from").map(|x| x.trim()).collect())
            }
        })
        .collect();

    (crates, moves)
}

fn part1(crates: &HashMap<u8, Vec<char>>, moves: &Vec<Vec<&str>>) -> String {
    let mut crates = crates.clone();

    for _move in moves {
        let mut move_number = _move[0].split(' ').collect::<Vec<&str>>()[1]
            .parse::<u8>()
            .unwrap();

        let [from, to]: [u8; 2] = _move[1]
            .split(" to ")
            .map(|e| e.parse().unwrap())
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        while move_number > 0 {
            let from_vec = crates.get_mut(&from).unwrap();
            if let Some(popped) = from_vec.pop() {
                crates.get_mut(&to).unwrap().push(popped);
            }

            // println!("moved {popped} from {from} to {to}");
            move_number -= 1;
        }
    }

    let mut result = String::from("");
    for i in 1..=crates.keys().len() {
        let stack = crates.get(&u8::try_from(i).unwrap()).unwrap();
        if stack.len() > 0 {
            result.push(stack[stack.len() - 1]);
        }
    }

    println!("Part1: {result}");
    result
}

fn part2(crates: &HashMap<u8, Vec<char>>, moves: &Vec<Vec<&str>>) -> String {
    let mut crates = crates.clone();

    for _move in moves {
        let mut move_number = _move[0].split(' ').collect::<Vec<&str>>()[1]
            .parse::<u8>()
            .unwrap();

        let [from, to]: [u8; 2] = _move[1]
            .split(" to ")
            .map(|e| e.parse().unwrap())
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        let from_vec = crates.get(&from).unwrap();
        let last_index = from_vec.len() - (move_number as usize);
        let to_move = from_vec[last_index..].to_vec();

        let to_vec = crates.get(&to).unwrap().clone();
        let new_vec = [to_vec, to_move].concat();

        crates.insert(from, from_vec[..last_index].to_vec().clone());
        crates.insert(to, new_vec.clone());
    }

    let mut result = String::from("");
    for i in 1..=crates.keys().len() {
        let stack = crates.get(&u8::try_from(i).unwrap()).unwrap();
        if stack.len() > 0 {
            result.push(stack[stack.len() - 1]);
        }
    }

    println!("Part2: {result}");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const example_input: &str = "
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test_part1() {
        let (crates, moves) = parse_input(example_input);
        let result = part1(&crates, &moves);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_part2() {
        let (crates, moves) = parse_input(example_input);
        let result = part2(&crates, &moves);
        assert_eq!(result, "MCD");
    }
}
