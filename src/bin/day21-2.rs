use adventofcode::read_file;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
enum Operator {
    Add,
    Div,
    Mul,
    Sub,
    Eq,
}

impl Operator {
    fn result(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Sub => lhs - rhs,
            Operator::Mul => lhs * rhs,
            Operator::Div => lhs / rhs,
            _ => panic!("wrong operation"),
        }
    }
}

#[derive(Debug)]
enum ParsedMonkeyJob {
    Humn,
    Number(u64),
    Operation(ParsedOperation),
}

#[derive(Debug)]
struct ParsedOperation {
    operator: Operator,
    lhs: String,
    rhs: String,
}

#[derive(Debug, Clone)]
enum MonkeyJob {
    Humn,
    Number(u64),
    Operation(Operation),
}

#[derive(Debug, Clone)]
struct Operation {
    operator: Operator,
    lhs: Box<MonkeyJob>,
    rhs: Box<MonkeyJob>,
}

fn main() {
    let file = read_file("21").unwrap();
    let jobs = parse_input(&file);
    let result2 = part2(&jobs);
    println!("Part2 {result2:?}");
}

fn parse_input(input: &str) -> HashMap<String, ParsedMonkeyJob> {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let [name, operation]: [&str; 2] =
                line.split(": ").collect::<Vec<&str>>().try_into().unwrap();

            if name == "humn" {
                return (name.to_string(), ParsedMonkeyJob::Humn);
            }

            (
                name.to_string(),
                match operation {
                    s if s.parse::<u64>().is_ok() => {
                        ParsedMonkeyJob::Number(s.parse::<u64>().unwrap())
                    }
                    _ => {
                        let mut op = operation.split(' ').collect::<Vec<&str>>();
                        if name == "root" {
                            op[1] = "=";
                        }
                        ParsedMonkeyJob::Operation(ParsedOperation {
                            operator: match op[1] {
                                "+" => Operator::Add,
                                "-" => Operator::Sub,
                                "/" => Operator::Div,
                                "*" => Operator::Mul,
                                "=" => Operator::Eq,
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

fn part2(jobs: &HashMap<String, ParsedMonkeyJob>) -> Option<u64> {
    let root = &jobs[&"root".to_string()];

    if let ParsedMonkeyJob::Operation(root) = root {
        let lhs = to_monkey_job(jobs, &root.lhs);
        let rhs = to_monkey_job(jobs, &root.rhs);

        if let MonkeyJob::Number(x) = transform_operations(&lhs, &rhs) {
            return Some(x);
        }
    }
    None
}

fn transform_operations(lhs: &MonkeyJob, rhs: &MonkeyJob) -> MonkeyJob {
    match (lhs, rhs) {
        (_, MonkeyJob::Humn) => lhs.to_owned(),
        (MonkeyJob::Humn, _) => rhs.to_owned(),
        (MonkeyJob::Operation(op), MonkeyJob::Number(n)) => {
            let op_left = compute_value(op.lhs.as_ref());
            let op_right = compute_value(op.rhs.as_ref());

            if op_left.is_some() {
                match op.operator {
                    Operator::Add => {
                        transform_operations(&op.rhs, &MonkeyJob::Number(n - op_left.unwrap()))
                    }
                    Operator::Sub => {
                        transform_operations(&op.rhs, &MonkeyJob::Number(op_left.unwrap() - n))
                    }
                    Operator::Mul => {
                        transform_operations(&op.rhs, &MonkeyJob::Number(n / op_left.unwrap()))
                    }
                    Operator::Div => {
                        transform_operations(&op.rhs, &MonkeyJob::Number(op_left.unwrap() / n))
                    }
                    _ => panic!("wrong op "),
                }
            } else {
                match op.operator {
                    Operator::Add => {
                        transform_operations(&op.lhs, &MonkeyJob::Number(n - op_right.unwrap()))
                    }
                    Operator::Sub => {
                        transform_operations(&op.lhs, &MonkeyJob::Number(n + op_right.unwrap()))
                    }
                    Operator::Mul => {
                        transform_operations(&op.lhs, &MonkeyJob::Number(n / op_right.unwrap()))
                    }
                    Operator::Div => {
                        transform_operations(&op.lhs, &MonkeyJob::Number(n * op_right.unwrap()))
                    }
                    _ => panic!("wrong op "),
                }
            }
        }
        (MonkeyJob::Number(_), MonkeyJob::Number(_)) => panic!("we shouldnt have 2 numbers"),
        _ => panic!("ERROR"),
    }
}

fn to_monkey_job(jobs: &HashMap<String, ParsedMonkeyJob>, monkey: &String) -> MonkeyJob {
    match jobs.get(monkey).unwrap() {
        ParsedMonkeyJob::Number(x) => MonkeyJob::Number(*x),
        ParsedMonkeyJob::Humn => MonkeyJob::Humn,
        ParsedMonkeyJob::Operation(op) => {
            let lhs = Box::new(to_monkey_job(jobs, &op.lhs));
            let rhs = Box::new(to_monkey_job(jobs, &op.rhs));

            if let (MonkeyJob::Number(l), MonkeyJob::Number(r)) = ((*lhs).clone(), (*rhs).clone()) {
                MonkeyJob::Number(op.operator.result(l, r))
            } else {
                MonkeyJob::Operation(Operation {
                    operator: op.operator,
                    lhs,
                    rhs,
                })
            }
        }
        _ => panic!("error !"),
    }
}

fn compute_value(job: &MonkeyJob) -> Option<u64> {
    match job {
        MonkeyJob::Humn => None,
        MonkeyJob::Number(x) => Some(*x),
        MonkeyJob::Operation(op) => match op.operator {
            Operator::Add => Some(compute_value(&op.lhs)? + compute_value(&op.rhs)?),
            Operator::Sub => Some(compute_value(&op.lhs)? - compute_value(&op.rhs)?),
            Operator::Mul => Some(compute_value(&op.lhs)? * compute_value(&op.rhs)?),
            Operator::Div => Some(compute_value(&op.lhs)? / compute_value(&op.rhs)?),
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
    fn test_part2() {
        let jobs = parse_input(example);
        assert_eq!(part2(&jobs), Some(301));
    }
}
