use std::fs;
use std::path::Path;

pub fn run() {
    let path = Path::new("src").join("day1").join("input");
    let contents = fs::read_to_string(path).expect("cant read file");

    let elves: Vec<&str> = contents.trim().split("\n\n").collect();
    let mut summed_calories: Vec<u32> = elves
        .iter()
        .map(|e| {
            e.split('\n')
                .map(|e| e.parse().expect(format!("cannot unwrap {e}").as_str()))
                .collect::<Vec<u32>>()
                .iter()
                .sum()
        })
        .collect();

    // PART 1
    let max = summed_calories.iter().max().unwrap();
    println!("MAX: {max}");

    // PART 2
    summed_calories.sort_by(|a, b| b.cmp(a));
    let top3 = summed_calories[0..3].to_vec().iter().sum::<u32>();
    println!("TOP3: {top3}");
}
