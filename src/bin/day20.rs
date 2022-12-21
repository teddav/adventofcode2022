use adventofcode::read_file;

#[derive(Debug, Clone, Copy)]
struct EncryptedNumber {
    index: usize,
    value: i64,
}

fn main() {
    let file = read_file("20").unwrap();
    let input = parse_input(&file);

    let result1 = part1(&input);
    println!("Part1 {}", result1);

    let input = parse_input2(&file);
    let result2 = part2(&input);
    println!("Part2 {:?}", result2);
}

fn parse_input(input: &str) -> Vec<EncryptedNumber> {
    input
        .trim()
        .split('\n')
        .enumerate()
        .map(|(i, n)| EncryptedNumber {
            index: i,
            value: n.parse().unwrap(),
        })
        .collect()
}

fn parse_input2(input: &str) -> Vec<EncryptedNumber> {
    input
        .trim()
        .split('\n')
        .enumerate()
        .map(|(i, n)| EncryptedNumber {
            index: i,
            value: n.parse::<i64>().unwrap() * 811589153,
        })
        .collect()
}

fn part1(input: &Vec<EncryptedNumber>) -> i64 {
    let mut input = input.clone();

    let mut i = 0;
    while i < input.len() {
        let current_index = input.iter().position(|n| n.index == i).unwrap();
        let element = input.get(current_index).unwrap().clone();
        let to_move = element.value;

        let new_index = (current_index as i64 + to_move).rem_euclid((input.len() - 1) as i64);

        input.remove(current_index);
        input.insert(new_index.try_into().unwrap(), element);

        i += 1;
    }

    let index_0 = input.iter().position(|n| n.value == 0).unwrap();

    vec![1000, 2000, 3000]
        .iter()
        .map(|i| input.get((index_0 + i) % input.len()).unwrap().value)
        .sum()
}

fn part2(input: &Vec<EncryptedNumber>) -> i64 {
    let mut input = input.clone();

    for _ in 0..10 {
        let mut i = 0;
        while i < input.len() {
            let current_index = input.iter().position(|n| n.index == i).unwrap();
            let element = input.get(current_index).unwrap().clone();
            let to_move = element.value;

            let new_index = (current_index as i64 + to_move).rem_euclid((input.len() - 1) as i64);

            input.remove(current_index);
            input.insert(new_index.try_into().unwrap(), element);

            i += 1;
        }
    }

    let index_0 = input.iter().position(|n| n.value == 0).unwrap();

    vec![1000, 2000, 3000]
        .iter()
        .map(|i| input.get((index_0 + i) % input.len()).unwrap().value)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const example: &str = "
1
2
-3
3
-2
0
4
";

    #[test]
    fn test_part1() {
        let input = parse_input(example);
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_part2() {
        let input = parse_input2(example);
        assert_eq!(part2(&input), 1623178306);
    }
}
