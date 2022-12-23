use std::collections::HashMap;

use adventofcode::read_file;

#[derive(Debug)]
enum MonkeyJob {
    Number(u64),
    Operation(Operation),
}

#[derive(Debug)]
enum Operator {
    Add,
    Div,
    Mul,
    Sub,
}

#[derive(Debug)]
struct Operation {
    operator: Operator,
    lhs: String,
    rhs: String,
}

fn main() {
    let file = read_file("21").unwrap();
    let jobs = parse_input(&file);

    let result1 = part1(&jobs);
    println!("Part1 {result1}");
}

fn parse_input(input: &str) -> HashMap<String, MonkeyJob> {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let [name, operation]: [&str; 2] =
                line.split(": ").collect::<Vec<&str>>().try_into().unwrap();
            (
                name.to_string(),
                match operation {
                    s if s.parse::<u64>().is_ok() => MonkeyJob::Number(s.parse::<u64>().unwrap()),
                    _ => {
                        let op = operation.split(' ').collect::<Vec<&str>>();
                        MonkeyJob::Operation(Operation {
                            operator: match op[1] {
                                "+" => Operator::Add,
                                "-" => Operator::Sub,
                                "/" => Operator::Div,
                                "*" => Operator::Mul,
                                _ => panic!("wrong operator"),
                            },
                            lhs: op[0].to_string(),
                            rhs: op[2].to_string(),
                        })
                    }
                },
            )
        })
        .collect()
}

fn part1(jobs: &HashMap<String, MonkeyJob>) -> u64 {
    monkey_number(&"root".to_string(), jobs)
}

fn monkey_number(monkey: &String, jobs: &HashMap<String, MonkeyJob>) -> u64 {
    match jobs.get(monkey).unwrap() {
        MonkeyJob::Number(x) => *x,
        MonkeyJob::Operation(op) => match op.operator {
            Operator::Add => monkey_number(&op.lhs, jobs) + monkey_number(&op.rhs, jobs),
            Operator::Sub => monkey_number(&op.lhs, jobs) - monkey_number(&op.rhs, jobs),
            Operator::Mul => monkey_number(&op.lhs, jobs) * monkey_number(&op.rhs, jobs),
            Operator::Div => monkey_number(&op.lhs, jobs) / monkey_number(&op.rhs, jobs),
            _ => panic!("wrong operation"),
        },
        _ => panic!("wrong monkey job"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const example: &str = "
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

    #[test]
    fn test_part1() {
        let jobs = parse_input(example);
        assert_eq!(part1(&jobs), 152);
    }
}
