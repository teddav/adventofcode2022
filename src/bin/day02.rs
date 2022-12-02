use adventofcode::read_file;
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
enum Error {
    CharError,
}

fn main() -> io::Result<()> {
    let contents = read_file("02")?;
    let input = parse_input(contents.as_str());
    // println!("{:?}", input);

    part1(&input);
    part2(&input);

    Ok(())
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|v| v.chars().filter(|c| *c != ' ').collect::<Vec<char>>())
        .collect()
}

fn part1(input: &Vec<Vec<char>>) -> Result<(), Error> {
    let mut points: HashMap<char, u8> = HashMap::new();
    points.insert('X', 1); // Rock
    points.insert('Y', 2); // Paper
    points.insert('Z', 3); // Scissors

    let mut total_score: u32 = 0;
    for round in input {
        let score = round_output1(round[0], round[1], &points)?;
        total_score += score as u32;
    }

    println!("total score:  {total_score}");

    Ok(())
}

fn round_output1(opponent: char, me: char, points: &HashMap<char, u8>) -> Result<u8, Error> {
    let score = match opponent {
        'A' => match me {
            'X' => 3,
            'Y' => 6,
            'Z' => 0,
            _ => return Err(Error::CharError),
        },
        'B' => match me {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => return Err(Error::CharError),
        },
        'C' => match me {
            'X' => 6,
            'Y' => 0,
            'Z' => 3,
            _ => return Err(Error::CharError),
        },
        _ => return Err(Error::CharError),
    };
    Ok(score + points.get(&me).unwrap())
}

fn part2(input: &Vec<Vec<char>>) -> Result<(), Error> {
    let mut points: HashMap<char, u8> = HashMap::new();
    points.insert('A', 1); // Rock
    points.insert('B', 2); // Paper
    points.insert('C', 3); // Scissors

    let mut total_score: u32 = 0;
    for round in input {
        let score = round_output2(round[0], round[1], &points)?;
        total_score += score as u32;
    }

    println!("total score 2:  {total_score}");

    Ok(())
}

fn round_output2(opponent: char, me: char, points: &HashMap<char, u8>) -> Result<u8, Error> {
    let choice = match opponent {
        'A' => match me {
            'X' => 'C',
            'Y' => 'A',
            'Z' => 'B',
            _ => return Err(Error::CharError),
        },
        'B' => match me {
            'X' => 'A',
            'Y' => 'B',
            'Z' => 'C',
            _ => return Err(Error::CharError),
        },
        'C' => match me {
            'X' => 'B',
            'Y' => 'C',
            'Z' => 'A',
            _ => return Err(Error::CharError),
        },
        _ => return Err(Error::CharError),
    };

    let score: u8 = match me {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => 0,
    };
    Ok(score + points.get(&choice).unwrap())
}
