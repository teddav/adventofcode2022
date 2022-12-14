use std::cmp::Ordering;

use adventofcode::read_file;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Number(u8),
    List(Vec<Packet>),
}

impl Packet {
    fn add_to_list(&mut self, to_add: Packet) {
        if let Packet::List(list) = self {
            list.push(to_add);
        }
    }
}

fn main() {
    let file = read_file("13").expect("cant read file");
    let input: Vec<&str> = file.trim().split("\n\n").collect();

    let pairs = parse_input_part1(&input);
    let result1 = part1(&pairs);
    println!("Part1: {result1}");

    let packets = parse_input_part2(&input);
    let result2 = part2(&packets);
    println!("Part2: {result2}");
}

fn parse_input_part1(input: &Vec<&str>) -> Vec<[Vec<char>; 2]> {
    input
        .iter()
        .map(|pair| {
            pair.split('\n')
                .map(|packet| packet.chars().collect())
                .collect::<Vec<Vec<char>>>()
                .try_into()
                .unwrap()
        })
        .collect()
}

fn parse_input_part2(input: &Vec<&str>) -> Vec<Packet> {
    input
        .iter()
        .map(|pair| {
            pair.split('\n')
                .map(|packet| parse_packet(&packet.chars().collect()))
                .collect::<Vec<Packet>>()
        })
        .flatten()
        .collect()
}

fn parse_packet(packet: &Vec<char>) -> Packet {
    let mut index = 0;
    parse_list(packet, &mut index)
}

fn parse_list(list: &Vec<char>, index: &mut usize) -> Packet {
    let mut current_list = Packet::List(vec![]);
    *index += 1;

    let mut temp_number = String::from("");

    loop {
        match list[*index] {
            '[' => {
                current_list.add_to_list(parse_list(list, index));
            }
            ']' => {
                if temp_number.len() > 0 {
                    current_list.add_to_list(Packet::Number(temp_number.parse().unwrap()));
                }
                return current_list;
            }
            ',' => {
                if temp_number.len() > 0 {
                    current_list.add_to_list(Packet::Number(temp_number.parse().unwrap()));
                }
                temp_number.clear();
            }
            _ => temp_number.push(list[*index]),
        };
        *index += 1;
    }
}

fn compare_packets(left_packet: &Packet, right_packet: &Packet) -> Ordering {
    match (left_packet, right_packet) {
        (Packet::Number(left), Packet::Number(right)) => left.cmp(right),
        (Packet::Number(left), Packet::List(_)) => {
            let as_list = Packet::List(vec![Packet::Number(*left)]);
            compare_packets(&as_list, right_packet)
        }
        (Packet::List(left), Packet::List(right)) => {
            for (i, item) in left.iter().enumerate() {
                if i < right.len() {
                    let comparison = compare_packets(item, &right[i]);
                    if comparison != Ordering::Equal {
                        return comparison;
                    }
                } else {
                    return Ordering::Greater;
                }
            }
            left.len().cmp(&right.len())
        }
        (Packet::List(_), Packet::Number(right)) => {
            let as_list = Packet::List(vec![Packet::Number(*right)]);
            compare_packets(left_packet, &as_list)
        }
        _ => Ordering::Equal,
    }
}

fn part1(pairs: &Vec<[Vec<char>; 2]>) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter(|(_, [l, r])| {
            let left = parse_packet(l);
            let right = parse_packet(r);
            compare_packets(&left, &right) == Ordering::Less
        })
        .fold(0, |sum, (i, _)| sum + i + 1)
}

fn part2(packets: &Vec<Packet>) -> usize {
    let divider1 = parse_packet(&"[[2]]".chars().collect());
    let divider2 = parse_packet(&"[[6]]".chars().collect());

    let mut sorted_packets = (*packets).clone();
    sorted_packets.push(divider1.clone());
    sorted_packets.push(divider2.clone());
    sorted_packets.sort_by(|a, b| compare_packets(a, b));

    let mut result = 1;
    for (i, packet) in sorted_packets.iter().enumerate() {
        if packet.eq(&divider1) || packet.eq(&divider2) {
            result *= i + 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const example: &str = "
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn test_part1() {
        let input: Vec<&str> = example.trim().split("\n\n").collect();
        let pairs = parse_input_part1(&input);
        let result1 = part1(&pairs);
        assert_eq!(result1, 13);
    }

    #[test]
    fn test_part2() {
        let input: Vec<&str> = example.trim().split("\n\n").collect();
        let packets = parse_input_part2(&input);
        let result2 = part2(&packets);
        assert_eq!(result2, 140);
    }
}
