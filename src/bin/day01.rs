use adventofcode::read_file;
use std::io;

pub fn main() -> io::Result<()> {
    let contents = read_file("01")?;

    let elves: Vec<&str> = contents.trim().split("\n\n").collect();
    let mut summed_calories: Vec<u32> = elves
        .iter()
        .map(|e| {
            e.split('\n')
                .filter_map(|e| e.parse().ok())
                .collect::<Vec<u32>>()
                .iter()
                .sum()
        })
        .collect();

    // PART 1
    let max = summed_calories.iter().max().expect("cannot compute max");
    println!("MAX:  {max}");

    // PART 2
    summed_calories.sort_by(|a, b| b.cmp(a));
    let top3 = summed_calories[0..3].to_vec().iter().sum::<u32>();
    println!("TOP3: {top3}");

    Ok(())
}
