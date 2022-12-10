use std::fs;
use std::io;
use std::path::Path;

pub fn read_file(day: &str) -> io::Result<String> {
    let path = Path::new("inputs").join(format!("day{day}.txt"));
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

pub fn read_and_parse_file(day: &str) -> io::Result<Vec<String>> {
    let path = Path::new("inputs").join(format!("day{day}.txt"));
    let contents = fs::read_to_string(path)?;
    let splitted: Vec<String> = contents
        .as_str()
        .trim()
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    Ok(splitted)
}

pub fn print_type_of<T>(_: &T) {
    println!(" type: {}", std::any::type_name::<T>());
}
