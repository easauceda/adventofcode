use itertools::Itertools;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn solve() -> (i32, i32){
    let input = get_input();

    let phrases : Vec<Vec<_>> = input.lines()
		.filter_map(|line| line.ok())
		.map(|line| line.split_whitespace().map(|w| w.to_string()).collect())
        .collect();

    let valid_count = phrases.iter()
        .filter(|phrase| phrase.iter().unique().count() == phrase.len())
        .count() as i32;

    let strict_valid_count = phrases.iter()
        .filter(|phrase| {
            let words : Vec<_> = phrase.iter().map(|w| w.chars().sorted()).collect();
            words.iter().unique().count() == words.len()
        })
        .count() as i32;

    return (valid_count, strict_valid_count);
}

fn get_input() -> BufReader<File> {
    let path = format!("{}/inputs/{}.txt", env!("CARGO_MANIFEST_DIR"), 4);
    let f = File::open(path).expect("Failed to open file");

    return BufReader::new(f);
}