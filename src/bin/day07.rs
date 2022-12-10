use adventofcode::read_file;
use std::collections::HashMap;

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

#[derive(Debug)]
struct Dir {
    name: String,
    path: Vec<String>,
    subdirs: Vec<String>,
    files: Vec<File>,
}

impl Dir {
    fn new(name: String, path: Vec<String>) -> Self {
        Dir {
            name,
            path,
            subdirs: vec![],
            files: vec![],
        }
    }
}

const PART1_MAX_SIZE: u32 = 100000;
const TOTAL_SYSTEM_SIZE: u32 = 70000000;
const NEEDED_SIZE: u32 = 30000000;

fn main() {
    let file = read_file("07").expect("cant read file");
    let filesystem = parse_input(&file);
    println!("{filesystem:#?}");

    let (part1_result, part2_result) = run(&filesystem);
    println!("Part1: {part1_result}");
    println!("Part2: {part2_result}");
}

fn format_dir_name(path: &Vec<String>) -> String {
    format!("/{}", path[1..].join("/"))
}

fn parse_input(input: &str) -> HashMap<String, Dir> {
    let mut input = input.trim().split('\n').map(|e| e.trim()).peekable();

    let mut cwd: Vec<String> = vec![];
    let mut filesystem: HashMap<String, Dir> = HashMap::new();

    while let Some(line) = input.next() {
        if line.starts_with('$') {
            let command = line[2..].to_string();

            if command.starts_with("cd") {
                let to_dir = command[3..].to_string();
                if to_dir == ".." {
                    cwd.pop();
                } else {
                    cwd.push(to_dir);
                }
            } else if command.starts_with("ls") {
                let mut current_dir = Dir::new(cwd.last().unwrap().to_string(), cwd.clone());
                loop {
                    match input.peek() {
                        Some(next_line) => {
                            if next_line.starts_with('$') {
                                break;
                            }
                        }
                        None => break,
                    }

                    let next_line = input.next().unwrap();

                    let file_data = next_line.split(' ').collect::<Vec<&str>>();

                    if file_data[0] == "dir" {
                        current_dir.subdirs.push(file_data[1].to_string());
                    } else if let Ok(size) = file_data[0].parse::<u32>() {
                        current_dir.files.push(File {
                            name: file_data[1].to_string(),
                            size,
                        });
                    }
                }
                filesystem.insert(format_dir_name(&cwd), current_dir);
            }
        }
    }

    filesystem
}

fn run(filesystem: &HashMap<String, Dir>) -> (u32, u32) {
    let mut sizes: HashMap<String, u32> = HashMap::new();

    for dir in filesystem.values() {
        let dir_size: u32 = compute_dir_size(dir, filesystem);
        sizes.insert(format_dir_name(&dir.path), dir_size);
    }

    println!("Part1: sizes: {sizes:#?}");
    let result_part1 = sizes.values().fold(0, |sum, size| {
        if size < &PART1_MAX_SIZE {
            sum + size
        } else {
            sum
        }
    });

    let to_free = NEEDED_SIZE - (TOTAL_SYSTEM_SIZE - sizes.get("/").unwrap());

    println!("to_free: {to_free}");

    let mut sorted_sizes: Vec<(&String, &u32)> = sizes.iter().collect();
    sorted_sizes.sort_by(|a, b| a.1.cmp(b.1));
    println!("{sorted_sizes:#?}");
    for size in sorted_sizes {
        if *size.1 >= to_free {
            return (result_part1, *size.1);
        }
    }
    return (result_part1, 0);
}

fn compute_dir_size(dir: &Dir, filesystem: &HashMap<String, Dir>) -> u32 {
    let mut total_size = dir.files.iter().fold(0, |sum, file| sum + file.size);

    let path = dir.path.clone();
    for subdir in dir.subdirs.clone() {
        let subdir_name = format_dir_name(&[path.clone(), vec![subdir.clone()]].concat());
        let subdir_size = compute_dir_size(filesystem.get(&subdir_name).unwrap(), filesystem);

        total_size += subdir_size;
    }

    return total_size;
}

#[cfg(test)]
mod tests {
    use super::*;

    const example_input: &str = "
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
    ";

    #[test]
    fn test_part1() {
        let filesystem = parse_input(example_input);
        let (result, _) = run(&filesystem);
        println!("Part1: {result}");
        assert_eq!(result, 95437);
    }

    #[test]
    fn test_part2() {
        let filesystem = parse_input(example_input);
        let (_, result) = run(&filesystem);
        println!("Part2: {result}");
        assert_eq!(result, 24933642);
    }
}
