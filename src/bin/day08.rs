use adventofcode::read_file;

fn main() {
    let file = read_file("08").expect("cant read file");

    let input = file
        .trim()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let result_part1 = part1(&input);
    println!("Part1: {result_part1}");

    let result_part2 = part2(&input);
    println!("Part2: {result_part2}");
}

fn part1(input: &Vec<Vec<u32>>) -> u32 {
    let mut total_visible = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let line = &input[i];
            let column: Vec<u32> = input.iter().map(|l| l[j]).collect();

            let tree_height = input[i][j];
            let visible = is_visible(tree_height, line, &column, j, i);

            if visible {
                total_visible += 1;
            }
        }
    }

    total_visible
}

fn is_visible(
    tree_height: u32,
    line: &Vec<u32>,
    column: &Vec<u32>,
    line_index: usize,
    column_index: usize,
) -> bool {
    if line_index == 0 || line_index == line.len() - 1 {
        return true;
    }
    if column_index == 0 || column_index == column.len() - 1 {
        return true;
    }

    let left = line[..line_index].iter().max().unwrap();
    let right = line[line_index + 1..].iter().max().unwrap();
    let top = column[..column_index].iter().max().unwrap();
    let down = column[column_index + 1..].iter().max().unwrap();

    if tree_height > *left || tree_height > *right || tree_height > *top || tree_height > *down {
        return true;
    }

    false
}

fn part2(input: &Vec<Vec<u32>>) -> u32 {
    let mut max_scenic_score = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let line = &input[i];
            let column: Vec<u32> = input.iter().map(|l| l[j]).collect();

            let tree_height = input[i][j];
            let score = scenic_score(tree_height, line, &column, j, i);

            if score > max_scenic_score {
                max_scenic_score = score;
            }
        }
    }

    max_scenic_score
}

fn scenic_score(
    tree_height: u32,
    line: &Vec<u32>,
    column: &Vec<u32>,
    line_index: usize,
    column_index: usize,
) -> u32 {
    let left: Vec<u32> = line[..line_index].iter().copied().rev().collect();
    let right = line[line_index + 1..].to_vec();
    let top: Vec<u32> = column[..column_index].iter().copied().rev().collect();
    let down = column[column_index + 1..].to_vec();

    let mut scores: Vec<u32> = vec![];

    for direction in [left, right, top, down] {
        let mut score = 0;
        for i in direction {
            score += 1;
            if i >= tree_height {
                break;
            }
        }
        scores.push(score)
    }

    scores.iter().fold(1, |total, v| total * v)
}

#[cfg(test)]
mod tests {
    use super::*;

    const example_input: &str = "
30373
25512
65332
33549
35390
    ";

    #[test]
    fn test_part1() {
        let input = example_input
            .trim()
            .split('\n')
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();

        let result = part1(&input);
        assert_eq!(result, 21);
    }

    fn test_part2() {
        let input = example_input
            .trim()
            .split('\n')
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();

        let result = part2(&input);
        assert_eq!(result, 8);
    }
}
