use adventofcode::read_file;

#[derive(Debug)]
enum Instruction {
    AddX(i32),
    Noop,
    Err,
}

#[derive(Debug)]
struct State {
    x_register: i32,
    cycle: i32,
}

fn main() -> Result<(), String> {
    let file = read_file("10").expect("cant read file");
    let instructions = parse_instructions(&file);

    let result1 = part1(&instructions)?;
    println!("Part1: {result1}");

    let result2 = part2(&instructions)?;
    println!(
        "Part2: {:#?}",
        result2
            .chars()
            .collect::<Vec<char>>()
            .chunks(40)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>()
    );

    Ok(())
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let input: Vec<&str> = input.trim().split('\n').collect();

    input
        .iter()
        .map(|line| {
            let split = line.split(' ').collect::<Vec<&str>>();
            if split[0] == "addx" {
                Instruction::AddX(split[1].parse().unwrap())
            } else if split[0] == "noop" {
                Instruction::Noop
            } else {
                Instruction::Err
            }
        })
        .collect()
}

fn part1(instructions: &Vec<Instruction>) -> Result<i32, &str> {
    let mut state: State = State {
        x_register: 1,
        cycle: 0,
    };

    let mut signal_strengths = 0;

    for instruction in instructions {
        match instruction {
            Instruction::AddX(value) => {
                state.cycle += 1;
                signal_strengths += check_state(&state);
                state.cycle += 1;
                signal_strengths += check_state(&state);
                state.x_register += value;
            }
            Instruction::Noop => {
                state.cycle += 1;
                signal_strengths += check_state(&state);
            }
            _ => return Err("Wrong instruction"),
        };
    }

    Ok(signal_strengths)
}

fn check_state(state: &State) -> i32 {
    if state.cycle % 40 == 20 {
        state.cycle * state.x_register
    } else {
        0
    }
}

fn part2(instructions: &Vec<Instruction>) -> Result<String, &str> {
    let mut state: State = State {
        x_register: 1,
        cycle: 0,
    };

    let mut drawing = String::from("");

    for instruction in instructions {
        match instruction {
            Instruction::AddX(value) => {
                drawing.push(check_sprite(&state));
                state.cycle += 1;
                drawing.push(check_sprite(&state));
                state.cycle += 1;
                state.x_register += value;
            }
            Instruction::Noop => {
                drawing.push(check_sprite(&state));
                state.cycle += 1;
            }
            _ => return Err("Wrong instruction"),
        };
    }

    Ok(drawing)
}

fn check_sprite(state: &State) -> char {
    let sprite = [state.x_register - 1, state.x_register, state.x_register + 1];
    if sprite.contains(&(state.cycle % 40)) {
        '#'
    } else {
        '.'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const example_input: &str = "
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[test]
    fn test_part1() {
        let instructions = parse_instructions(example_input);
        let result = part1(&instructions).unwrap();
        assert_eq!(result, 13140);
    }

    #[test]
    fn test_part2() {
        let instructions = parse_instructions(example_input);
        let result = part2(&instructions).unwrap();
        assert_eq!(result, "##..##..##..##..##..##..##..##..##..##..###...###...###...###...###...###...###.####....####....####....####....####....#####.....#####.....#####.....#####.....######......######......######......###########.......#######.......#######.....".to_string());
    }
}
