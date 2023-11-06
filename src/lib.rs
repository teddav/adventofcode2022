use std::fs;
use std::io;
use std::path::Path;

pub fn read_file(day: &str) -> io::Result<String> {
    let path = Path::new("inputs").join(format!("day{day}.txt"));
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}
