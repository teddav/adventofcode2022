use adventofcode::read_file;
use regex::Regex;
use std::fmt;

struct Monkey {
    n: u8,
    items: Vec<f64>,
    operation: Box<dyn Fn(f64) -> f64>,
    test: Box<dyn Fn(f64) -> u8>,
    n_inspect_items: u64,
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Monkey {} : {:?} (inspections: {})",
            self.n, self.items, self.n_inspect_items
        )
    }
}

fn main() -> Result<(), ()> {
    let file = read_file("11").expect("fail to read file");
    let input: Vec<&str> = file.trim().split("\n\n").collect();

    let (mut monkeys, _) = parse_input(&input).expect("error parsing file");
    let result1 = part1(&mut monkeys).unwrap();
    println!("Part1: {result1}");

    let (mut monkeys, lcm) = parse_input(&input).expect("error parsing file");
    let result2 = part2(&mut monkeys, lcm).unwrap();
    println!("Part2: {result2}");

    Ok(())
}

fn parse_input(input: &Vec<&str>) -> Result<(Vec<Monkey>, f64), Box<dyn std::error::Error>> {
    let re = Regex::new(
        r"Monkey (\d+):\n.*items:([0-9 ,]+)\n.*Operation: new = old (.*)\n.*Test: divisible by (\d+)\n.*true: throw to monkey (\d+)\n.*false: throw to monkey (\d+)",
    )?;

    let mut monkeys: Vec<Monkey> = vec![];
    let mut lcm: f64 = 1.0;

    for monkey_info in input {
        let captures = re.captures(monkey_info).unwrap();

        // println!("{monkey_info:#?}");
        // println!("{captures:?}");
        let n: u8 = captures.get(1).unwrap().as_str().parse().unwrap();
        let items: Vec<f64> = captures
            .get(2)
            .unwrap()
            .as_str()
            .split(',')
            .map(|i| i.trim().parse().unwrap())
            .collect();
        let operation: Vec<String> = captures
            .get(3)
            .unwrap()
            .as_str()
            .split(' ')
            .map(|s| s.to_string())
            .collect();
        let test: f64 = captures.get(4).unwrap().as_str().parse().unwrap();
        let test_true: u8 = captures.get(5).unwrap().as_str().parse().unwrap();
        let test_false: u8 = captures.get(6).unwrap().as_str().parse().unwrap();

        // println!(
        //     "{} {:?} {:?} {} {} {}",
        //     n, items, operation, test, test_true, test_false
        // );

        lcm *= test;

        monkeys.push(Monkey {
            n,
            items,
            operation: Box::new(move |a| match operation[0].as_str() {
                "*" => {
                    a * if operation[1] == "old" {
                        a
                    } else {
                        operation[1].parse::<f64>().unwrap()
                    }
                }
                "+" => {
                    a + if operation[1] == "old" {
                        a
                    } else {
                        operation[1].parse::<f64>().unwrap()
                    }
                }
                _ => a,
            }),
            test: Box::new(move |a| {
                if a % test == 0.0 {
                    test_true
                } else {
                    test_false
                }
            }),
            n_inspect_items: 0,
        });
    }

    Ok((monkeys, lcm))
}

fn part1(monkeys: &mut Vec<Monkey>) -> Result<u64, Box<dyn std::error::Error>> {
    let mut monkeys = monkeys;

    for _ in 0..20 {
        for n in 0..monkeys.len() {
            while monkeys[n].items.len() > 0 {
                monkeys[n].n_inspect_items += 1;

                let item = monkeys[n].items[0];
                monkeys[n].items = monkeys[n].items[1..].to_vec();

                let worry = ((monkeys[n].operation)(item) / 3.0).floor();
                let to_monkey = (monkeys[n].test)(worry);
                monkeys[to_monkey as usize].items.push(worry);

                // println!("{item} -> {worry} from {} to {to_monkey}", monkeys[n].n);
            }
        }
    }

    println!("{monkeys:#?}");
    let mut inspected_items = monkeys
        .iter()
        .map(|m| m.n_inspect_items)
        .collect::<Vec<u64>>();
    inspected_items.sort_by(|a, b| b.cmp(a));

    Ok(inspected_items[0] * inspected_items[1])
}

fn part2(monkeys: &mut Vec<Monkey>, lcm: f64) -> Result<u64, Box<dyn std::error::Error>> {
    let mut monkeys = monkeys;

    for _ in 0..10000 {
        for n in 0..monkeys.len() {
            while monkeys[n].items.len() > 0 {
                monkeys[n].n_inspect_items += 1;

                let item = monkeys[n].items[0];
                monkeys[n].items = monkeys[n].items[1..].to_vec();

                let worry = (monkeys[n].operation)(item).floor() % lcm;
                let to_monkey = (monkeys[n].test)(worry);
                monkeys[to_monkey as usize].items.push(worry);

                // println!("{item} -> {worry} from {} to {to_monkey}", monkeys[n].n);
            }
        }
    }

    println!("{monkeys:#?}");
    let mut inspected_items = monkeys
        .iter()
        .map(|m| m.n_inspect_items)
        .collect::<Vec<u64>>();
    inspected_items.sort_by(|a, b| b.cmp(a));

    Ok(inspected_items[0] * inspected_items[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    const example: &str = "
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn test_part1() {
        let input: Vec<&str> = example.trim().split("\n\n").collect();
        let (mut monkeys, _) = parse_input(&input).expect("error parsing file");
        let result = part1(&mut monkeys).unwrap();
        assert_eq!(result, 10605);
    }

    #[test]
    fn test_part2() {
        let input: Vec<&str> = example.trim().split("\n\n").collect();
        let (mut monkeys, lcm) = parse_input(&input).expect("error parsing file");
        let result = part2(&mut monkeys, lcm).unwrap();
        assert_eq!(result, 2713310158);
    }
}
