use adventofcode::read_file;

#[derive(Debug, PartialEq, Eq)]
struct Snafu(String);

impl Snafu {
    fn to_number(&self) -> i64 {
        self.0.chars().rev().enumerate().fold(0, |acc, (i, n)| {
            let n = match n {
                '-' => -1,
                '=' => -2,
                _ => n.to_digit(10).unwrap() as i64,
            };
            acc + (n * 5_i64.pow(i as u32))
        })
    }

    fn to_snafu(number: i64) -> Self {
        let mut number = number;
        let mut snafu_number = String::new();

        while number > 0 {
            let mut digit = number % 5;
            if digit > 2 {
                digit -= 5;
            }
            snafu_number.push(match digit {
                -2 => '=',
                -1 => '-',
                _ => char::from_digit((digit as u8).into(), 10).unwrap(),
            });
            number = (number - digit) / 5;
        }

        Snafu(snafu_number.chars().rev().collect())
    }
}

fn main() {
    let file = read_file("25").unwrap();
    let result1 = part1(&file);
    println!("Part1: {result1:?}");
}

fn part1(input: &str) -> Snafu {
    let sum: i64 = input
        .trim()
        .split('\n')
        .map(|n| Snafu(n.to_string()).to_number())
        .sum();

    Snafu::to_snafu(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const example: &str = "
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(example), Snafu("2=-1=0".to_string()));
    }
}
